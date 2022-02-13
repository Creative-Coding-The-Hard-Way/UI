//! This module defines the main application initialization, event loop, and
//! rendering.

use std::sync::Arc;

use anyhow::{Context, Result};
use ccthw::{
    asset_loader::AssetLoader,
    frame_pipeline::{FrameError, FramePipeline},
    glfw_window::GlfwWindow,
    graphics2::{Frame, Graphics2, Vec2},
    math::projections,
    multisample_renderpass::MultisampleRenderpass,
    timing::FrameRateLimit,
    vulkan::{self, Framebuffer, MemoryAllocator, RenderDevice},
};

// The main application state.
pub struct Application {
    // renderers
    msaa_renderpass: MultisampleRenderpass,
    framebuffers: Vec<Framebuffer>,
    graphics2: Graphics2,
    camera: nalgebra::Matrix4<f32>,

    // app state
    fps_limit: FrameRateLimit,
    paused: bool,
    swapchain_needs_rebuild: bool,
    _asset_loader: AssetLoader,

    // vulkan core
    frame_pipeline: FramePipeline,
    vk_dev: Arc<RenderDevice>,
    vk_alloc: Arc<dyn MemoryAllocator>,
    glfw_window: GlfwWindow,
}

impl Application {
    /// Build a new instance of the application.
    pub fn new() -> Result<Self> {
        let mut glfw_window = GlfwWindow::new("Swapchain")?;
        glfw_window.window.set_key_polling(true);
        glfw_window.window.set_framebuffer_size_polling(true);

        // Create the vulkan render device
        let vk_dev = Arc::new(glfw_window.create_vulkan_device()?);
        let vk_alloc = vulkan::create_default_allocator(vk_dev.clone());

        let mut asset_loader =
            AssetLoader::new(vk_dev.clone(), vk_alloc.clone())?;
        let texture1 = asset_loader.read_texture("assets/example3_tex1.jpg")?;

        // Create per-frame resources and the renderpass
        let frame_pipeline = FramePipeline::new(vk_dev.clone())?;

        // create the renderer
        let msaa_renderpass = MultisampleRenderpass::for_current_swapchain(
            vk_dev.clone(),
            vk_alloc.clone(),
        )?;
        let framebuffers = msaa_renderpass.create_swapchain_framebuffers()?;
        let graphics2 = Graphics2::new(
            &msaa_renderpass,
            &[texture1],
            vk_alloc.clone(),
            vk_dev.clone(),
        )?;

        let (w, h) = glfw_window.window.get_framebuffer_size();
        let (half_width, half_height) = (w as f32 / 2.0, h as f32 / 2.0);
        let camera = projections::ortho(
            -half_width,
            half_width,
            -half_height,
            half_height,
            0.0,
            1.0,
        );

        Ok(Self {
            msaa_renderpass,
            framebuffers,
            graphics2,
            camera,

            fps_limit: FrameRateLimit::new(60, 30),
            paused: false,
            swapchain_needs_rebuild: false,
            _asset_loader: asset_loader,

            frame_pipeline,
            vk_dev,
            vk_alloc,
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
        let (index, _) = self.frame_pipeline.begin_frame()?;

        let mut frame = self
            .graphics2
            .acquire_frame(index)
            .with_context(|| "unable to acquire graphics2 frame")?;
        self.draw(&mut frame)?;

        unsafe {
            let cmd = self.frame_pipeline.frame_cmds(index);
            self.msaa_renderpass.begin_renderpass_inline(
                cmd,
                &self.framebuffers[index],
                [0.0, 0.0, 0.0, 1.0],
                1.0,
            );
            self.graphics2.complete_frame(cmd, frame, index)?;
            self.msaa_renderpass.end_renderpass(cmd);
        };

        self.frame_pipeline.end_frame(index)
    }

    fn draw(&mut self, frame: &mut Frame) -> Result<()> {
        frame.set_view_projection(self.camera)?;
        frame.draw_quad(Vec2::new(-200.0, 0.0), Vec2::new(150.0, 150.0), 0)?;
        Ok(())
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
        self.msaa_renderpass = MultisampleRenderpass::for_current_swapchain(
            self.vk_dev.clone(),
            self.vk_alloc.clone(),
        )?;
        self.framebuffers =
            self.msaa_renderpass.create_swapchain_framebuffers()?;
        self.graphics2
            .rebuild_swapchain_resources(&self.msaa_renderpass)?;

        let (half_width, half_height) = (w as f32 / 2.0, h as f32 / 2.0);
        self.camera = projections::ortho(
            -half_width,
            half_width,
            -half_height,
            half_height,
            0.0,
            1.0,
        );
        Ok(())
    }

    /// Handle a GLFW window event.
    fn handle_event(&mut self, event: glfw::WindowEvent) -> Result<()> {
        use glfw::{Action, Key, Modifiers, WindowEvent};
        match event {
            WindowEvent::Close => {
                self.glfw_window.window.set_should_close(true);
            }
            WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                self.glfw_window.window.set_should_close(true);
            }
            WindowEvent::Key(
                Key::Space,
                _,
                Action::Press,
                Modifiers::Control,
            ) => {
                self.glfw_window.toggle_fullscreen()?;
            }
            WindowEvent::FramebufferSize(w, h) => {
                self.paused = w == 0 || h == 0;
                self.swapchain_needs_rebuild = true;
            }
            _ => {}
        }
        Ok(())
    }
}

impl Drop for Application {
    fn drop(&mut self) {
        unsafe {
            self.vk_dev
                .logical_device
                .device_wait_idle()
                .expect("error while waiting for graphics device idle");
        }
    }
}
