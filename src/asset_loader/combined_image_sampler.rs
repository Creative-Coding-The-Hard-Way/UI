use std::sync::Arc;

use crate::vulkan::{ImageView, Sampler};

// A ImageView and Sampler pair which can be stored and moved around easily.
#[derive(Clone)]
pub struct CombinedImageSampler {
    pub image_view: Arc<ImageView>,
    pub sampler: Arc<Sampler>,
}

impl CombinedImageSampler {
    // Take ownership of the given sampler and view to create a combined image
    // sampler.
    pub fn of(image_view: ImageView, sampler: Sampler) -> Self {
        Self::new(Arc::new(image_view), Arc::new(sampler))
    }

    // Create a combined image sampler from the given view and sampler.
    pub fn new(image_view: Arc<ImageView>, sampler: Arc<Sampler>) -> Self {
        Self {
            image_view,
            sampler,
        }
    }
}
