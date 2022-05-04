use ::{
    anyhow::Result,
    ccthw::{
        asset_loader::AssetLoader,
        demo::{run_application, State},
        glfw_window::GlfwWindow,
        immediate_mode_graphics::{Drawable, Frame},
        math::projections,
        timing::FrameRateLimit,
        ui::{primitives::Line, text::Text},
        vec2,
        vulkan::{MemoryAllocator, RenderDevice},
        Vec2,
    },
    std::sync::Arc,
};

struct Example {
    camera: nalgebra::Matrix4<f32>,
    text: Text,
}

impl State for Example {
    fn init(
        _window: &mut GlfwWindow,
        fps_limit: &mut FrameRateLimit,
        asset_loader: &mut AssetLoader,
        _vk_dev: &Arc<RenderDevice>,
        _vk_alloc: &Arc<dyn MemoryAllocator>,
    ) -> Result<Self> {
        fps_limit.set_target_fps(120);
        let text = Text::from_font_file(
            "assets/Roboto-Regular.ttf",
            24.0,
            asset_loader,
        )?;
        Ok(Self {
            camera: nalgebra::Matrix4::identity(),
            text,
        })
    }

    fn rebuild_swapchain_resources(
        &mut self,
        _window: &GlfwWindow,
        framebuffer_size: (u32, u32),
    ) -> Result<()> {
        let (half_width, half_height) = (
            framebuffer_size.0 as f32 / 2.0,
            framebuffer_size.1 as f32 / 2.0,
        );
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

    fn draw_frame(&mut self, frame: &mut Frame) -> Result<()> {
        frame.set_view_projection(self.camera)?;

        Line {
            start: vec2(-10000.0, 0.0),
            end: vec2(10000.0, 0.0),
            ..Default::default()
        }
        .fill(frame)?;

        Line {
            start: vec2(0.0, -10000.0),
            end: vec2(0.0, 10000.0),
            ..Default::default()
        }
        .fill(frame)?;

        Line {
            start: vec2(0.0, -100.0),
            end: vec2(2000.0, -100.0),
            ..Default::default()
        }
        .fill(frame)?;

        self.text.draw_text(
            frame,
            Vec2::new(20.0, -100.0),
            &"Hello World\nThis is some Text.",
        )?;

        Ok(())
    }
}

fn main() -> Result<()> {
    run_application::<Example>()
}
