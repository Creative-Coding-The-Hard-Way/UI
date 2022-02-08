use ::{ash::vk, std::sync::Arc};

use crate::{
    multisample_renderpass::MultisampleRenderpass,
    vulkan::{
        errors::VulkanError, Image, ImageView, MemoryAllocator, RenderDevice,
        RenderPass,
    },
};

impl MultisampleRenderpass {
    /// Create a multisample render target based on the swapchain's current
    /// extent and parameters.
    pub(super) fn create_msaa_render_target(
        vk_dev: Arc<RenderDevice>,
        vk_alloc: Arc<dyn MemoryAllocator>,
    ) -> Result<Arc<ImageView>, VulkanError> {
        let samples = vk_dev.get_supported_msaa(vk::SampleCountFlags::TYPE_4);
        let (swap_extent, format) =
            vk_dev.with_swapchain(|swap| (swap.extent, swap.format));
        let create_info = vk::ImageCreateInfo {
            flags: vk::ImageCreateFlags::empty(),
            image_type: vk::ImageType::TYPE_2D,
            extent: vk::Extent3D {
                width: swap_extent.width,
                height: swap_extent.height,
                depth: 1,
            },
            mip_levels: 1,
            array_layers: 1,
            format,
            samples,
            tiling: vk::ImageTiling::OPTIMAL,
            initial_layout: vk::ImageLayout::UNDEFINED,
            usage: vk::ImageUsageFlags::COLOR_ATTACHMENT
                | vk::ImageUsageFlags::TRANSIENT_ATTACHMENT,
            sharing_mode: vk::SharingMode::EXCLUSIVE,
            ..Default::default()
        };
        let msaa_render_target = Arc::new(Image::new(
            vk_dev.clone(),
            vk_alloc,
            &create_info,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
        )?);
        let view = Arc::new(ImageView::new_2d(
            msaa_render_target,
            format,
            vk::ImageAspectFlags::COLOR,
        )?);
        Ok(view)
    }

    /// Create a render pass with two color attachments, the first is the MSAA
    /// render target and the second is a target single-sampled image to be
    /// specified by the framebuffer.
    pub(super) fn create_render_pass(
        msaa_render_target: &ImageView,
        vk_dev: Arc<RenderDevice>,
    ) -> Result<Arc<RenderPass>, VulkanError> {
        let format = msaa_render_target.image.create_info.format;
        let color_attachment = vk::AttachmentDescription {
            flags: vk::AttachmentDescriptionFlags::empty(),
            format,
            samples: msaa_render_target.image.create_info.samples,
            load_op: vk::AttachmentLoadOp::CLEAR,
            store_op: vk::AttachmentStoreOp::STORE,
            stencil_load_op: vk::AttachmentLoadOp::DONT_CARE,
            stencil_store_op: vk::AttachmentStoreOp::DONT_CARE,
            initial_layout: vk::ImageLayout::UNDEFINED,
            final_layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        };
        let color_attachment_reference = vk::AttachmentReference {
            attachment: 0,
            layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        };

        let color_resolve_attachment = vk::AttachmentDescription {
            flags: vk::AttachmentDescriptionFlags::empty(),
            format,
            samples: vk::SampleCountFlags::TYPE_1,
            load_op: vk::AttachmentLoadOp::DONT_CARE,
            store_op: vk::AttachmentStoreOp::STORE,
            stencil_load_op: vk::AttachmentLoadOp::DONT_CARE,
            stencil_store_op: vk::AttachmentStoreOp::DONT_CARE,
            initial_layout: vk::ImageLayout::UNDEFINED,
            final_layout: vk::ImageLayout::PRESENT_SRC_KHR,
        };
        let resolve_attachment_reference = vk::AttachmentReference {
            attachment: 1,
            layout: vk::ImageLayout::COLOR_ATTACHMENT_OPTIMAL,
        };

        let dependencies = [vk::SubpassDependency {
            src_subpass: vk::SUBPASS_EXTERNAL,
            dst_subpass: 0,
            src_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            dst_stage_mask: vk::PipelineStageFlags::COLOR_ATTACHMENT_OUTPUT,
            src_access_mask: vk::AccessFlags::empty(),
            dst_access_mask: vk::AccessFlags::COLOR_ATTACHMENT_READ
                | vk::AccessFlags::COLOR_ATTACHMENT_WRITE,
            dependency_flags: vk::DependencyFlags::empty(),
        }];
        let subpass = vk::SubpassDescription {
            flags: vk::SubpassDescriptionFlags::empty(),
            pipeline_bind_point: vk::PipelineBindPoint::GRAPHICS,
            input_attachment_count: 0,
            p_input_attachments: std::ptr::null(),
            color_attachment_count: 1,
            p_color_attachments: &color_attachment_reference,
            p_depth_stencil_attachment: std::ptr::null(),
            preserve_attachment_count: 0,
            p_preserve_attachments: std::ptr::null(),
            p_resolve_attachments: &resolve_attachment_reference,
        };
        let attachments = vec![color_attachment, color_resolve_attachment];
        let render_pass_info = vk::RenderPassCreateInfo {
            flags: vk::RenderPassCreateFlags::empty(),
            attachment_count: attachments.len() as u32,
            p_attachments: attachments.as_ptr(),
            subpass_count: 1,
            p_subpasses: &subpass,
            dependency_count: dependencies.len() as u32,
            p_dependencies: dependencies.as_ptr(),
            ..Default::default()
        };

        Ok(Arc::new(RenderPass::new(vk_dev, &render_pass_info)?))
    }
}
