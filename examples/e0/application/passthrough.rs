use ::{
    ash::vk,
    ccthw::vulkan::{
        errors::VulkanError, DescriptorSetLayout, Pipeline, PipelineLayout,
        RenderDevice, RenderPass, ShaderModule,
    },
    std::sync::Arc,
};

pub struct Passthrough {
    pub pipeline: Pipeline,
}

impl Passthrough {
    pub fn new(
        extent: vk::Extent2D,
        render_pass: Arc<RenderPass>,
        samples: vk::SampleCountFlags,
        vk_dev: Arc<RenderDevice>,
    ) -> Result<Self, VulkanError> {
        let vertex_module = ShaderModule::from_spirv(
            vk_dev.clone(),
            std::include_bytes!("./shaders/passthrough.vert.spirv"),
        )?;
        let fragment_module = ShaderModule::from_spirv(
            vk_dev.clone(),
            std::include_bytes!("./shaders/passthrough.frag.spirv"),
        )?;
        let vertex_input_state = vk::PipelineVertexInputStateCreateInfo {
            ..Default::default()
        };
        let input_assembly = vk::PipelineInputAssemblyStateCreateInfo {
            topology: vk::PrimitiveTopology::TRIANGLE_LIST,
            primitive_restart_enable: 0,
            ..Default::default()
        };
        let viewport = vk::Viewport {
            x: 0.0,
            y: 0.0,
            width: extent.width as f32,
            height: extent.height as f32,
            min_depth: 0.0,
            max_depth: 1.0,
        };
        let scissors = vk::Rect2D {
            offset: vk::Offset2D { x: 0, y: 0 },
            extent,
        };
        let viewport_state = vk::PipelineViewportStateCreateInfo {
            p_viewports: &viewport,
            viewport_count: 1,
            p_scissors: &scissors,
            scissor_count: 1,
            ..Default::default()
        };
        let raster_state = vk::PipelineRasterizationStateCreateInfo {
            depth_clamp_enable: 0,
            rasterizer_discard_enable: 0,
            polygon_mode: vk::PolygonMode::FILL,
            line_width: 1.0,
            cull_mode: vk::CullModeFlags::NONE,
            front_face: vk::FrontFace::CLOCKWISE,
            ..Default::default()
        };
        let multisample_state = vk::PipelineMultisampleStateCreateInfo {
            sample_shading_enable: 0,
            rasterization_samples: samples,
            p_sample_mask: std::ptr::null(),
            min_sample_shading: 1.0,
            ..Default::default()
        };
        let blend_attachment = vk::PipelineColorBlendAttachmentState {
            color_write_mask: vk::ColorComponentFlags::R
                | vk::ColorComponentFlags::G
                | vk::ColorComponentFlags::B
                | vk::ColorComponentFlags::A,
            blend_enable: 1,
            src_color_blend_factor: vk::BlendFactor::SRC_ALPHA,
            dst_color_blend_factor: vk::BlendFactor::ONE_MINUS_SRC_ALPHA,
            color_blend_op: vk::BlendOp::ADD,
            src_alpha_blend_factor: vk::BlendFactor::ONE,
            dst_alpha_blend_factor: vk::BlendFactor::ZERO,
            alpha_blend_op: vk::BlendOp::ADD,
        };
        let blend_state = vk::PipelineColorBlendStateCreateInfo {
            p_attachments: &blend_attachment,
            attachment_count: 1,
            ..Default::default()
        };
        let stages = [
            vertex_module.stage_create_info(vk::ShaderStageFlags::VERTEX),
            fragment_module.stage_create_info(vk::ShaderStageFlags::FRAGMENT),
        ];
        let descriptor_layout = Arc::new(DescriptorSetLayout::new(
            vk_dev.clone(),
            &[vk::DescriptorSetLayoutBinding {
                binding: 0,
                descriptor_type: vk::DescriptorType::STORAGE_BUFFER,
                descriptor_count: 1,
                stage_flags: vk::ShaderStageFlags::VERTEX,
                ..Default::default()
            }],
        )?);
        let pipeline_layout = Arc::new(PipelineLayout::new(
            vk_dev.clone(),
            &[descriptor_layout],
            &[],
        )?);
        let pipeline_create_info = vk::GraphicsPipelineCreateInfo {
            p_stages: stages.as_ptr(),
            stage_count: stages.len() as u32,
            p_vertex_input_state: &vertex_input_state,
            p_input_assembly_state: &input_assembly,
            p_viewport_state: &viewport_state,
            p_rasterization_state: &raster_state,
            p_multisample_state: &multisample_state,
            p_color_blend_state: &blend_state,
            render_pass: render_pass.raw,
            layout: pipeline_layout.raw,
            ..Default::default()
        };
        let pipeline = Pipeline::new_graphics_pipeline(
            pipeline_create_info,
            render_pass,
            pipeline_layout,
            vk_dev.clone(),
        )?;
        Ok(Self { pipeline })
    }
}
