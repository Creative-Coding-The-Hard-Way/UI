use crate::immediate_mode_graphics::Frame;

use anyhow::Result;

/// Things which implement this trait can render themselves to a given frame.
pub trait Drawable {
    /// Render a solid view of the object.
    fn fill(&self, frame: &mut Frame) -> Result<()>;

    /// Render a wireframe view of the object.
    fn outline(&self, frame: &mut Frame) -> Result<()>;
}
