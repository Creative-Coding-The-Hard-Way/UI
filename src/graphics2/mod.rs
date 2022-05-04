//! Graphics 2 is a high-level module for efficently rendering immediate-mode
//! graphics which change every frame.

mod drawable;
mod frame;
mod graphics2_error;
mod pipeline;
mod text;
mod vertex;

pub mod primitives;

use ::{anyhow::Result, ash::vk, std::sync::Arc};

pub type Vec2 = nalgebra::Vector2<f32>;
pub type Vec3 = nalgebra::Vector3<f32>;
pub type Vec4 = nalgebra::Vector4<f32>;

pub use self::{
    drawable::Drawable, frame::Frame, graphics2_error::Graphics2Error,
    text::Text, vertex::Vertex,
};
use crate::{
    asset_loader::CombinedImageSampler,
    multisample_renderpass::MultisampleRenderpass,
    vulkan::{
        errors::VulkanError, CommandBuffer, MemoryAllocator, Pipeline,
        RenderDevice,
    },
};

pub struct Graphics2 {
    pub textures: Vec<CombinedImageSampler>,
    pub pipeline: Pipeline,
    pub frames: Vec<Option<Frame>>,
    pub vk_alloc: Arc<dyn MemoryAllocator>,
    pub vk_dev: Arc<RenderDevice>,
}

impl Graphics2 {
    pub fn new(
        msaa_renderpass: &MultisampleRenderpass,
        textures: &[CombinedImageSampler],
        vk_alloc: Arc<dyn MemoryAllocator>,
        vk_dev: Arc<RenderDevice>,
    ) -> Result<Self, VulkanError> {
        let pipeline = pipeline::create_pipeline(
            msaa_renderpass,
            textures.len() as u32,
            vk_dev.clone(),
        )?;
        let frames = {
            let mut frames = vec![];
            for _ in 0..vk_dev.swapchain_image_count() {
                let frame = Frame::new(
                    vk_dev.clone(),
                    vk_alloc.clone(),
                    textures,
                    &pipeline.pipeline_layout.descriptor_layouts[0],
                )?;
                frames.push(Some(frame));
            }
            frames
        };
        Ok(Self {
            textures: textures.to_owned(),
            pipeline,
            frames,
            vk_alloc,
            vk_dev,
        })
    }

    /// Rebuild only the swapchain-dependent resources for this renderer
    pub fn rebuild_swapchain_resources(
        &mut self,
        msaa_renderpass: &MultisampleRenderpass,
    ) -> Result<(), VulkanError> {
        self.pipeline = pipeline::create_pipeline(
            msaa_renderpass,
            self.textures.len() as u32,
            self.vk_dev.clone(),
        )?;
        self.frames = {
            let mut frames = vec![];
            for _ in 0..self.vk_dev.swapchain_image_count() {
                let frame = Frame::new(
                    self.vk_dev.clone(),
                    self.vk_alloc.clone(),
                    &self.textures,
                    &self.pipeline.pipeline_layout.descriptor_layouts[0],
                )?;
                frames.push(Some(frame));
            }
            frames
        };
        Ok(())
    }

    /// Acquire per-frame resources for the currently-targeted swapchain
    /// image.
    pub fn acquire_frame(
        &mut self,
        swapchain_image_index: usize,
    ) -> Result<Frame, Graphics2Error> {
        let mut frame = self.frames[swapchain_image_index].take().ok_or(
            Graphics2Error::FrameResourcesUnavailable(swapchain_image_index),
        )?;
        frame.clear();
        Ok(frame)
    }

    /// Complete the frame by writing it's draw commands into the given
    /// command buffer.
    ///
    /// UNSAFE BECAUSE:
    ///   - Assumes that the render pass associated with this pipeline has
    ///     already been started in the given command buffer.
    pub unsafe fn complete_frame(
        &mut self,
        cmd: &CommandBuffer,
        mut frame: Frame,
        swapchain_image_index: usize,
    ) -> Result<()> {
        self.vk_dev.logical_device.cmd_bind_pipeline(
            cmd.raw,
            vk::PipelineBindPoint::GRAPHICS,
            self.pipeline.raw,
        );
        frame.write_frame_commands(cmd, &self.pipeline.pipeline_layout);
        self.frames[swapchain_image_index] = Some(frame);
        Ok(())
    }
}
