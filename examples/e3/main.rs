mod example_ui;

use ::{
    anyhow::Result,
    ccthw::{
        asset_loader::AssetLoader,
        demo::{run_application, State},
        glfw_window::GlfwWindow,
        immediate_mode_graphics::Frame,
        timing::FrameRateLimit,
        ui::UI,
        vulkan::{MemoryAllocator, RenderDevice},
    },
    std::sync::Arc,
};

use example_ui::{ExampleMessage, ExampleUi};

struct Example {
    ui: UI<ExampleUi>,
}

impl State for Example {
    fn init(
        window: &mut GlfwWindow,
        fps_limit: &mut FrameRateLimit,
        asset_loader: &mut AssetLoader,
        _vk_dev: &Arc<RenderDevice>,
        _vk_alloc: &Arc<dyn MemoryAllocator>,
    ) -> Result<Self> {
        let scale = window.window.get_content_scale();
        fps_limit.set_target_fps(120);
        Ok(Self {
            ui: UI::new(
                window.window.get_framebuffer_size().into(),
                ExampleUi::new(scale.0, asset_loader)?,
            ),
        })
    }

    fn handle_event(
        &mut self,
        event: glfw::WindowEvent,
        window: &mut GlfwWindow,
    ) -> Result<()> {
        match self.ui.handle_event(&event)? {
            Some(ExampleMessage::ToggleFullscreen) => {
                window.toggle_fullscreen()?
            }
            None => (),
        }
        Ok(())
    }

    fn draw_frame(&mut self, frame: &mut Frame) -> Result<()> {
        self.ui.draw_frame(frame)?;

        Ok(())
    }
}

impl Example {}

fn main() -> Result<()> {
    run_application::<Example>()
}
