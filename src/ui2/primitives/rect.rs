use crate::{
    ui2::{Bounds, Dimensions},
    vec2, Vec2,
};

/// Define a rectangular region on the screen.
/// Values assume a coordinate system where (0,0) is the top left corner of the
/// screen and (width,height) is the bottom right corner of the screen.
///
/// E.g. positive X points to the right, and positive Y points down.
///
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
            y - half_height,
            x - half_width,
            y + half_height,
            x + half_width,
        )
    }

    /// The leftmost extent of the Rectangle.
    #[inline]
    pub fn left(&self) -> f32 {
        self.top_left.x
    }

    /// The rightmost extent of the Rectangle.
    #[inline]
    pub fn right(&self) -> f32 {
        self.bottom_right.x
    }

    /// The top of the rectangle
    #[inline]
    pub fn top(&self) -> f32 {
        self.top_left.y
    }

    /// The bottom of the rectangle
    #[inline]
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

    pub fn dimensions(&self) -> Dimensions {
        (self.width(), self.height()).into()
    }

    /// Translate this rect by the given offset.
    pub fn translate(&self, offset: Vec2) -> Self {
        Self {
            top_left: self.top_left + offset,
            bottom_right: self.bottom_right + offset,
        }
    }

    /// Returns true if the given point is inside the current rectangular
    /// region.
    pub fn contains(&self, point: Vec2) -> bool {
        let horizontal = self.left() <= point.x && point.x <= self.right();
        let vertical = self.top() <= point.y && point.y <= self.bottom();
        horizontal && vertical
    }

    /// Create a new rect which fully contains both self and the provided rect.
    pub fn expand(&self, other: Rect) -> Self {
        Self {
            top_left: vec2(
                self.left().min(other.left()),
                self.top().min(other.top()),
            ),
            bottom_right: vec2(
                self.right().max(other.right()),
                self.bottom().max(other.bottom()),
            ),
        }
    }
}

impl Bounds for Rect {
    /// The bounding box for any given rectangular region is simply itself.
    #[inline]
    fn bounds(&self) -> Rect {
        *self
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
        let rect = Rect::new(10.0, -9.0, -10.0, 9.0).translate(vec2(9.0, 10.0));

        assert_eq!(rect.left(), 0.0);
        assert_eq!(rect.right(), 18.0);
        assert_eq!(rect.top(), 20.0);
        assert_eq!(rect.bottom(), 0.0);

        assert_eq!(rect.width(), 18.0);
        assert_eq!(rect.height(), 20.0);
    }

    #[test]
    fn test_contains() {
        let rect = Rect::centered_at(0.0, 0.0, 10.0, 10.0);

        // check a point inside
        assert!(rect.contains(vec2(0.0, 0.0)));

        // check the corners
        assert!(rect.contains(vec2(-5.0, -5.0)), "msg");
        assert!(rect.contains(vec2(5.0, -5.0)));
        assert!(rect.contains(vec2(-5.0, 5.0)));
        assert!(rect.contains(vec2(5.0, 5.0)));

        // check outside the rect
        assert!(!rect.contains(vec2(-6.0, 0.0)));
        assert!(!rect.contains(vec2(6.0, 0.0)));
        assert!(!rect.contains(vec2(0.0, 6.0)));
        assert!(!rect.contains(vec2(0.0, -6.0)));
    }

    #[test]
    fn test_expand() {
        let rect = Rect::new(0.0, 0.0, 10.0, 10.0);
        let other = Rect::new(20.0, 2.0, 23.0, 5.0);
        let expanded = rect.expand(other);

        assert_eq!(expanded.top(), 0.0);
        assert_eq!(expanded.left(), 0.0);
        assert_eq!(expanded.right(), 10.0);
        assert_eq!(expanded.bottom(), 23.0);
    }
}
