use ccthw::{immediate_mode_graphics::Drawable, ui::primitives::Line};

use ::{
    anyhow::{Context, Result},
    ccthw::{
        asset_loader::AssetLoader,
        demo::{run_application, State},
        glfw_window::GlfwWindow,
        immediate_mode_graphics::{ImmediateModeGraphics, Text},
        math::projections,
        multisample_renderpass::MultisampleRenderpass,
        timing::FrameRateLimit,
        vec2,
        vulkan::{Framebuffer, MemoryAllocator, RenderDevice},
        Vec2,
    },
    std::sync::Arc,
};

struct Example {
    msaa_renderpass: MultisampleRenderpass,
    framebuffers: Vec<Framebuffer>,
    immediate_mode_graphics: ImmediateModeGraphics,
    camera: nalgebra::Matrix4<f32>,
    _asset_loader: AssetLoader,
    text: Text,
    vk_alloc: Arc<dyn MemoryAllocator>,
    vk_dev: Arc<RenderDevice>,
}

impl State for Example {
    fn init(
        _window: &mut GlfwWindow,
        fps_limit: &mut FrameRateLimit,
        vk_dev: &Arc<RenderDevice>,
        vk_alloc: &Arc<dyn MemoryAllocator>,
    ) -> Result<Self> {
        fps_limit.set_target_fps(120);
        let msaa_renderpass = MultisampleRenderpass::for_current_swapchain(
            vk_dev.clone(),
            vk_alloc.clone(),
        )?;
        let framebuffers = msaa_renderpass.create_swapchain_framebuffers()?;
        let mut asset_loader =
            AssetLoader::new(vk_dev.clone(), vk_alloc.clone())?;
        let text = Text::from_font_file("assets/Roboto-Regular.ttf", 64.0)?;
        let immediate_mode_graphics = ImmediateModeGraphics::new(
            &msaa_renderpass,
            &[
                asset_loader.blank_white()?,
                asset_loader
                    .create_texture_with_data(&[text.rasterized.clone()])?,
            ],
            vk_alloc.clone(),
            vk_dev.clone(),
        )?;
        Ok(Self {
            msaa_renderpass,
            framebuffers,
            immediate_mode_graphics,
            camera: nalgebra::Matrix4::identity(),
            text,
            _asset_loader: asset_loader,
            vk_alloc: vk_alloc.clone(),
            vk_dev: vk_dev.clone(),
        })
    }

    fn rebuild_swapchain_resources(
        &mut self,
        _window: &GlfwWindow,
        framebuffer_size: (u32, u32),
    ) -> Result<()> {
        self.msaa_renderpass = MultisampleRenderpass::for_current_swapchain(
            self.vk_dev.clone(),
            self.vk_alloc.clone(),
        )?;
        self.framebuffers =
            self.msaa_renderpass.create_swapchain_framebuffers()?;
        self.immediate_mode_graphics
            .rebuild_swapchain_resources(&self.msaa_renderpass)?;
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

    fn update(
        &mut self,
        index: usize,
        cmds: &ccthw::vulkan::CommandBuffer,
    ) -> Result<()> {
        unsafe {
            self.msaa_renderpass.begin_renderpass_inline(
                cmds,
                &self.framebuffers[index],
                [0.05, 0.05, 0.05, 1.0],
                1.0,
            );
        }

        let mut frame = self
            .immediate_mode_graphics
            .acquire_frame(index)
            .with_context(|| "unable to acquire graphics2 frame")?;
        frame.set_view_projection(self.camera)?;

        Line {
            start: vec2(-10000.0, 0.0),
            end: vec2(10000.0, 0.0),
            ..Default::default()
        }
        .fill(&mut frame)?;

        Line {
            start: vec2(0.0, -10000.0),
            end: vec2(0.0, 10000.0),
            ..Default::default()
        }
        .fill(&mut frame)?;

        Line {
            start: vec2(0.0, -100.0),
            end: vec2(2000.0, -100.0),
            ..Default::default()
        }
        .fill(&mut frame)?;

        self.text.draw_text(
            &mut frame,
            Vec2::new(20.0, -100.0),
            &"Hello World\nThis is some Text.",
        )?;

        unsafe {
            self.immediate_mode_graphics
                .complete_frame(cmds, frame, index)?;
            self.msaa_renderpass.end_renderpass(cmds);
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    run_application::<Example>()
}
