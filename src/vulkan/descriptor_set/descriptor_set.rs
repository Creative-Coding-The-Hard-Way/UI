use ::{ash::vk, std::sync::Arc};

use crate::vulkan::{
    errors::VulkanDebugError, ImageView, RenderDevice, Sampler, VulkanDebug,
};

/// A Vulkan descriptor set wrapper.
pub struct DescriptorSet {
    /// the raw Vulkan descriptor set handle.
    pub raw: vk::DescriptorSet,

    /// The device used to create the descriptor set.
    pub vk_dev: Arc<RenderDevice>,
}

impl DescriptorSet {
    /// Write a buffer binding to this descripor set.
    ///
    /// # Unsafe
    ///
    /// - because the application must ensure the descriptor set is not in-use
    ///   when it modified by this function.
    pub unsafe fn bind_buffer(
        &self,
        binding: u32,
        buffer: &vk::Buffer,
        descriptor_type: vk::DescriptorType,
    ) {
        let descriptor_buffer_info = vk::DescriptorBufferInfo {
            buffer: *buffer,
            offset: 0,
            range: vk::WHOLE_SIZE,
        };
        let write = vk::WriteDescriptorSet {
            dst_set: self.raw,
            dst_binding: binding,
            dst_array_element: 0,
            descriptor_count: 1,
            descriptor_type,
            p_image_info: std::ptr::null(),
            p_texel_buffer_view: std::ptr::null(),
            p_buffer_info: &descriptor_buffer_info,
            ..Default::default()
        };
        self.vk_dev
            .logical_device
            .update_descriptor_sets(&[write], &[]);
    }

    pub unsafe fn bind_combined_image_sampler(
        &self,
        binding: u32,
        array_element: u32,
        image_view: &ImageView,
        sampler: &Sampler,
    ) {
        let descriptor_image_info = vk::DescriptorImageInfo {
            sampler: sampler.raw,
            image_view: image_view.raw,
            image_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
        };
        let write = vk::WriteDescriptorSet {
            dst_set: self.raw,
            dst_binding: binding,
            dst_array_element: array_element,
            descriptor_count: 1,
            descriptor_type: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
            p_image_info: &descriptor_image_info,
            ..Default::default()
        };
        self.vk_dev
            .logical_device
            .update_descriptor_sets(&[write], &[]);
    }
}

impl VulkanDebug for DescriptorSet {
    fn set_debug_name(
        &self,
        debug_name: impl Into<String>,
    ) -> Result<(), VulkanDebugError> {
        self.vk_dev.name_vulkan_object(
            debug_name,
            vk::ObjectType::DESCRIPTOR_SET,
            self.raw,
        )?;
        Ok(())
    }
}
