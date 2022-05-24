use ::{anyhow::Result, ash::vk, std::sync::Arc};

use crate::{
    asset_loader::CombinedImageSampler,
    immediate_mode_graphics::{Vertex, VertexStream},
    vulkan::{
        errors::VulkanError, Buffer, CommandBuffer, DescriptorPool,
        DescriptorSet, DescriptorSetLayout, GpuVec, MemoryAllocator,
        PipelineLayout, RenderDevice,
    },
};

/// All data sent to the shaders in a Uniform Buffer.
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UniformBufferData {
    /// The view-projection matrix created with: projection * view.
    pub view_projection: [[f32; 4]; 4],
}

/// All resources required to render a single frame's vertices.
pub struct Frame {
    /// The descriptor pool owns gpu resources used by the descriptor set.
    _descriptor_pool: DescriptorPool,

    /// The descriptor set enables texture indexing.
    descriptor_set: DescriptorSet,

    /// This frame's uniform data.
    uniform_data: Buffer,

    /// All of the vertices to be rendered on the current frame.
    /// This is cleared each time the frame is acquired.
    vertex_data: GpuVec<Vertex>,

    /// Flag is set to 'true' if the vertex buffer needs to be rebound to the
    /// descriptor set. This occurs when the GpuVec grows and needs to be
    /// re-allocated.
    vertex_data_needs_rebound: bool,

    /// The set of all vertex indices.
    index_data: GpuVec<u32>,

    /// The Vulkan render device.
    vk_dev: Arc<RenderDevice>,
}

impl Frame {
    /// Allocate resources and buffers for a single frame.
    pub fn new(
        vk_dev: Arc<RenderDevice>,
        vk_alloc: Arc<dyn MemoryAllocator>,
        textures: &[CombinedImageSampler],
        descriptor_layout: &DescriptorSetLayout,
    ) -> Result<Self, VulkanError> {
        let descriptor_pool = DescriptorPool::new(
            vk_dev.clone(),
            1,
            &[
                vk::DescriptorPoolSize {
                    ty: vk::DescriptorType::STORAGE_BUFFER,
                    descriptor_count: 1,
                },
                vk::DescriptorPoolSize {
                    ty: vk::DescriptorType::UNIFORM_BUFFER,
                    descriptor_count: 1,
                },
                vk::DescriptorPoolSize {
                    ty: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
                    descriptor_count: textures.len() as u32,
                },
            ],
        )?;
        let descriptor_set = descriptor_pool
            .allocate_with_variable_counts(
                descriptor_layout,
                1,
                textures.len() as u32,
            )?
            .pop()
            .unwrap();

        let vertex_data = GpuVec::new(
            vk_dev.clone(),
            vk_alloc.clone(),
            vk::BufferUsageFlags::STORAGE_BUFFER,
            1, // initial buffer capacity
        )?;
        let index_data = GpuVec::new(
            vk_dev.clone(),
            vk_alloc.clone(),
            vk::BufferUsageFlags::INDEX_BUFFER,
            500,
        )?;
        let mut uniform_data = Buffer::new(
            vk_dev.clone(),
            vk_alloc.clone(),
            vk::BufferUsageFlags::UNIFORM_BUFFER,
            vk::MemoryPropertyFlags::HOST_VISIBLE
                | vk::MemoryPropertyFlags::HOST_COHERENT,
            std::mem::size_of::<UniformBufferData>() as u64,
        )?;
        uniform_data.map()?;

        unsafe {
            descriptor_set.bind_buffer(
                1,
                &uniform_data.raw,
                vk::DescriptorType::UNIFORM_BUFFER,
            );
            for (texture_index, texture) in textures.iter().enumerate() {
                descriptor_set.bind_combined_image_sampler(
                    2,
                    texture_index as u32,
                    &texture.image_view,
                    &texture.sampler,
                );
            }
        }

        Ok(Self {
            vertex_data,
            vertex_data_needs_rebound: true,
            index_data,
            uniform_data,
            _descriptor_pool: descriptor_pool,
            descriptor_set,
            vk_dev,
        })
    }

    /// Set the view projection used to render geometry for the current frame.
    pub fn set_view_projection(
        &mut self,
        view_projection: nalgebra::Matrix4<f32>,
    ) -> Result<()> {
        self.uniform_data.data_mut::<UniformBufferData>()?[0] =
            UniformBufferData {
                view_projection: view_projection.into(),
            };
        Ok(())
    }
}

impl VertexStream for Frame {
    /// Push vertices into the frame. Indices index into the given vertex slice.
    fn push_vertices(
        &mut self,
        vertices: &[Vertex],
        indices: &[u32],
    ) -> Result<()> {
        let base_index = self.vertex_data.len() as u32;
        for vertex in vertices {
            self.push_vertex(*vertex)?;
        }
        for index in indices {
            self.index_data.push_back(base_index + index)?;
        }
        Ok(())
    }
}

impl Frame {
    /// Write this frame's draw commands into a given command buffer.
    ///
    /// # UNSAFE BECAUSE
    ///
    /// - This command assumes that the required pipeline has already been
    ///   bound.
    /// - This command is not internally synchronized, it is up to the caller
    ///   to ensure that the frame's resources are not currently in use by the
    ///   gpu.
    pub(super) unsafe fn write_frame_commands(
        &mut self,
        cmd: &CommandBuffer,
        pipeline_layout: &PipelineLayout,
    ) {
        if self.vertex_data_needs_rebound {
            self.rebind_vertex_data();
            self.vertex_data_needs_rebound = false;
        }

        self.vk_dev.logical_device.cmd_bind_descriptor_sets(
            cmd.raw,
            vk::PipelineBindPoint::GRAPHICS,
            pipeline_layout.raw,
            0,
            &[self.descriptor_set.raw],
            &[],
        );
        self.vk_dev.logical_device.cmd_bind_index_buffer(
            cmd.raw,
            self.index_data.buffer.raw,
            0,
            vk::IndexType::UINT32,
        );
        self.vk_dev.logical_device.cmd_draw_indexed(
            cmd.raw,
            self.index_data.len() as u32,
            1,
            0,
            0,
            0,
        );
    }

    pub(super) fn clear(&mut self) {
        self.vertex_data.clear();
        self.index_data.clear();
    }

    /// Add a vertex to the vertex buffer.
    /// Automatically updates the needs rebound flag.
    fn push_vertex(&mut self, vertex: Vertex) -> Result<()> {
        self.vertex_data_needs_rebound |= self.vertex_data.push_back(vertex)?;
        Ok(())
    }

    /// Rebind the vertex buffer descriptor. This is needed because the
    /// underlying vulkan buffer can change when the GpuVec resizes.
    ///
    /// # UNSAFE BECAUSE
    ///
    /// - The caller must ensure the descriptor set has not been bound to a
    ///   command buffer yet.
    unsafe fn rebind_vertex_data(&mut self) {
        self.descriptor_set.bind_buffer(
            0,
            &self.vertex_data.buffer.raw,
            vk::DescriptorType::STORAGE_BUFFER,
        );
    }
}
