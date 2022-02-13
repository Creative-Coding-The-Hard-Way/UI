use std::ffi::c_void;

use ::{ash::vk, std::sync::Arc};

use crate::vulkan::{
    descriptor_set::DescriptorSetError, errors::VulkanDebugError, RenderDevice,
    VulkanDebug,
};

/// An owned Descriptor Set Layout which is automatically destroyed when
/// dropped.
pub struct DescriptorSetLayout {
    /// The raw vulkan Descriptor Set Layout handle
    pub raw: vk::DescriptorSetLayout,

    /// The device used to create the layout
    pub vk_dev: Arc<RenderDevice>,
}

impl DescriptorSetLayout {
    pub fn new(
        vk_dev: Arc<RenderDevice>,
        bindings: &[vk::DescriptorSetLayoutBinding],
    ) -> Result<Self, DescriptorSetError> {
        let bindings_and_flags: Vec<_> = bindings
            .iter()
            .map(|binding| (*binding, vk::DescriptorBindingFlags::empty()))
            .collect();
        Self::new_with_flags(vk_dev, &bindings_and_flags)
    }

    pub fn new_with_flags(
        vk_dev: Arc<RenderDevice>,
        bindings_and_flags: &[(
            vk::DescriptorSetLayoutBinding,
            vk::DescriptorBindingFlags,
        )],
    ) -> Result<Self, DescriptorSetError> {
        let flags: Vec<vk::DescriptorBindingFlags> = bindings_and_flags
            .iter()
            .map(|(_binding, flag)| *flag)
            .collect();
        let bindings: Vec<vk::DescriptorSetLayoutBinding> = bindings_and_flags
            .iter()
            .map(|(binding, _flag)| *binding)
            .collect();
        let binding_flags_create_info =
            vk::DescriptorSetLayoutBindingFlagsCreateInfo {
                binding_count: flags.len() as u32,
                p_binding_flags: flags.as_ptr(),
                ..Default::default()
            };
        let create_info = vk::DescriptorSetLayoutCreateInfo {
            p_next: &binding_flags_create_info
                as *const vk::DescriptorSetLayoutBindingFlagsCreateInfo
                as *const c_void,
            flags: vk::DescriptorSetLayoutCreateFlags::empty(),
            p_bindings: bindings.as_ptr(),
            binding_count: bindings.len() as u32,
            ..Default::default()
        };
        let raw = unsafe {
            vk_dev
                .logical_device
                .create_descriptor_set_layout(&create_info, None)
                .map_err(DescriptorSetError::UnableToCreateLayout)?
        };
        Ok(Self { raw, vk_dev })
    }
}

impl VulkanDebug for DescriptorSetLayout {
    fn set_debug_name(
        &self,
        debug_name: impl Into<String>,
    ) -> Result<(), VulkanDebugError> {
        self.vk_dev.name_vulkan_object(
            debug_name,
            vk::ObjectType::DESCRIPTOR_SET_LAYOUT,
            self.raw,
        )?;
        Ok(())
    }
}

impl Drop for DescriptorSetLayout {
    /// # DANGER
    ///
    /// There is no internal synchronization for this type. Unexpected behavior
    /// can occur if this instance is still in-use by the GPU when it is
    /// dropped.
    fn drop(&mut self) {
        unsafe {
            self.vk_dev
                .logical_device
                .destroy_descriptor_set_layout(self.raw, None);
        }
    }
}
