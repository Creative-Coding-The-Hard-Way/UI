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
            ui_projection, MouseState,
        },
        vec4,
        vulkan::{MemoryAllocator, RenderDevice},
    },
    std::sync::Arc,
};

struct Example {
    camera: nalgebra::Matrix4<f32>,
    ui: ui::State,
}

impl State for Example {
    fn init(
        _window: &mut GlfwWindow,
        fps_limit: &mut FrameRateLimit,
        _asset_loader: &mut AssetLoader,
        _vk_dev: &Arc<RenderDevice>,
        _vk_alloc: &Arc<dyn MemoryAllocator>,
    ) -> Result<Self> {
        fps_limit.set_target_fps(120);
        Ok(Self {
            camera: nalgebra::Matrix4::identity(),
            ui: ui::State::new(),
        })
    }

    fn rebuild_swapchain_resources(
        &mut self,
        _window: &GlfwWindow,
        framebuffer_size: (u32, u32),
    ) -> Result<()> {
        self.camera = ui_projection(framebuffer_size.0, framebuffer_size.1);
        Ok(())
    }

    fn handle_event(
        &mut self,
        event: glfw::WindowEvent,
        _window: &mut GlfwWindow,
    ) -> Result<()> {
        self.ui.handle_event(event)
    }

    fn draw_frame(&mut self, frame: &mut Frame) -> Result<()> {
        frame.set_view_projection(self.camera)?;

        let mouse = self.ui.get_mouse_position();
        Tile {
            model: Rect::centered_at(mouse.x, mouse.y, 150.0, 150.0),
            color: if self.ui.get_mouse_state() == MouseState::Pressed {
                vec4(0.0, 0.0, 0.0, 1.0)
            } else {
                vec4(1.0, 1.0, 1.0, 1.0)
            },
            ..Default::default()
        }
        .fill(frame)?;

        Ok(())
    }
}

fn main() -> Result<()> {
    run_application::<Example>()
}
