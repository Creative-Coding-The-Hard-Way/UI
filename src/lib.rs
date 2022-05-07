mod markdown;

pub mod asset_loader;
pub mod demo;
pub mod frame_pipeline;
pub mod glfw_window;
pub mod immediate_mode_graphics;
pub mod math;
pub mod multisample_renderpass;
pub mod timing;
pub mod ui;
pub mod ui2;
pub mod vulkan;
pub mod vulkan_ext;

pub type Mat4 = nalgebra::Matrix4<f32>;
pub type Vec2 = nalgebra::Vector2<f32>;
pub type Vec3 = nalgebra::Vector3<f32>;
pub type Vec4 = nalgebra::Vector4<f32>;

#[inline]
pub fn vec2(x: f32, y: f32) -> Vec2 {
    Vec2::new(x, y)
}

#[inline]
pub fn vec3(x: f32, y: f32, z: f32) -> Vec3 {
    Vec3::new(x, y, z)
}

#[inline]
pub fn vec4(x: f32, y: f32, z: f32, w: f32) -> Vec4 {
    Vec4::new(x, y, z, w)
}
