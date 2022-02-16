/// Instances of this struct represent mipmap data on the CPU.
/// Data is always assumed to be in R8G8B8A8_SRGB format, e.g. four u8's per
/// pixel.
pub struct MipmapData {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}
