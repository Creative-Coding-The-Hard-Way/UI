use ::{anyhow::Result, std::sync::Arc};

use crate::{
    asset_loader::AssetLoader,
    glfw_window::GlfwWindow,
    immediate_mode_graphics::Frame,
    timing::FrameRateLimit,
    vulkan::{MemoryAllocator, RenderDevice},
};

pub trait State {
    /// Create a new instance of the Application state.
    fn init(
        window: &mut GlfwWindow,
        fps_limit: &mut FrameRateLimit,
        asset_loader: &mut AssetLoader,
        vk_dev: &Arc<RenderDevice>,
        vk_alloc: &Arc<dyn MemoryAllocator>,
    ) -> Result<Self>
    where
        Self: Sized;

    /// Draw a single application frame to the screen.
    fn draw_frame(&mut self, frame: &mut Frame) -> Result<()>;

    /// Rebuild any swapchain dependent resources after it's been invalidated
    /// for some reason.
    fn rebuild_swapchain_resources(
        &mut self,
        _window: &GlfwWindow,
        _framebuffer_size: (u32, u32),
    ) -> Result<()> {
        Ok(())
    }

    /// Handle GLFW window events.
    fn handle_event(
        &mut self,
        _event: glfw::WindowEvent,
        _window: &mut GlfwWindow,
    ) -> Result<()> {
        Ok(())
    }
}
