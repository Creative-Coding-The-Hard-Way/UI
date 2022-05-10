use crate::{builder_field, ui::primitives::Rect, vec4, Vec4};

/// Margin is part of the box model. Margin can be used to generate a new
/// Rect based on an existing Rect. The new Rect shinks in from the existing
/// one.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Margin {
    pub left: f32,
    pub right: f32,
    pub bottom: f32,
    pub top: f32,
}

impl Margin {
    /// Create a new margin instance with zeros for all sides.
    pub fn zero() -> Self {
        Default::default()
    }

    /// Create a new margin instance with the given value for all side.
    pub fn all(margin: f32) -> Self {
        Self {
            top: margin,
            left: margin,
            bottom: margin,
            right: margin,
        }
    }

    builder_field!(left, f32);
    builder_field!(right, f32);
    builder_field!(bottom, f32);
    builder_field!(top, f32);

    /// Construct a new Rect by applying margin inside. Generally, this
    /// *shrinks* the Rect.
    pub fn apply(&self, rect: Rect) -> Rect {
        Rect::new(
            rect.top() + self.top,
            rect.left() + self.left,
            rect.bottom() - self.bottom,
            rect.right() - self.right,
        )
    }
}

impl Into<Vec4> for Margin {
    /// Turn this margin into a vec4 with values:
    ///
    /// vec4(left, top, right, bottom)
    ///
    fn into(self) -> Vec4 {
        vec4(self.left, self.top, self.right, self.bottom)
    }
}

impl Into<Margin> for Vec4 {
    /// Turn this margin into a vec4 with values:
    ///
    /// vec4(left, top, right, bottom)
    ///
    fn into(self) -> Margin {
        Margin {
            left: self.x,
            top: self.y,
            bottom: self.w,
            right: self.z,
        }
    }
}

impl Default for Margin {
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
        let margin = Margin::zero().top(1.0).left(2.0).bottom(3.0).right(4.0);
        let rect = Rect::centered_at(0.0, 0.0, 20.0, 20.0);
        let with_margin = margin.apply(rect);

        assert_eq!(with_margin, Rect::new(-9.0, -8.0, 7.0, 6.0));
    }

    #[test]
    fn test_all_same() {
        let margin = 10.0;
        let all_same = Margin::all(10.0);

        assert_eq!(all_same.top, margin);
        assert_eq!(all_same.left, margin);
        assert_eq!(all_same.bottom, margin);
        assert_eq!(all_same.right, margin);
    }

    #[test]
    fn test_zero() {
        let all_zero = Margin::zero();

        assert_eq!(all_zero.top, 0.0);
        assert_eq!(all_zero.left, 0.0);
        assert_eq!(all_zero.bottom, 0.0);
        assert_eq!(all_zero.right, 0.0);
    }

    #[test]
    fn test_left() {
        let pad_left = 10.0;
        let margin_struct = Margin::zero().left(pad_left);

        assert_eq!(margin_struct.top, 0.0);
        assert_eq!(margin_struct.left, pad_left);
        assert_eq!(margin_struct.bottom, 0.0);
        assert_eq!(margin_struct.right, 0.0);
    }

    #[test]
    fn test_right() {
        let pad_right = 10.0;
        let margin_struct = Margin::zero().right(pad_right);

        assert_eq!(margin_struct.top, 0.0);
        assert_eq!(margin_struct.left, 0.0);
        assert_eq!(margin_struct.bottom, 0.0);
        assert_eq!(margin_struct.right, pad_right);
    }

    #[test]
    fn test_top() {
        let pad_top = 10.0;
        let margin_struct = Margin::zero().top(pad_top);

        assert_eq!(margin_struct.top, pad_top);
        assert_eq!(margin_struct.left, 0.0);
        assert_eq!(margin_struct.bottom, 0.0);
        assert_eq!(margin_struct.right, 0.0);
    }

    #[test]
    fn test_bottom() {
        let pad_bottom = 10.0;
        let margin_struct = Margin::zero().bottom(pad_bottom);

        assert_eq!(margin_struct.top, 0.0);
        assert_eq!(margin_struct.left, 0.0);
        assert_eq!(margin_struct.bottom, pad_bottom);
        assert_eq!(margin_struct.right, 0.0);
    }
}
