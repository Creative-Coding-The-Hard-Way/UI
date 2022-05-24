mod example_ui;

use ::{
    anyhow::Result,
    ccthw::{
        asset_loader::AssetLoader,
        demo::{run_application, State},
        glfw_window::GlfwWindow,
        immediate_mode_graphics::{triangles::Frame, Sprite},
        math::projections,
        timing::FrameRateLimit,
        ui::UI,
        vulkan::{MemoryAllocator, RenderDevice},
        Mat4,
    },
    std::sync::Arc,
};

use example_ui::{ExampleMessage, ExampleUi};

struct Example {
    sprite_texture: i32,
    ui: UI<ExampleUi>,
    app_camera: Mat4,
}

impl Example {
    fn projection(aspect_ratio: f32) -> Mat4 {
        let height = 10.0;
        let width = height * aspect_ratio;
        projections::ortho(
            -0.5 * width,
            0.5 * width,
            -0.5 * height,
            0.5 * height,
            0.0,
            1.0,
        )
    }
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

        let (w, h) = window.window.get_framebuffer_size();
        let aspect_ratio = w as f32 / h as f32;

        fps_limit.set_target_fps(60);

        let sprite_texture =
            asset_loader.read_texture("assets/texture_orientation.png")?;

        Ok(Self {
            sprite_texture,
            ui: UI::new(
                window.window.get_framebuffer_size().into(),
                ExampleUi::new(scale.0, asset_loader)?,
            ),
            app_camera: Self::projection(aspect_ratio),
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
            _ => (),
        }

        match event {
            glfw::WindowEvent::FramebufferSize(w, h) => {
                self.app_camera = Self::projection(w as f32 / h as f32);
            }
            _ => (),
        }

        Ok(())
    }

    fn draw_frame(
        &mut self,
        app_frame: &mut Frame,
        ui_frame: &mut Frame,
    ) -> Result<()> {
        self.ui.draw_frame(ui_frame)?;

        app_frame.set_view_projection(self.app_camera)?;

        Sprite {
            width: 6.0,
            height: 6.0,
            texture_index: self.sprite_texture,
            angle_in_radians: self.ui.state().angle,
            ..Default::default()
        }
        .draw(app_frame)?;

        Ok(())
    }
}

impl Example {}

fn main() -> Result<()> {
    run_application::<Example>()
}
