use crate::{ui::primitives::Rect, vec2, Vec2};

/// The Dimensions of something on screen.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Dimensions {
    pub width: f32,
    pub height: f32,
}

impl Dimensions {
    /// Create a new dimensions object.
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }

    /// Take the minimum values of the two dimensions.
    #[inline]
    pub fn min(&self, other: &Self) -> Self {
        Self::new(self.width.min(other.width), self.height.min(other.height))
    }

    /// Take the maximum values of the two dimensions.
    #[inline]
    pub fn max(&self, other: &Self) -> Self {
        Self::new(self.width.max(other.width), self.height.max(other.height))
    }

    /// Create a rect with top-left at (0, 0) and dimensions matching these
    /// dimensions.
    pub fn as_rect(&self) -> Rect {
        Rect::new(0.0, 0.0, self.height, self.width)
    }
}

impl Into<Dimensions> for Vec2 {
    fn into(self) -> Dimensions {
        Dimensions {
            width: self.x,
            height: self.y,
        }
    }
}

impl Into<Vec2> for Dimensions {
    fn into(self) -> Vec2 {
        vec2(self.width, self.height)
    }
}

impl Into<Dimensions> for (i32, i32) {
    fn into(self) -> Dimensions {
        Dimensions {
            width: self.0 as f32,
            height: self.1 as f32,
        }
    }
}

impl Into<Dimensions> for (f32, f32) {
    fn into(self) -> Dimensions {
        Dimensions {
            width: self.0,
            height: self.1,
        }
    }
}
