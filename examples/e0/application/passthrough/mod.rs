mod pipeline;

use std::sync::Arc;

use anyhow::Result;
use ash::vk;
use ccthw::{
    multisample_renderpass::MultisampleRenderpass,
    vulkan::{
        errors::{InstanceError, VulkanError},
        CommandBuffer, DescriptorPool, DescriptorSet, GpuVec, MemoryAllocator,
        Pipeline, RenderDevice,
    },
};

#[derive(Debug, Copy, Clone)]
pub struct Vertex2D {
    pub pos: [f32; 2],
    pub rgba: [f32; 4],
}

pub struct Passthrough {
    pub pipeline: Pipeline,
    pub descriptor_pool: DescriptorPool,
    pub descriptor_set: DescriptorSet,
    pub vertex_data: GpuVec<Vertex2D>,
    pub vk_dev: Arc<RenderDevice>,
}

impl Passthrough {
    pub fn new(
        msaa_renderpass: &MultisampleRenderpass,
        vk_alloc: Arc<dyn MemoryAllocator>,
        vk_dev: Arc<RenderDevice>,
    ) -> Result<Self, VulkanError> {
        let pipeline =
            pipeline::create_pipeline(msaa_renderpass, vk_dev.clone())?;
        let descriptor_pool = DescriptorPool::new(
            vk_dev.clone(),
            1,
            &[vk::DescriptorPoolSize {
                ty: vk::DescriptorType::STORAGE_BUFFER,
                descriptor_count: 1,
            }],
        )?;
        let descriptor_set = descriptor_pool
            .allocate(&pipeline.pipeline_layout.descriptor_layouts[0], 1)?
            .pop()
            .unwrap();

        let vertex_data = GpuVec::new(
            vk_dev.clone(),
            vk_alloc.clone(),
            vk::BufferUsageFlags::STORAGE_BUFFER,
            3,
        )?;

        unsafe {
            descriptor_set.bind_buffer(
                0,
                &vertex_data.buffer.raw,
                vk::DescriptorType::STORAGE_BUFFER,
            );
        }

        Ok(Self {
            pipeline,
            descriptor_pool,
            descriptor_set,
            vertex_data,
            vk_dev,
        })
    }

    /// Rebuild only the swapchain-dependent resources for this renderer
    pub fn rebuild_swapchain_resources(
        &mut self,
        msaa_renderpass: &MultisampleRenderpass,
    ) -> Result<(), VulkanError> {
        self.pipeline =
            pipeline::create_pipeline(msaa_renderpass, self.vk_dev.clone())?;
        Ok(())
    }

    /// Write draw commands into the given command buffer.
    ///
    /// UNSAFE BECAUSE:
    ///   - Assumes that the render pass associated with this pipeline has
    ///     already been started in the given command buffer.
    pub unsafe fn write_commands(&self, cmd: &CommandBuffer) -> Result<()> {
        self.vk_dev.logical_device.cmd_bind_pipeline(
            cmd.raw,
            vk::PipelineBindPoint::GRAPHICS,
            self.pipeline.raw,
        );
        self.vk_dev.logical_device.cmd_bind_descriptor_sets(
            cmd.raw,
            vk::PipelineBindPoint::GRAPHICS,
            self.pipeline.pipeline_layout.raw,
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
        Ok(())
    }

    pub fn push_vertices(
        &mut self,
        vertices: &[Vertex2D],
    ) -> Result<(), VulkanError> {
        unsafe {
            // Stall the gpu before writing any data to the buffer.
            // This is painfully slow but is needed because this buffer can be
            // used by multiple in-flight frames. (more granular sync or more
            // buffers could remove the need for this)
            self.vk_dev
                .logical_device
                .device_wait_idle()
                .map_err(InstanceError::UnableToWaitIdle)?;
        }

        let mut needs_rebound = false;
        for vertex in vertices {
            needs_rebound |= self.vertex_data.push_back(*vertex)?;
        }
        if needs_rebound {
            unsafe {
                self.descriptor_set.bind_buffer(
                    0,
                    &self.vertex_data.buffer.raw,
                    vk::DescriptorType::STORAGE_BUFFER,
                );
            }
        }
        Ok(())
    }
}
