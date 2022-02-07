use std::sync::Arc;

use anyhow::Result;
use ash::vk;

use crate::vulkan::{
    errors::VulkanError, Pipeline, PipelineLayout, RenderDevice, RenderPass,
    ShaderModule,
};

pub(super) fn create_pipeline(
    vk_dev: Arc<RenderDevice>,
    render_pass: &RenderPass,
    pipeline_layout: &PipelineLayout,
) -> Result<Pipeline, VulkanError> {
    let extent = vk_dev.with_swapchain(|swapchain| swapchain.extent);
    let vertex_module = ShaderModule::from_spirv(
        vk_dev.clone(),
        std::include_bytes!("shaders/triangle.vert.sprv"),
    )?;
    let fragment_module = ShaderModule::from_spirv(
        vk_dev.clone(),
        std::include_bytes!("shaders/triangle.frag.sprv"),
    )?;
    let vertex_input_state = vk::PipelineVertexInputStateCreateInfo {
        ..Default::default()
    };
    let input_assembly = vk::PipelineInputAssemblyStateCreateInfo {
        topology: vk::PrimitiveTopology::TRIANGLE_LIST,
        primitive_restart_enable: 0,
        ..Default::default()
    };
    let viewports = [vk::Viewport {
        x: 0.0,
        y: 0.0,
        width: extent.width as f32,
        height: extent.height as f32,
        min_depth: 0.0,
        max_depth: 1.0,
    }];
    let scissors = [vk::Rect2D {
        offset: vk::Offset2D { x: 0, y: 0 },
        extent,
    }];
    let viewport_state = vk::PipelineViewportStateCreateInfo {
        p_viewports: viewports.as_ptr(),
        viewport_count: viewports.len() as u32,
        p_scissors: scissors.as_ptr(),
        scissor_count: scissors.len() as u32,
        ..Default::default()
    };
    let raster_state = vk::PipelineRasterizationStateCreateInfo {
        depth_clamp_enable: 0,
        rasterizer_discard_enable: 0,
        polygon_mode: vk::PolygonMode::FILL,
        line_width: 1.0,
        cull_mode: vk::CullModeFlags::NONE,
        front_face: vk::FrontFace::CLOCKWISE,
        depth_bias_enable: 0,
        depth_bias_constant_factor: 0.0,
        depth_bias_clamp: 0.0,
        depth_bias_slope_factor: 0.0,
        ..Default::default()
    };
    let multisample_state = vk::PipelineMultisampleStateCreateInfo {
        sample_shading_enable: 0,
        rasterization_samples: vk_dev
            .get_supported_msaa(vk::SampleCountFlags::TYPE_4),
        p_sample_mask: std::ptr::null(),
        min_sample_shading: 1.0,
        alpha_to_coverage_enable: 0,
        alpha_to_one_enable: 0,
        ..Default::default()
    };
    let blend_attachments = [vk::PipelineColorBlendAttachmentState {
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
    }];
    let blend_state = vk::PipelineColorBlendStateCreateInfo {
        logic_op_enable: 0,
        logic_op: vk::LogicOp::COPY,
        blend_constants: [0.0, 0.0, 0.0, 0.0],
        p_attachments: blend_attachments.as_ptr(),
        attachment_count: blend_attachments.len() as u32,
        ..Default::default()
    };
    let stages = [
        vertex_module.stage_create_info(vk::ShaderStageFlags::VERTEX),
        fragment_module.stage_create_info(vk::ShaderStageFlags::FRAGMENT),
    ];
    let pipeline_create_info = vk::GraphicsPipelineCreateInfo {
        p_stages: stages.as_ptr(),
        stage_count: stages.len() as u32,
        p_vertex_input_state: &vertex_input_state,
        p_input_assembly_state: &input_assembly,
        p_viewport_state: &viewport_state,
        p_rasterization_state: &raster_state,
        p_multisample_state: &multisample_state,
        p_color_blend_state: &blend_state,

        p_tessellation_state: std::ptr::null(),
        p_dynamic_state: std::ptr::null(),
        p_depth_stencil_state: std::ptr::null(),

        layout: pipeline_layout.raw,
        render_pass: render_pass.raw,
        subpass: 0,
        base_pipeline_index: -1,
        base_pipeline_handle: vk::Pipeline::null(),

        ..Default::default()
    };
    Ok(Pipeline::new_graphics_pipeline(
        vk_dev,
        pipeline_create_info,
    )?)
}