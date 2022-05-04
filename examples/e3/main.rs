use ::{
    anyhow::Result,
    ccthw::{
        asset_loader::AssetLoader,
        demo::{run_application, State},
        glfw_window::GlfwWindow,
        immediate_mode_graphics::{Drawable, Frame},
        math::projections,
        timing::FrameRateLimit,
        ui::primitives::{Line, Rect, Tile},
        vulkan::{MemoryAllocator, RenderDevice},
        Vec2, Vec4,
    },
    std::sync::Arc,
};

struct Example {
    camera: nalgebra::Matrix4<f32>,
    example3_texture_id: i32,
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

        let example3_texture_id =
            asset_loader.read_texture("assets/example3_tex1.jpg")?;

        Ok(Self {
            camera: nalgebra::Matrix4::identity(),
            example3_texture_id,
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

        Tile {
            model: Rect::centered_at(-200.0, 0.0, 150.0, 150.0),
            texture_index: self.example3_texture_id,
            ..Default::default()
        }
        .fill(frame)?;

        let img2 = Tile {
            model: Rect::centered_at(200.0, 0.0, 200.0, 200.0),
            outline_width: 5.0,
            texture_index: self.example3_texture_id,
            ..Default::default()
        };
        img2.fill(frame)?;
        img2.outline(frame)?;

        Line {
            start: Vec2::new(350.0, 150.0),
            end: Vec2::new(-350.0, 150.0),
            width: 2.0,
            color: Vec4::new(0.5, 0.5, 0.8, 1.0),
            ..Default::default()
        }
        .fill(frame)?;

        Line {
            start: Vec2::new(350.0, -150.0),
            end: Vec2::new(-350.0, -150.0),
            width: 2.0,
            color: Vec4::new(0.5, 0.5, 0.8, 1.0),
            ..Default::default()
        }
        .fill(frame)?;

        Ok(())
    }
}

fn main() -> Result<()> {
    run_application::<Example>()
}
