use crate::{vec2, Vec2};

/// Define a rectangular region on the screen.
#[derive(Debug, Copy, Clone)]
pub struct Rect {
    pub top_left: Vec2,
    pub bottom_right: Vec2,
}

impl Rect {
    /// Create a new rectange with the given coordinates.
    pub fn new(top: f32, left: f32, bottom: f32, right: f32) -> Self {
        Self {
            top_left: vec2(left, top),
            bottom_right: vec2(right, bottom),
        }
    }

    /// Create a new rectangle centered at the given position with the given
    /// width and height.
    pub fn centered_at(x: f32, y: f32, width: f32, height: f32) -> Self {
        let half_height = 0.5 * height;
        let half_width = 0.5 * width;
        Self::new(
            y + half_height,
            x - half_width,
            y - half_height,
            x + half_width,
        )
    }

    /// The leftmost extent of the Rectangle.
    pub fn left(&self) -> f32 {
        self.top_left.x
    }

    /// The rightmost extent of the Rectangle.
    pub fn right(&self) -> f32 {
        self.bottom_right.x
    }

    /// The top of the rectangle
    pub fn top(&self) -> f32 {
        self.top_left.y
    }

    pub fn bottom(&self) -> f32 {
        self.bottom_right.y
    }

    /// The Width of the rectangle, always positive.
    pub fn width(&self) -> f32 {
        (self.left() - self.right()).abs()
    }

    /// The height of the rectangle, always positive.
    pub fn height(&self) -> f32 {
        (self.top() - self.bottom()).abs()
    }

    /// Translate this rect by the given offset.
    pub fn translate(&mut self, offset: Vec2) {
        self.top_left += offset;
        self.bottom_right += offset;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let rect = Rect::new(10.0, -9.0, -10.0, 9.0);

        assert_eq!(rect.left(), rect.top_left.x);
        assert_eq!(rect.right(), rect.bottom_right.x);
        assert_eq!(rect.top(), rect.top_left.y);
        assert_eq!(rect.bottom(), rect.bottom_right.y);
    }

    #[test]
    fn test_width_and_height() {
        let rect = Rect::new(10.0, -9.0, -10.0, 9.0);
        assert_eq!(rect.width(), 18.0);
        assert_eq!(rect.height(), 20.0);
    }

    #[test]
    fn test_width_and_height_abs() {
        let rect = Rect::new(-10.0, 9.0, 10.0, -9.0);
        assert_eq!(rect.width(), 18.0);
        assert_eq!(rect.height(), 20.0);
    }

    #[test]
    fn test_translate() {
        let mut rect = Rect::new(10.0, -9.0, -10.0, 9.0);
        rect.translate(vec2(9.0, 10.0));

        assert_eq!(rect.left(), 0.0);
        assert_eq!(rect.right(), 18.0);
        assert_eq!(rect.top(), 20.0);
        assert_eq!(rect.bottom(), 0.0);

        assert_eq!(rect.width(), 18.0);
        assert_eq!(rect.height(), 20.0);
    }
}
