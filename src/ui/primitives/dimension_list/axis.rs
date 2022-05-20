use crate::{ui::primitives::Dimensions, vec2, Vec2};

/// Used to represent a layout axis for a list of dimensions.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Axis {
    Horizontal,
    Vertical,
}

impl Axis {
    /// Add the given dimensions along the current axis.
    ///
    /// Horizontal - Keep the original height and sum the widths.
    /// Vertical - Keep the original width and sum the heights.
    ///
    pub(super) fn sum(
        &self,
        original: &Dimensions,
        to_add: &Dimensions,
    ) -> Dimensions {
        match *self {
            Axis::Horizontal => {
                Dimensions::new(original.width + to_add.width, original.height)
            }
            Axis::Vertical => {
                Dimensions::new(original.width, original.height + to_add.height)
            }
        }
    }

    /// Sub the given dimensions along the current axis.
    ///
    /// Horizontal - Keep the original height and subtract the widths.
    /// Vertical - Keep the original width and subtract the heights.
    ///
    pub(super) fn sub(
        &self,
        original: &Dimensions,
        to_sub: &Dimensions,
    ) -> Dimensions {
        match *self {
            Axis::Horizontal => Dimensions::new(
                (original.width - to_sub.width).abs(),
                original.height,
            ),
            Axis::Vertical => Dimensions::new(
                original.width,
                (original.height - to_sub.height).abs(),
            ),
        }
    }

    /// Add a float value to the dimension along the current axis.
    ///
    /// Horizontal - Keep the original height and add the value to the width
    /// Vertical - Keep the original width and aff the value to the height
    ///
    pub(super) fn add_scalar(
        &self,
        original: &Dimensions,
        to_add: f32,
    ) -> Dimensions {
        match *self {
            Axis::Horizontal => {
                Dimensions::new(original.width + to_add, original.height)
            }
            Axis::Vertical => {
                Dimensions::new(original.width, original.height + to_add)
            }
        }
    }

    /// Generate new dimensions which use the maximum value for the current Axis
    /// and the original value for the other axis.
    ///
    /// Horizontal - Keep the original height and take the max width.
    /// Vertical - Keep the original width and take the max height.
    ///
    pub(super) fn max(
        &self,
        original: &Dimensions,
        to_compare: &Dimensions,
    ) -> Dimensions {
        match *self {
            Axis::Horizontal => Dimensions::new(
                original.width.max(to_compare.width),
                original.height,
            ),
            Axis::Vertical => Dimensions::new(
                original.width,
                original.height.max(to_compare.height),
            ),
        }
    }

    /// Generate new dimensions which use the minimum value for the current Axis
    /// and the original value for the other axis.
    ///
    /// Horizontal - Keep the original height and take the min width.
    /// Vertical - Keep the original width and take the min height.
    ///
    pub(super) fn min(
        &self,
        original: &Dimensions,
        to_compare: &Dimensions,
    ) -> Dimensions {
        match *self {
            Axis::Horizontal => Dimensions::new(
                original.width.min(to_compare.width),
                original.height,
            ),
            Axis::Vertical => Dimensions::new(
                original.width,
                original.height.min(to_compare.height),
            ),
        }
    }

    /// Get the value corresponding to the main axis.
    ///
    /// Horizontal - the width
    /// Vertical - the height
    ///
    pub(super) fn get(&self, dimensions: &Dimensions) -> f32 {
        match *self {
            Axis::Horizontal => dimensions.width,
            Axis::Vertical => dimensions.height,
        }
    }

    /// Create a vector which points in the direction of the current axis.
    ///
    /// Horizontal - vec2(value, 0.0)
    /// Vertical - vec2(0.0, value)
    ///
    pub(super) fn vec2(&self, value: f32) -> Vec2 {
        match *self {
            Axis::Horizontal => vec2(value, 0.0),
            Axis::Vertical => vec2(0.0, value),
        }
    }
}
