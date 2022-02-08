mod msaa_render_target;

use ::{anyhow::Result, ash::vk, std::sync::Arc};

use crate::vulkan::{
    errors::{FramebufferError, VulkanDebugError, VulkanError},
    CommandBuffer, Framebuffer, ImageView, MemoryAllocator, RenderDevice,
    RenderPass, VulkanDebug,
};

/// All resources required for an on-screen renderpass which uses a multisampled
/// color buffer.
pub struct MultisampleRenderpass {
    /// A managed vulkan render pass instance.
    pub render_pass: Arc<RenderPass>,

    /// The multisampled color target.
    /// This is used as an additional color attachment on the render pass and
    /// associated framebuffers. Values are resolved at the end of the render
    /// pass into the output target specified by the framebuffer.
    pub msaa_render_target: Arc<ImageView>,

    /// The vulkan device handle.
    pub vk_dev: Arc<RenderDevice>,
}

impl MultisampleRenderpass {
    /// Create a new multisampled renderpass based on the swapchain's current
    /// extent and format.
    pub fn for_current_swapchain(
        vk_dev: Arc<RenderDevice>,
        vk_alloc: Arc<dyn MemoryAllocator>,
    ) -> Result<Self, VulkanError> {
        let msaa_render_target =
            MultisampleRenderpass::create_msaa_render_target(
                vk_dev.clone(),
                vk_alloc.clone(),
            )?;
        let render_pass = MultisampleRenderpass::create_render_pass(
            &msaa_render_target,
            vk_dev.clone(),
        )?;
        Ok(Self {
            render_pass,
            msaa_render_target,
            vk_dev,
        })
    }

    /// Create framebuffers for presenting to the correspondingly-indexed
    /// swapchain images.
    pub fn create_swapchain_framebuffers(
        &self,
    ) -> Result<Vec<Framebuffer>, FramebufferError> {
        let name = "MSAARenderPass Framebuffer";
        self.vk_dev.with_swapchain(
            |swapchain| -> Result<Vec<Framebuffer>, FramebufferError> {
                let mut framebuffers = vec![];
                for i in 0..swapchain.image_views.len() {
                    let views = vec![
                        self.msaa_render_target.raw,
                        swapchain.image_views[i],
                    ];
                    let framebuffer = Framebuffer::with_color_attachments(
                        self.vk_dev.clone(),
                        &self.render_pass,
                        &views,
                        swapchain.extent,
                    )?;
                    framebuffer.set_debug_name(format!("{} - {}", name, i))?;
                    framebuffers.push(framebuffer);
                }
                Ok(framebuffers)
            },
        )
    }

    /// Begin the render pass for the current frame
    pub unsafe fn begin_renderpass_inline(
        &self,
        command_buffer: &CommandBuffer,
        framebuffer: &Framebuffer,
        rgba_clear_color: [f32; 4],
    ) {
        let clear_value = vk::ClearValue {
            color: vk::ClearColorValue {
                float32: rgba_clear_color,
            },
        };
        let render_pass_begin_info = vk::RenderPassBeginInfo {
            render_pass: framebuffer.render_pass.raw,
            framebuffer: framebuffer.raw,
            render_area: vk::Rect2D {
                offset: vk::Offset2D { x: 0, y: 0 },
                extent: framebuffer.extent,
            },
            clear_value_count: 1,
            p_clear_values: &clear_value,
            ..Default::default()
        };
        self.vk_dev.logical_device.cmd_begin_render_pass(
            command_buffer.raw,
            &render_pass_begin_info,
            vk::SubpassContents::INLINE,
        );
    }

    pub unsafe fn end_renderpass(&self, command_buffer: &CommandBuffer) {
        self.vk_dev
            .logical_device
            .cmd_end_render_pass(command_buffer.raw);
    }

    /// The number of samples used by the render target
    pub fn samples(&self) -> vk::SampleCountFlags {
        self.msaa_render_target.image.create_info.samples
    }
}

impl VulkanDebug for MultisampleRenderpass {
    fn set_debug_name(
        &self,
        debug_name: impl Into<String>,
    ) -> Result<(), VulkanDebugError> {
        let name = debug_name.into();
        self.render_pass
            .set_debug_name(format!("MSAARenderPass - {}", name))?;
        Ok(())
    }
}
