use ::{
    anyhow::{Context, Result},
    ccthw::{
        asset_loader::AssetLoader,
        demo::{run_application, State},
        glfw_window::GlfwWindow,
        graphics2::{Graphics2, Text},
        math::projections,
        multisample_renderpass::MultisampleRenderpass,
        timing::FrameRateLimit,
        vulkan::{Framebuffer, MemoryAllocator, RenderDevice},
        Vec2,
    },
    std::sync::Arc,
};

struct Example {
    msaa_renderpass: MultisampleRenderpass,
    framebuffers: Vec<Framebuffer>,
    graphics2: Graphics2,
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
        let graphics2 = Graphics2::new(
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
            graphics2,
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
        self.graphics2
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
            .graphics2
            .acquire_frame(index)
            .with_context(|| "unable to acquire graphics2 frame")?;
        frame.set_view_projection(self.camera)?;
        //frame.draw_line(LineArgs {
        //    start: Vec2::new(-10000.0, 0.0),
        //    end: Vec2::new(10000.0, 0.0),
        //    ..Default::default()
        //})?;
        //frame.draw_line(LineArgs {
        //    start: Vec2::new(0.0, 10000.0),
        //    end: Vec2::new(0.0, -10000.0),
        //    ..Default::default()
        //})?;
        //frame.draw_line(LineArgs {
        //    start: Vec2::new(0.0, -100.0),
        //    end: Vec2::new(2000.0, -100.0),
        //    ..Default::default()
        //})?;
        self.text.draw_text(
            &mut frame,
            Vec2::new(20.0, -100.0),
            &"[]{}\n{hello world}(thing)",
        )?;

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
