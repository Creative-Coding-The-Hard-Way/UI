use ccthw::asset_loader::MipmapData;

use ::{
    ab_glyph::{Font, FontArc, Glyph, PxScaleFont, ScaleFont},
    anyhow::{Context, Result},
    ccthw::{
        asset_loader::AssetLoader,
        demo::{run_application, State},
        glfw_window::GlfwWindow,
        graphics2::{
            primitives::{Quad, Rect},
            Draw2D, Graphics2, LineArgs, Vec2, Vec4,
        },
        math::projections,
        multisample_renderpass::MultisampleRenderpass,
        timing::FrameRateLimit,
        vulkan::{Framebuffer, MemoryAllocator, RenderDevice},
    },
    std::sync::Arc,
    std::{fs::File, io::Read},
};

struct Example {
    msaa_renderpass: MultisampleRenderpass,
    framebuffers: Vec<Framebuffer>,
    graphics2: Graphics2,
    camera: nalgebra::Matrix4<f32>,
    _asset_loader: AssetLoader,
    font: PxScaleFont<FontArc>,
    glyph: Glyph,
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

        let bytes = {
            let mut buffer = vec![];
            File::open("assets/Roboto-Regular.ttf")?
                .read_to_end(&mut buffer)?;
            buffer
        };
        let font = FontArc::try_from_vec(bytes)?.into_scaled(64.0);
        let glyph = font.scaled_glyph('m');
        let outline = font.outline_glyph(glyph.clone()).unwrap();
        let px_bounds = outline.px_bounds();
        let mut mip_data = MipmapData::allocate(
            px_bounds.width() as u32,
            px_bounds.height() as u32,
            [0xFF, 0xFF, 0xFF, 0x00],
        );
        outline.draw(|x, y, luma| {
            log::info!("{}x{}: {}", x, y, luma);
            mip_data.write_pixel(
                x,
                y,
                [0xFF, 0xFF, 0xFF, (luma * 0xFF as f32) as u8],
            );
        });

        let graphics2 = Graphics2::new(
            &msaa_renderpass,
            &[
                asset_loader.blank_white()?,
                asset_loader.read_texture("assets/texture_orientation.png")?,
                asset_loader.create_texture_with_data(&[mip_data])?,
            ],
            vk_alloc.clone(),
            vk_dev.clone(),
        )?;

        Ok(Self {
            msaa_renderpass,
            framebuffers,
            graphics2,
            camera: nalgebra::Matrix4::identity(),
            _asset_loader: asset_loader,
            glyph,
            font,
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
        self.graphics2
            .rebuild_swapchain_resources(&self.msaa_renderpass)?;

        let aspect = framebuffer_size.0 as f32 / framebuffer_size.1 as f32;
        let height = 200.0;
        let width = height * aspect;
        let half_width = width / 2.0;
        let half_height = height / 2.0;

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
            .graphics2
            .acquire_frame(index)
            .with_context(|| "unable to acquire graphics2 frame")?;
        frame.set_view_projection(self.camera)?;

        // draw grid
        frame.draw_line(LineArgs {
            start: Vec2::new(0.0, 0.0),
            end: Vec2::new(100.0, 0.0),
            width: 0.2,
            ..Default::default()
        })?;

        frame.draw_line(LineArgs {
            start: Vec2::new(0.0, 0.0),
            end: Vec2::new(0.0, 100.0),
            width: 0.2,
            ..Default::default()
        })?;

        let outline = self.font.outline_glyph(self.glyph.clone()).unwrap();
        let bounds = outline.px_bounds();
        //log::info!(
        //    "top:{} left:{} bottom:{} right:{}",
        //    bounds.min.y,
        //    bounds.min.x,
        //    bounds.max.y,
        //    bounds.max.x
        //);

        // Render the letter
        Quad {
            model: Rect::new(
                -bounds.min.y,
                bounds.min.x,
                -bounds.max.y,
                bounds.max.x,
            ),
            ..Default::default()
        }
        .draw(&mut frame, Vec4::new(1.0, 1.0, 1.0, 1.0), 2)?;

        let gbounds = self.font.glyph_bounds(&self.glyph);
        frame.draw_line(LineArgs {
            start: Vec2::new(gbounds.min.x, -gbounds.min.y),
            end: Vec2::new(gbounds.min.x, -gbounds.max.y),
            ..Default::default()
        })?;
        frame.draw_line(LineArgs {
            start: Vec2::new(gbounds.max.x, -gbounds.min.y),
            end: Vec2::new(gbounds.max.x, -gbounds.max.y),
            ..Default::default()
        })?;
        frame.draw_line(LineArgs {
            start: Vec2::new(gbounds.min.x, -gbounds.min.y),
            end: Vec2::new(gbounds.max.x, -gbounds.min.y),
            ..Default::default()
        })?;
        frame.draw_line(LineArgs {
            start: Vec2::new(gbounds.min.x, -gbounds.max.y),
            end: Vec2::new(gbounds.max.x, -gbounds.max.y),
            ..Default::default()
        })?;

        unsafe {
            self.graphics2.complete_frame(cmds, frame, index)?;
            self.msaa_renderpass.end_renderpass(cmds);
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    run_application::<Example>()
}
