use crate::{builder_field, ui::primitives::Dimensions, vec2, Vec2};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Justify {
    Begin,
    Center,
    End,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum SpaceBetween {
    /// Put a fixed amount of space.
    Fixed(f32),

    /// Add space between elements such that they're evenly spaced.
    /// This will make the first and last elements touch the edge of the
    /// available space.
    EvenSpaceBetween,

    /// Add space around elements such that thei're evenly spaced.
    /// This will put even space around teh first and last elements.
    EvenSpaceAround,
}

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
    fn add(&self, original: &Dimensions, to_add: &Dimensions) -> Dimensions {
        match *self {
            Axis::Horizontal => {
                Dimensions::new(original.width + to_add.width, original.height)
            }
            Axis::Vertical => {
                Dimensions::new(original.width, original.height + to_add.height)
            }
        }
    }

    /// Generate new dimensions which use the maximum value for the current Axis
    /// and the original value for the other axis.
    ///
    /// Horizontal - Keep the original height and take the max width.
    /// Vertical - Keep the original width and take the max height.
    ///
    fn max(
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

    /// Get the value corresponding to the main axis.
    ///
    /// Horizontal - the width
    /// Vertical - the height
    ///
    fn get(&self, dimensions: &Dimensions) -> f32 {
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
    fn vec2(&self, value: f32) -> Vec2 {
        match *self {
            Axis::Horizontal => vec2(value, 0.0),
            Axis::Vertical => vec2(0.0, value),
        }
    }
}

/// This type represents a collection of objects with dimensions.
/// It provides methods for computing layout positions for each child given
/// size constraints and layout parameters.
pub struct DimensionList {
    main_axis: Axis,
    off_axis: Axis,
    children: Vec<(Dimensions, Justify)>,
    total_children_size: Dimensions,
    max_size: Dimensions,
    space_between: SpaceBetween,
}

impl DimensionList {
    pub fn new(main_axis: Axis, off_axsis: Axis) -> Self {
        Self {
            main_axis,
            off_axis: off_axsis,
            total_children_size: Dimensions::new(0.0, 0.0),
            max_size: Dimensions::new(0.0, 0.0),
            children: Vec::new(),
            space_between: SpaceBetween::Fixed(0.0),
        }
    }

    pub fn horizontal() -> Self {
        Self::new(Axis::Horizontal, Axis::Vertical)
    }

    pub fn vertical() -> Self {
        Self::new(Axis::Vertical, Axis::Horizontal)
    }

    builder_field!(space_between, SpaceBetween);

    /// Set the maximum size for the dimension list.
    /// This is used when computing layouts which favor the space between
    /// elements.
    pub fn set_max_size(&mut self, max_size: &Dimensions) {
        self.max_size = *max_size;
    }

    /// Add a child to the list. The total internal dimensions are updated
    /// automatically. The remaining space within the max_size is returned and
    /// can be used when calling dimensions for subsequent child elements.
    pub fn add_child_dimensions(
        &mut self,
        child_dimensions: Dimensions,
        justify: Justify,
    ) {
        self.children.push((child_dimensions, justify));

        // Add the child's width or height to the total size based on the
        // current main axis
        self.total_children_size = self
            .main_axis
            .add(&self.total_children_size, &child_dimensions);

        // Set the width or height to the max of the current value and the
        // child's current value.
        self.total_children_size = self
            .off_axis
            .max(&self.total_children_size, &child_dimensions);
    }

    /// Compute positions - relative to 0,0 in the top left - for each child
    /// element's top left corner.
    pub fn compute_child_positions(&self) -> Vec<Vec2> {
        let main_axis_offset = match self.space_between {
            SpaceBetween::Fixed(size) => self.main_axis.vec2(size),
            SpaceBetween::EvenSpaceBetween => {
                let remaining_size = self.main_axis.get(&self.max_size)
                    - self.main_axis.get(&self.total_children_size);
                let offset =
                    remaining_size / (self.children.len() - 1).max(1) as f32;
                self.main_axis.vec2(offset)
            }
            SpaceBetween::EvenSpaceAround => {
                let remaining_size = self.main_axis.get(&self.max_size)
                    - self.main_axis.get(&self.total_children_size);
                let offset = remaining_size / (self.children.len() + 1) as f32;
                self.main_axis.vec2(offset)
            }
        };

        let mut position = match self.space_between {
            SpaceBetween::EvenSpaceAround => main_axis_offset,
            _ => vec2(0.0, 0.0),
        };

        let mut child_positions = Vec::with_capacity(self.children.len());
        for (child, justify) in &self.children {
            let off_axis_remaining_size =
                self.off_axis.get(&self.total_children_size)
                    - self.off_axis.get(child);
            let off_axis_offset = match *justify {
                Justify::Begin => self.off_axis.vec2(0.0),
                Justify::End => self.off_axis.vec2(off_axis_remaining_size),
                Justify::Center => {
                    self.off_axis.vec2(0.5 * off_axis_remaining_size)
                }
            };

            child_positions.push(position + off_axis_offset);

            position += main_axis_offset
                + self.main_axis.vec2(self.main_axis.get(child));
        }

        child_positions
    }
}
