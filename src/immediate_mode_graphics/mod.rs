//! This module defines structs and functions for efficiently rendering
//! Vertices which are generated new on the CPU every frame.
//!

mod drawable;
mod error;
mod frame;
mod pipeline;
mod text;
mod vertex;

pub mod primitives;

use ::{anyhow::Result, ash::vk, std::sync::Arc};

pub use self::{
    drawable::Drawable, error::ImmediateModeGraphicsError, frame::Frame,
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

/// This struct holds all of the resources required to render textured vertices
/// which the CPU changes every frame.
///
pub struct ImmediateModeGraphics {
    /// The set of all indexable textures.
    /// Vertex texture_id's are treated as indexes into this vector.
    textures: Vec<CombinedImageSampler>,

    /// The graphics pipeline used to render vertices.
    pipeline: Pipeline,

    /// All per-frame resources used to render vertices.
    frames: Vec<Option<Frame>>,

    /// The device allocator.
    vk_alloc: Arc<dyn MemoryAllocator>,

    /// The vulkan render device.
    vk_dev: Arc<RenderDevice>,
}

impl ImmediateModeGraphics {
    /// Create a new Immediate Mode Graphics object which targets the provided
    /// renderpass.
    ///
    /// Vertices can reference any texture in the textures array by their index.
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
    ) -> Result<Frame, ImmediateModeGraphicsError> {
        let mut frame = self.frames[swapchain_image_index].take().ok_or(
            ImmediateModeGraphicsError::FrameResourcesUnavailable(
                swapchain_image_index,
            ),
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
