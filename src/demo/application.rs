use ::{anyhow::Result, std::sync::Arc};

use crate::{
    demo::State,
    frame_pipeline::{FrameError, FramePipeline},
    glfw_window::GlfwWindow,
    timing::FrameRateLimit,
    vulkan::{self, MemoryAllocator, RenderDevice},
};

pub struct Application<S: State> {
    fps_limit: FrameRateLimit,
    paused: bool,
    swapchain_needs_rebuild: bool,
    frame_pipeline: FramePipeline,
    vk_dev: Arc<RenderDevice>,
    _vk_alloc: Arc<dyn MemoryAllocator>,
    glfw_window: GlfwWindow,
    state: S,
}

impl<S: State> Application<S> {
    /// Create a new application instance.
    pub fn new() -> Result<Self> {
        let mut glfw_window = GlfwWindow::new("Swapchain")?;
        let vk_dev = Arc::new(glfw_window.create_vulkan_device()?);
        let vk_alloc = vulkan::create_default_allocator(vk_dev.clone());
        let frame_pipeline = FramePipeline::new(vk_dev.clone())?;
        let mut fps_limit = FrameRateLimit::new(60, 30);

        glfw_window.window.set_key_polling(true);
        glfw_window.window.set_framebuffer_size_polling(true);

        Ok(Self {
            state: S::init(
                &mut glfw_window,
                &mut fps_limit,
                &vk_dev,
                &vk_alloc,
            )?,
            paused: false,
            swapchain_needs_rebuild: true,
            fps_limit,
            frame_pipeline,
            vk_dev,
            _vk_alloc: vk_alloc,
            glfw_window,
        })
    }

    /// Run the application, blocks until the main event loop exits.
    pub fn run(mut self) -> Result<()> {
        let event_receiver = self.glfw_window.take_event_receiver()?;
        while !self.glfw_window.window.should_close() {
            self.fps_limit.start_frame();
            for (_, event) in
                self.glfw_window.flush_window_events(&event_receiver)
            {
                self.handle_event(event)?;
            }
            if self.swapchain_needs_rebuild {
                self.rebuild_swapchain_resources()?;
                self.swapchain_needs_rebuild = false;
            }
            if !self.paused {
                let result = self.compose_frame();
                match result {
                    Err(FrameError::SwapchainNeedsRebuild) => {
                        self.swapchain_needs_rebuild = true;
                    }
                    _ => result?,
                }
            }
            self.fps_limit.sleep_to_limit();
        }
        Ok(())
    }

    /// Render the applications state in in a three-step process.
    fn compose_frame(&mut self) -> Result<(), FrameError> {
        let (index, cmd) = self.frame_pipeline.begin_frame()?;
        self.state.update(index, cmd)?;
        self.frame_pipeline.end_frame(index)
    }

    /// Rebuild the swapchain and any dependent resources.
    fn rebuild_swapchain_resources(&mut self) -> Result<()> {
        if self.paused {
            self.glfw_window.glfw.wait_events();
            return Ok(());
        }
        unsafe {
            self.vk_dev.logical_device.device_wait_idle()?;
        }
        let (w, h) = self.glfw_window.window.get_framebuffer_size();
        self.vk_dev.rebuild_swapchain((w as u32, h as u32))?;
        self.frame_pipeline.rebuild_swapchain_resources()?;

        self.state.rebuild_swapchain_resources(
            &self.glfw_window,
            (w as u32, h as u32),
        )
    }

    /// Handle a GLFW window event.
    fn handle_event(&mut self, event: glfw::WindowEvent) -> Result<()> {
        use glfw::WindowEvent;
        match event {
            WindowEvent::Close => {
                self.glfw_window.window.set_should_close(true);
            }
            WindowEvent::FramebufferSize(w, h) => {
                self.paused = w == 0 || h == 0;
                self.swapchain_needs_rebuild = true;
            }
            _ => {}
        }

        self.state.handle_event(event, &mut self.glfw_window)
    }
}
