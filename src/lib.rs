mod markdown;

pub mod asset_loader;
pub mod demo;
pub mod frame_pipeline;
pub mod glfw_window;
pub mod graphics2;
pub mod math;
pub mod multisample_renderpass;
pub mod timing;
pub mod ui;
pub mod vulkan;
pub mod vulkan_ext;

pub type Vec2 = nalgebra::Vector2<f32>;
pub type Vec3 = nalgebra::Vector3<f32>;
pub type Vec4 = nalgebra::Vector4<f32>;
