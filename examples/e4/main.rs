use ::{
    anyhow::Result,
    ccthw::{
        asset_loader::AssetLoader,
        demo::{run_application, State},
        gen_id,
        glfw_window::GlfwWindow,
        immediate_mode_graphics::Frame,
        timing::FrameRateLimit,
        ui::{self, primitives::Rect, Button},
        vulkan::{MemoryAllocator, RenderDevice},
    },
    std::sync::Arc,
};

struct Example {
    ui: ui::State,
    counter: i32,
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
            counter: 0,
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
        frame.set_view_projection(self.ui.get_projection())?;

        let counter = &mut self.counter;
        self.ui.render(|ui| {
            use ccthw::ui::Id;
            let button1 = Button {
                dimensions: Rect::centered_at(200.0, 200.0, 200.0, 100.0),
                ..Default::default()
            };

            let button2 = Button {
                dimensions: Rect::centered_at(500.0, 200.0, 200.0, 100.0),
                ..Default::default()
            };

            if ui.button(frame, gen_id!(), button1)? {
                log::info!("CLICKED button 1, {} times", *counter);
                *counter = *counter + 1;
            }

            if ui.button(frame, gen_id!(), button2)? {
                log::info!("CLICKED button 2");
            }
            Ok(())
        })?;

        Ok(())
    }
}

impl Example {}

fn main() -> Result<()> {
    run_application::<Example>()
}
