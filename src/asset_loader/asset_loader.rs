use ::{
    anyhow::Result,
    ash::vk,
    image::io::Reader,
    std::{path::Path, sync::Arc},
};

use crate::{
    asset_loader::CombinedImageSampler,
    vulkan::{
        errors::VulkanError, GpuVec, Image, ImageView, MemoryAllocator,
        OneTimeSubmitCommandPool, RenderDevice, Sampler,
    },
};

pub struct AssetLoader {
    textures: Vec<CombinedImageSampler>,
    default_sampler: Arc<Sampler>,
    staging_buffer: GpuVec<u8>,
    command_pool: OneTimeSubmitCommandPool,
    vk_alloc: Arc<dyn MemoryAllocator>,
    vk_dev: Arc<RenderDevice>,
}

impl AssetLoader {
    // Create a new asset loader
    pub fn new(
        vk_dev: Arc<RenderDevice>,
        vk_alloc: Arc<dyn MemoryAllocator>,
    ) -> Result<Self, VulkanError> {
        Ok(Self {
            textures: vec![],
            default_sampler: Arc::new(Sampler::linear(vk_dev.clone())?),
            staging_buffer: GpuVec::new(
                vk_dev.clone(),
                vk_alloc.clone(),
                vk::BufferUsageFlags::TRANSFER_SRC,
                (8 * 4) * 512 * 512,
            )?,
            command_pool: OneTimeSubmitCommandPool::new(
                vk_dev.clone(),
                &vk_dev.graphics_queue,
            )?,
            vk_alloc,
            vk_dev,
        })
    }

    // Load a texture from the image at the given path.
    pub fn read_texture<T>(
        &mut self,
        path_to_texture_image: T,
    ) -> Result<CombinedImageSampler>
    where
        T: AsRef<Path>,
    {
        let loaded = Reader::open(path_to_texture_image)?.decode()?;
        let rgba = loaded.flipv().into_rgba8();
        let (width, height) = (rgba.width(), rgba.height());
        let vulkan_image = self.create_empty_2d(width, height)?;

        self.staging_buffer.clear();
        for v in rgba.as_raw() {
            self.staging_buffer.push_back(*v)?;
        }

        self.command_pool
            .submit_sync_commands(|vk_dev, cmd| unsafe {
                let prepare_write_barrier = vk::ImageMemoryBarrier {
                    src_access_mask: vk::AccessFlags::empty(),
                    dst_access_mask: vk::AccessFlags::TRANSFER_WRITE,
                    old_layout: vk::ImageLayout::UNDEFINED,
                    new_layout: vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                    src_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
                    dst_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
                    image: vulkan_image.raw,
                    subresource_range: vk::ImageSubresourceRange {
                        aspect_mask: vk::ImageAspectFlags::COLOR,
                        base_mip_level: 0,
                        level_count: 1,
                        base_array_layer: 0,
                        layer_count: 1,
                    },
                    ..Default::default()
                };
                vk_dev.logical_device.cmd_pipeline_barrier(
                    cmd,
                    vk::PipelineStageFlags::TOP_OF_PIPE,
                    vk::PipelineStageFlags::TRANSFER,
                    vk::DependencyFlags::empty(),
                    &[],
                    &[],
                    &[prepare_write_barrier],
                );

                let buffer_image_copy = vk::BufferImageCopy {
                    buffer_offset: 0,
                    buffer_row_length: 0,
                    buffer_image_height: 0,
                    image_subresource: vk::ImageSubresourceLayers {
                        aspect_mask: vk::ImageAspectFlags::COLOR,
                        mip_level: 0,
                        base_array_layer: 0,
                        layer_count: 1,
                    },
                    image_offset: vk::Offset3D::default(),
                    image_extent: vk::Extent3D {
                        width,
                        height,
                        depth: 1,
                    },
                };
                vk_dev.logical_device.cmd_copy_buffer_to_image(
                    cmd,
                    self.staging_buffer.buffer.raw,
                    vulkan_image.raw,
                    vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                    &[buffer_image_copy],
                );

                let prepare_read_barrier = vk::ImageMemoryBarrier {
                    src_access_mask: vk::AccessFlags::TRANSFER_WRITE,
                    dst_access_mask: vk::AccessFlags::SHADER_READ,
                    old_layout: vk::ImageLayout::TRANSFER_DST_OPTIMAL,
                    new_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
                    src_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
                    dst_queue_family_index: vk::QUEUE_FAMILY_IGNORED,
                    image: vulkan_image.raw,
                    subresource_range: vk::ImageSubresourceRange {
                        aspect_mask: vk::ImageAspectFlags::COLOR,
                        base_mip_level: 0,
                        level_count: 1,
                        base_array_layer: 0,
                        layer_count: 1,
                    },
                    ..Default::default()
                };
                vk_dev.logical_device.cmd_pipeline_barrier(
                    cmd,
                    vk::PipelineStageFlags::TRANSFER,
                    vk::PipelineStageFlags::FRAGMENT_SHADER,
                    vk::DependencyFlags::empty(),
                    &[],
                    &[],
                    &[prepare_read_barrier],
                );
            })?;
        let image_view = Arc::new(ImageView::new_2d(
            Arc::new(vulkan_image),
            vk::Format::R8G8B8A8_SRGB,
            vk::ImageAspectFlags::COLOR,
        )?);
        let texture =
            CombinedImageSampler::new(image_view, self.default_sampler.clone());
        self.textures.push(texture.clone());
        Ok(texture)
    }
}

impl AssetLoader {
    fn create_empty_2d(&mut self, width: u32, height: u32) -> Result<Image> {
        let create_info = vk::ImageCreateInfo {
            flags: vk::ImageCreateFlags::empty(),
            image_type: vk::ImageType::TYPE_2D,
            format: vk::Format::R8G8B8A8_SRGB,
            extent: vk::Extent3D {
                width,
                height,
                depth: 1,
            },
            mip_levels: 1,
            array_layers: 1,
            samples: vk::SampleCountFlags::TYPE_1,
            tiling: vk::ImageTiling::OPTIMAL,
            usage: vk::ImageUsageFlags::TRANSFER_DST
                | vk::ImageUsageFlags::SAMPLED,
            sharing_mode: vk::SharingMode::EXCLUSIVE,
            ..Default::default()
        };
        let image = Image::new(
            self.vk_dev.clone(),
            self.vk_alloc.clone(),
            &create_info,
            vk::MemoryPropertyFlags::DEVICE_LOCAL,
        )?;
        Ok(image)
    }
}
