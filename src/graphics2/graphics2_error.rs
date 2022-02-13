use ::thiserror::Error;

#[derive(Debug, Error)]
pub enum Graphics2Error {
    #[error("The Per-Frame resources for swapchain image {} were not available! Did you forget to end the previous frame?", .0)]
    FrameResourcesUnavailable(usize),
}
