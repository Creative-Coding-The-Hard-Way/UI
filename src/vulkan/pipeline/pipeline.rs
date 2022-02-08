use std::sync::Arc;

use ash::vk;

use crate::vulkan::{
    errors::VulkanDebugError, pipeline::PipelineError, PipelineLayout,
    RenderDevice, RenderPass, VulkanDebug,
};

/// An owned Pipeline which is destroyed automatically when it's dropped.
pub struct Pipeline {
    /// The render pass used when creating this pipeline.
    pub render_pass: Arc<RenderPass>,

    /// The pipeline layout used to create this pipeline.
    pub pipeline_layout: Arc<PipelineLayout>,

    /// The raw Vulkan pipeline handle.
    pub raw: vk::Pipeline,

    /// The Vulkan binding point this pipeline uses.
    pub bind_point: vk::PipelineBindPoint,

    /// The Vulkan pipeline layout.
    pub vk_dev: Arc<RenderDevice>,
}

impl Pipeline {
    /// Create a new graphics pipeline.
    pub fn new_graphics_pipeline(
        create_info: vk::GraphicsPipelineCreateInfo,
        render_pass: Arc<RenderPass>,
        pipeline_layout: Arc<PipelineLayout>,
        vk_dev: Arc<RenderDevice>,
    ) -> Result<Pipeline, PipelineError> {
        let raw = unsafe {
            vk_dev
                .logical_device
                .create_graphics_pipelines(
                    vk::PipelineCache::null(),
                    &[create_info],
                    None,
                )
                .map_err(|(_, err)| {
                    PipelineError::UnableToCreateGraphicsPipeline(err)
                })?[0]
        };
        Ok(Self {
            render_pass,
            pipeline_layout,
            raw,
            bind_point: vk::PipelineBindPoint::GRAPHICS,
            vk_dev,
        })
    }
}

impl VulkanDebug for Pipeline {
    fn set_debug_name(
        &self,
        debug_name: impl Into<String>,
    ) -> Result<(), VulkanDebugError> {
        self.vk_dev.name_vulkan_object(
            debug_name,
            vk::ObjectType::PIPELINE,
            self.raw,
        )?;
        Ok(())
    }
}

impl Drop for Pipeline {
    /// # DANGER
    ///
    /// There is no internal synchronization for this type. Unexpected behavior
    /// can occur if this instance is still in-use by the GPU when it is
    /// dropped.
    fn drop(&mut self) {
        unsafe {
            self.vk_dev.logical_device.destroy_pipeline(self.raw, None);
        }
    }
}
