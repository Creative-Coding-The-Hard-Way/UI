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
            primitives::{Line, Rect, Tile},
            text::Text,
        },
        vec2,
        vulkan::{MemoryAllocator, RenderDevice},
    },
    std::sync::Arc,
};

struct Example {
    ui: ui::State,
    example3_texture_id: i32,
    big_text: Text,
    little_text: Text,
    screen_dims: (f32, f32),
}

impl State for Example {
    fn init(
        window: &mut GlfwWindow,
        fps_limit: &mut FrameRateLimit,
        asset_loader: &mut AssetLoader,
        _vk_dev: &Arc<RenderDevice>,
        _vk_alloc: &Arc<dyn MemoryAllocator>,
    ) -> Result<Self> {
        fps_limit.set_target_fps(120);

        let (w, h) = window.window.get_framebuffer_size();
        let example3_texture_id =
            asset_loader.read_texture("assets/example3_tex1.jpg")?;

        let big_text = Text::from_font_file(
            "assets/Roboto-Regular.ttf",
            64.0,
            asset_loader,
        )?;

        let little_text = Text::from_font_file(
            "assets/Roboto-Regular.ttf",
            32.0,
            asset_loader,
        )?;

        Ok(Self {
            ui: ui::State::new(w, h),
            example3_texture_id,
            screen_dims: (w as f32, h as f32),
            big_text,
            little_text,
        })
    }

    fn draw_frame(&mut self, frame: &mut Frame) -> Result<()> {
        frame.set_view_projection(self.ui.get_projection())?;

        Tile {
            model: Rect::centered_at(200.0, 200.0, 150.0, 150.0),
            texture_index: self.example3_texture_id,
            ..Default::default()
        }
        .fill(frame)?;

        let img2 = Tile {
            model: Rect::centered_at(500.0, 200.0, 200.0, 200.0),
            outline_width: 5.0,
            texture_index: self.example3_texture_id,
            ..Default::default()
        };
        img2.fill(frame)?;
        img2.outline(frame)?;

        Line {
            start: vec2(0.0, 50.0),
            end: vec2(self.screen_dims.0, 50.0),
            width: 5.0,
            ..Default::default()
        }
        .fill(frame)?;

        Line {
            start: vec2(0.0, 350.0),
            end: vec2(self.screen_dims.0, 350.0),
            width: 5.0,
            ..Default::default()
        }
        .fill(frame)?;

        self.big_text
            .draw_text(frame, vec2(50.0, 450.0), &"Hello World")?;

        self.little_text.draw_text(
            frame,
            vec2(450.0, 450.0),
            &"Hello World\nhello WORLD\nLorem Ipsum dolor",
        )?;

        Line {
            start: vec2(50.0, 450.0),
            end: vec2(self.screen_dims.0, 450.0),
            width: 1.0,
            ..Default::default()
        }
        .fill(frame)?;

        Ok(())
    }
}

fn main() -> Result<()> {
    run_application::<Example>()
}
