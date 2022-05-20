/// Constraints can be applied to the width and height of containers to limit
/// their size.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Constraint {
    /// Represents an absolute maximum size for a given dimension. The contained
    /// element will never grow larger than this value.
    FixedMaxSize(f32),

    /// Represents a maximum size which is a percentage of the available space.
    /// 50% means that this container will never occupy more than 50% of the
    /// available space in this dimenions. Values are in the range (0-1].
    PercentMaxSize(f32),

    /// Represents no additional constraints on the maximum size. The container
    /// is allowed to grow or shrink within it's avialable space.
    NoConstraint,
}

impl Default for Constraint {
    /// Constraints default to no constraint.
    fn default() -> Self {
        Self::NoConstraint
    }
}

impl Constraint {
    /// Apply the constraint to a given value and return the result.
    pub(super) fn apply(&self, value: f32) -> f32 {
        match *self {
            Constraint::FixedMaxSize(max) => value.min(max),
            Constraint::PercentMaxSize(percentage) => value * percentage,
            Constraint::NoConstraint => value,
        }
    }
}
