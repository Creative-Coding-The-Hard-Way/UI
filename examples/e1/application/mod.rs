//! This module defines the main application initialization, event loop, and
//! rendering.

mod passthrough;

use ccthw::{
    frame_pipeline::{FrameError, FramePipeline},
    glfw_window::GlfwWindow,
    multisample_renderpass::MultisampleRenderpass,
    timing::FrameRateLimit,
    vulkan::{self, Framebuffer, MemoryAllocator, RenderDevice},
};
use ::{anyhow::Result, std::sync::Arc};

use self::passthrough::{Passthrough, Vertex2D};

// The main application state.
pub struct Application {
    // renderers
    msaa_renderpass: MultisampleRenderpass,
    framebuffers: Vec<Framebuffer>,
    passthrough: Passthrough,

    // app state
    fps_limit: FrameRateLimit,
    paused: bool,
    swapchain_needs_rebuild: bool,

    // vulkan core
    frame_pipeline: FramePipeline,
    vk_dev: Arc<RenderDevice>,
    vk_alloc: Arc<dyn MemoryAllocator>,
    glfw_window: GlfwWindow,
}

impl Application {
    /// Build a new instance of the application.
    pub fn new() -> Result<Self> {
        let mut glfw_window = GlfwWindow::new("Ortho Transform")?;
        glfw_window.window.set_key_polling(true);
        glfw_window.window.set_framebuffer_size_polling(true);

        // Create the vulkan render device
        let vk_dev = Arc::new(glfw_window.create_vulkan_device()?);
        let vk_alloc = vulkan::create_default_allocator(vk_dev.clone());

        // Create per-frame resources and the renderpass
        let frame_pipeline = FramePipeline::new(vk_dev.clone())?;

        // create the renderer
        let msaa_renderpass = MultisampleRenderpass::for_current_swapchain(
            vk_dev.clone(),
            vk_alloc.clone(),
        )?;
        let framebuffers = msaa_renderpass.create_swapchain_framebuffers()?;
        let mut passthrough = Passthrough::new(
            &msaa_renderpass,
            vk_alloc.clone(),
            vk_dev.clone(),
        )?;
        passthrough.push_vertices(&[
            /////////////////////////////
            // Draw the /near/ quad first.
            // Depth ranges from 0.0 on the near plane to 1.0 on the far plane,
            // so this quad is as close as it can be
            /////////////////////////////
            Vertex2D {
                pos: [-50.0, -50.0, 0.0],
                rgba: [0.2, 0.2, 0.2, 1.0],
            },
            Vertex2D {
                pos: [-50.0, 50.0, 0.0],
                rgba: [0.2, 0.2, 0.2, 1.0],
            },
            Vertex2D {
                pos: [50.0, 50.0, 0.0],
                rgba: [0.2, 0.2, 0.2, 1.0],
            },
            Vertex2D {
                pos: [-50.0, -50.0, 0.0],
                rgba: [0.2, 0.2, 0.2, 1.0],
            },
            Vertex2D {
                pos: [50.0, 50.0, 0.0],
                rgba: [0.2, 0.2, 0.2, 1.0],
            },
            Vertex2D {
                pos: [50.0, -50.0, 0.0],
                rgba: [0.2, 0.2, 0.2, 1.0],
            },
            ////////////////////////////////////////
            // Draw the /far/ quad second.
            // If depth testing is disabled this will completely occlude the
            // 'near' quad because of the draw order. BUT with depth testing
            // enabled, the near quad's fragments will overwrite the
            // foreground.
            //////////////////////////////////////
            Vertex2D {
                pos: [-150.0, -150.0, 0.5],
                rgba: [1.0, 1.0, 0.8, 1.0],
            },
            Vertex2D {
                pos: [-150.0, 150.0, 0.5],
                rgba: [1.0, 1.0, 0.8, 1.0],
            },
            Vertex2D {
                pos: [150.0, 150.0, 0.5],
                rgba: [1.0, 1.0, 0.8, 1.0],
            },
            Vertex2D {
                pos: [-150.0, -150.0, 0.0],
                rgba: [1.0, 1.0, 0.8, 1.0],
            },
            Vertex2D {
                pos: [150.0, 150.0, 0.0],
                rgba: [1.0, 1.0, 0.8, 1.0],
            },
            Vertex2D {
                pos: [150.0, -150.0, 0.0],
                rgba: [1.0, 1.0, 0.8, 1.0],
            },
        ])?;

        Ok(Self {
            msaa_renderpass,
            framebuffers,
            passthrough,

            fps_limit: FrameRateLimit::new(60, 30),
            paused: false,
            swapchain_needs_rebuild: false,

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
        let (index, cmd) = self.frame_pipeline.begin_frame()?;

        unsafe {
            self.msaa_renderpass.begin_renderpass_inline(
                cmd,
                &self.framebuffers[index],
                [0.0, 0.0, 0.0, 1.0],
                1.0,
            );
            self.passthrough.write_commands(cmd)?;
            self.msaa_renderpass.end_renderpass(cmd);
        };

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
        self.msaa_renderpass = MultisampleRenderpass::for_current_swapchain(
            self.vk_dev.clone(),
            self.vk_alloc.clone(),
        )?;
        self.framebuffers =
            self.msaa_renderpass.create_swapchain_framebuffers()?;
        self.passthrough
            .rebuild_swapchain_resources(&self.msaa_renderpass)?;

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
