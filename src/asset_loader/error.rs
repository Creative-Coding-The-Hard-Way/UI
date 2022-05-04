use ::{image::ImageError, std::io, thiserror::Error};

use crate::vulkan::errors::VulkanError;

#[derive(Debug, Error)]
pub enum AssetLoaderError {
    #[error("An unexpected Vulkan error occured!")]
    VulkanErrorWhileLoadingAssets(#[from] VulkanError),

    #[error("Unable to open the texture file")]
    UnableToOpenFile(#[from] io::Error),

    #[error("Unable to decode the texture file into rgba.")]
    UnableToDecodeImage(#[from] ImageError),
}
