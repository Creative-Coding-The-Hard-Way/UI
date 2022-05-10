use crate::{builder_field, ui::primitives::Rect, vec4, Vec4};

/// Padding is part of the box model. Padding can be used to generate a new
/// Rect based on an existing Rect. The new Rect grows out from the existing
/// one.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Padding {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
}

impl Padding {
    /// Create a new padding instance with zeros for all sides.
    pub fn zero() -> Self {
        Default::default()
    }

    /// Create a new padding instance with the given value for all side.
    pub fn all(padding: f32) -> Self {
        Self {
            top: padding,
            left: padding,
            bottom: padding,
            right: padding,
        }
    }

    builder_field!(left, f32);
    builder_field!(right, f32);
    builder_field!(bottom, f32);
    builder_field!(top, f32);

    /// Construct a new Rect by applying padding outside. Generally, this
    /// *grows* the Rect.
    pub fn apply(&self, rect: Rect) -> Rect {
        Rect::new(
            rect.top() - self.top,
            rect.left() - self.left,
            rect.bottom() + self.bottom,
            rect.right() + self.right,
        )
    }
}

impl Into<Vec4> for Padding {
    /// Turn this padding into a vec4 with values:
    ///
    /// vec4(left, top, right, bottom)
    ///
    fn into(self) -> Vec4 {
        vec4(self.left, self.top, self.right, self.bottom)
    }
}

impl Into<Padding> for Vec4 {
    /// Turn this padding into a vec4 with values:
    ///
    /// vec4(left, top, right, bottom)
    ///
    fn into(self) -> Padding {
        Padding {
            left: self.x,
            top: self.y,
            bottom: self.w,
            right: self.z,
        }
    }
}

impl Default for Padding {
    fn default() -> Self {
        Self {
            left: 0.0,
            right: 0.0,
            bottom: 0.0,
            top: 0.0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_apply() {
        let padding = Padding::zero().top(1.0).left(2.0).bottom(3.0).right(4.0);
        let rect = Rect::centered_at(0.0, 0.0, 20.0, 20.0);
        let padded = padding.apply(rect);

        assert_eq!(padded, Rect::new(-11.0, -12.0, 13.0, 14.0));
    }

    #[test]
    fn test_all_same() {
        let padding = 10.0;
        let all_same = Padding::all(10.0);

        assert_eq!(all_same.top, padding);
        assert_eq!(all_same.left, padding);
        assert_eq!(all_same.bottom, padding);
        assert_eq!(all_same.right, padding);
    }

    #[test]
    fn test_zero() {
        let all_zero = Padding::zero();

        assert_eq!(all_zero.top, 0.0);
        assert_eq!(all_zero.left, 0.0);
        assert_eq!(all_zero.bottom, 0.0);
        assert_eq!(all_zero.right, 0.0);
    }

    #[test]
    fn test_left() {
        let pad_left = 10.0;
        let padding_struct = Padding::zero().left(pad_left);

        assert_eq!(padding_struct.top, 0.0);
        assert_eq!(padding_struct.left, pad_left);
        assert_eq!(padding_struct.bottom, 0.0);
        assert_eq!(padding_struct.right, 0.0);
    }

    #[test]
    fn test_right() {
        let pad_right = 10.0;
        let padding_struct = Padding::zero().right(pad_right);

        assert_eq!(padding_struct.top, 0.0);
        assert_eq!(padding_struct.left, 0.0);
        assert_eq!(padding_struct.bottom, 0.0);
        assert_eq!(padding_struct.right, pad_right);
    }

    #[test]
    fn test_top() {
        let pad_top = 10.0;
        let padding_struct = Padding::zero().top(pad_top);

        assert_eq!(padding_struct.top, pad_top);
        assert_eq!(padding_struct.left, 0.0);
        assert_eq!(padding_struct.bottom, 0.0);
        assert_eq!(padding_struct.right, 0.0);
    }

    #[test]
    fn test_bottom() {
        let pad_bottom = 10.0;
        let padding_struct = Padding::zero().bottom(pad_bottom);

        assert_eq!(padding_struct.top, 0.0);
        assert_eq!(padding_struct.left, 0.0);
        assert_eq!(padding_struct.bottom, pad_bottom);
        assert_eq!(padding_struct.right, 0.0);
    }
}
