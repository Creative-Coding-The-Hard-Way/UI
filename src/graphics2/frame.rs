use ::{anyhow::Result, ash::vk, std::sync::Arc};

use crate::{
    asset_loader::CombinedImageSampler,
    graphics2::{Vec2, Vec4, Vertex},
    vulkan::{
        errors::VulkanError, Buffer, CommandBuffer, DescriptorPool,
        DescriptorSet, DescriptorSetLayout, GpuVec, MemoryAllocator,
        PipelineLayout, RenderDevice,
    },
};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct UniformBufferData {
    pub view_projection: [[f32; 4]; 4],
}

/// Per-Frame resources for Graphics2.
pub struct Frame {
    _descriptor_pool: DescriptorPool,
    descriptor_set: DescriptorSet,
    uniform_data: Buffer,
    vertex_data: GpuVec<Vertex>,
    vertex_data_needs_rebound: bool,
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

    pub fn draw_quad(
        &mut self,
        center: Vec2,
        dimensions: Vec2,
        texture_index: i32,
    ) -> Result<()> {
        type Vec2 = nalgebra::Vector2<f32>;

        let half_size = 0.5 * dimensions;
        let (left, right) = (-half_size.x, half_size.x);
        let (bottom, top) = (-half_size.y, half_size.y);
        let (uv_left, uv_right) = (0.0, 1.0);
        let (uv_bottom, uv_top) = (0.0, 1.0);
        self.push_vertex(Vertex::new_2d(
            center + Vec2::new(left, bottom),
            Vec4::new(1.0, 1.0, 1.0, 1.0),
            Vec2::new(uv_left, uv_bottom),
            texture_index,
        ))?;
        self.push_vertex(Vertex::new_2d(
            center + Vec2::new(left, top),
            Vec4::new(1.0, 1.0, 1.0, 1.0),
            Vec2::new(uv_left, uv_top),
            texture_index,
        ))?;
        self.push_vertex(Vertex::new_2d(
            center + Vec2::new(right, top),
            Vec4::new(1.0, 1.0, 1.0, 1.0),
            Vec2::new(uv_right, uv_top),
            texture_index,
        ))?;

        self.push_vertex(Vertex::new_2d(
            center + Vec2::new(right, top),
            Vec4::new(1.0, 1.0, 1.0, 1.0),
            Vec2::new(uv_right, uv_top),
            texture_index,
        ))?;
        self.push_vertex(Vertex::new_2d(
            center + Vec2::new(right, bottom),
            Vec4::new(1.0, 1.0, 1.0, 1.0),
            Vec2::new(uv_right, uv_bottom),
            texture_index,
        ))?;
        self.push_vertex(Vertex::new_2d(
            center + Vec2::new(left, bottom),
            Vec4::new(1.0, 1.0, 1.0, 1.0),
            Vec2::new(uv_left, uv_bottom),
            texture_index,
        ))?;
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
        self.vk_dev.logical_device.cmd_draw(
            cmd.raw,
            self.vertex_data.len() as u32,
            1, // index count
            0, // first vertex
            0, // first index
        );
    }

    pub(super) fn clear(&mut self) {
        self.vertex_data.clear();
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
