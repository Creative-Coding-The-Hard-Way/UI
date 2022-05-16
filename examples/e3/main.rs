mod example_ui;

use ::{
    anyhow::Result,
    ccthw::{
        asset_loader::AssetLoader,
        demo::{run_application, State},
        glfw_window::GlfwWindow,
        immediate_mode_graphics::{Drawable, Frame},
        timing::FrameRateLimit,
        ui::{
            primitives::{Rect, Tile},
            UI,
        },
        vec4,
        vulkan::{MemoryAllocator, RenderDevice},
    },
    std::sync::Arc,
};

use example_ui::{ExampleMessage, ExampleUi};

struct Example {
    texture_index: i32,
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
        let texture_index = asset_loader.read_texture("assets/border.png")?;
        Ok(Self {
            texture_index,
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
            Some(_) => (),
            None => (),
        }
        Ok(())
    }

    fn draw_frame(&mut self, frame: &mut Frame) -> Result<()> {
        self.ui.draw_frame(frame)?;

        let tile = Tile {
            model: Rect::centered_at(400.0, 400.0, 400.0, 100.0),
            color: vec4(0.25, 0.25, 0.25, 1.0),
            ..Default::default()
        };
        tile.fill(frame)?;

        Tile {
            color: vec4(0.0, 0.0, 0.0, 1.0),
            outline_width: self.ui.state().border_width,
            texture_index: self.texture_index,
            ..tile
        }
        .outline(frame)?;

        Ok(())
    }
}

impl Example {}

fn main() -> Result<()> {
    run_application::<Example>()
}
