use ::{ash::vk, std::sync::Arc};

use crate::{
    multisample_renderpass::MultisampleRenderpass,
    vulkan::{
        errors::VulkanError, Image, ImageView, MemoryAllocator, RenderDevice,
    },
};

impl MultisampleRenderpass {
    /// Create a multisample render target based on the swapchain's current
    /// extent and parameters.
    pub(super) fn create_msaa_render_target(
        vk_dev: Arc<RenderDevice>,
        vk_alloc: Arc<dyn MemoryAllocator>,
    ) -> Result<Arc<ImageView>, VulkanError> {
        let samples = Self::pick_max_supported_msaa_count(
            &vk_dev,
            vk::SampleCountFlags::TYPE_4,
        );
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

    /// Query the device for MSAA support.
    ///
    /// # Returns
    ///
    /// The minimum between the `desired` sample count and the sample count
    /// supported by the device.
    ///
    /// e.g. if the device supports 4xMSAA and 8xMSAA is desired, this method
    /// will return 4xMSAA. Similarly, if the device supports 4xMSAA and 2xMSAA
    /// is desired, then this method will return 2xMSAA.
    fn pick_max_supported_msaa_count(
        vk_dev: &RenderDevice,
        desired: vk::SampleCountFlags,
    ) -> vk::SampleCountFlags {
        let props = unsafe {
            vk_dev
                .instance
                .ash
                .get_physical_device_properties(vk_dev.physical_device)
        };
        let color_samples = props.limits.framebuffer_color_sample_counts;
        let depth_samples = props.limits.framebuffer_depth_sample_counts;
        let supported_samples = color_samples.min(depth_samples);

        if supported_samples.contains(vk::SampleCountFlags::TYPE_64) {
            desired.min(vk::SampleCountFlags::TYPE_64)
        } else if supported_samples.contains(vk::SampleCountFlags::TYPE_32) {
            desired.min(vk::SampleCountFlags::TYPE_32)
        } else if supported_samples.contains(vk::SampleCountFlags::TYPE_16) {
            desired.min(vk::SampleCountFlags::TYPE_16)
        } else if supported_samples.contains(vk::SampleCountFlags::TYPE_8) {
            desired.min(vk::SampleCountFlags::TYPE_8)
        } else if supported_samples.contains(vk::SampleCountFlags::TYPE_4) {
            desired.min(vk::SampleCountFlags::TYPE_4)
        } else if supported_samples.contains(vk::SampleCountFlags::TYPE_2) {
            desired.min(vk::SampleCountFlags::TYPE_2)
        } else {
            vk::SampleCountFlags::TYPE_1
        }
    }
}
