use super::{Image, ImageError};

use crate::vulkan::{
    errors::VulkanDebugError, MemoryAllocator, RenderDevice, VulkanDebug,
};

use ::{
    ash::{version::DeviceV1_0, vk},
    std::sync::Arc,
};

impl Image {
    pub fn new(
        vk_dev: Arc<RenderDevice>,
        vk_alloc: Arc<dyn MemoryAllocator>,
        create_info: &vk::ImageCreateInfo,
        memory_property_flags: vk::MemoryPropertyFlags,
    ) -> Result<Self, ImageError> {
        let raw = unsafe {
            vk_dev
                .logical_device
                .create_image(create_info, None)
                .map_err(ImageError::UnableToCreateImage)?
        };
        let memory_requirements =
            unsafe { vk_dev.logical_device.get_image_memory_requirements(raw) };

        let allocation = unsafe {
            vk_alloc
                .allocate_memory(memory_requirements, memory_property_flags)?
        };

        unsafe {
            vk_dev
                .logical_device
                .bind_image_memory(raw, allocation.memory, allocation.offset)
                .map_err(ImageError::UnableToBindImageMemory)?;
        }

        Ok(Self {
            raw,
            allocation,
            vk_alloc,
            vk_dev,
        })
    }
}

impl Drop for Image {
    /// # DANGER
    ///
    /// There is no internal synchronization for this type. Unexpected behavior
    /// can occur if this instance is still in-use by the GPU when it is
    /// dropped.
    fn drop(&mut self) {
        unsafe {
            self.vk_dev.logical_device.destroy_image(self.raw, None);
            self.vk_alloc
                .free(&self.allocation)
                .expect("unable to free the image's memory");
        }
    }
}

impl VulkanDebug for Image {
    fn set_debug_name(
        &self,
        debug_name: impl Into<String>,
    ) -> Result<(), VulkanDebugError> {
        self.vk_dev.name_vulkan_object(
            debug_name,
            vk::ObjectType::IMAGE,
            self.raw,
        )?;
        Ok(())
    }
}
