use ::{anyhow::Result, std::sync::Arc};

use crate::{
    glfw_window::GlfwWindow,
    timing::FrameRateLimit,
    vulkan::{CommandBuffer, MemoryAllocator, RenderDevice},
};

pub trait State {
    /// Create a new instance of the Application state.
    fn init(
        window: &mut GlfwWindow,
        fps_limit: &mut FrameRateLimit,
        vk_dev: &Arc<RenderDevice>,
        vk_alloc: &Arc<dyn MemoryAllocator>,
    ) -> Result<Self>
    where
        Self: Sized;

    /// Update application state and push per-frame rendering commands to the
    /// given buffer.
    fn update(
        &mut self,
        swapchain_index: usize,
        frame_cmds: &CommandBuffer,
    ) -> Result<()>;

    /// Rebuild any swapchain dependent resources after it's been invalidated
    /// for some reason.
    fn rebuild_swapchain_resources(
        &mut self,
        window: &GlfwWindow,
        framebuffer_size: (u32, u32),
    ) -> Result<()>;

    /// Handle GLFW window events.
    fn handle_event(
        &mut self,
        _event: glfw::WindowEvent,
        _window: &mut GlfwWindow,
    ) -> Result<()> {
        Ok(())
    }
}
