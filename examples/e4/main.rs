use ::{
    anyhow::Result,
    ccthw::{
        asset_loader::AssetLoader,
        demo::{run_application, State},
        glfw_window::GlfwWindow,
        immediate_mode_graphics::{Drawable, Frame},
        timing::FrameRateLimit,
        ui::{
            self,
            primitives::{Rect, Tile},
            MouseState,
        },
        vec2, vec4,
        vulkan::{MemoryAllocator, RenderDevice},
    },
    std::sync::Arc,
};

struct Example {
    ui: ui::State,
}

impl State for Example {
    fn init(
        window: &mut GlfwWindow,
        fps_limit: &mut FrameRateLimit,
        _asset_loader: &mut AssetLoader,
        _vk_dev: &Arc<RenderDevice>,
        _vk_alloc: &Arc<dyn MemoryAllocator>,
    ) -> Result<Self> {
        fps_limit.set_target_fps(120);
        let screen_dimensions = window.window.get_framebuffer_size();
        Ok(Self {
            ui: ui::State::new(screen_dimensions.0, screen_dimensions.1),
        })
    }

    fn handle_event(
        &mut self,
        event: glfw::WindowEvent,
        _window: &mut GlfwWindow,
    ) -> Result<()> {
        self.ui.handle_event(event)
    }

    fn draw_frame(&mut self, frame: &mut Frame) -> Result<()> {
        use ccthw::ui::Id;
        frame.set_view_projection(self.ui.get_projection())?;

        self.ui.prepare();

        if self.ui.button(frame, Id::Number(1), vec2(200.0, 200.0))? {
            log::info!("CLICKED One!")
        }

        if self.ui.button(frame, Id::Number(2), vec2(500.0, 200.0))? {
            log::info!("CLICKED two!")
        }

        self.ui.finish();

        Ok(())
    }
}

fn main() -> Result<()> {
    run_application::<Example>()
}
