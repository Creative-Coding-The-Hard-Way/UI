use crate::ui::primitives::Dimensions;

/// Used to represent a layout axis for a list of dimensions.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Axis {
    Horizontal,
    Vertical,
}

impl Axis {
    /// Set the width or the height depending on the Axis's value.
    /// Horizontal - set the width
    /// Vertical - set the height
    fn set(&self, dimensions: Dimensions, value: f32) -> Dimensions {
        match *self {
            Axis::Horizontal => Dimensions::new(value, dimensions.height),
            Axis::Vertical => Dimensions::new(dimensions.width, value),
        }
    }

    /// Get the width or the height depending on the Axis's value.
    /// Horizonal - get the width
    /// Vertical - get the height
    fn get(&self, dimensions: Dimensions) -> f32 {
        match *self {
            Axis::Horizontal => dimensions.width,
            Axis::Vertical => dimensions.height,
        }
    }
}

/// This type represents a collection of objects with dimensions.
/// It provides methods for computing layout positions for each child given
/// size constraints and layout parameters.
pub struct DimensionList {
    main_axis: Axis,
    off_axsis: Axis,
    children: Vec<Dimensions>,
    total_children_size: Dimensions,
    max_size: Dimensions,
}

impl DimensionList {
    pub fn new(main_axis: Axis, off_axsis: Axis) -> Self {
        Self {
            main_axis,
            off_axsis,
            total_children_size: Dimensions::new(0.0, 0.0),
            max_size: Dimensions::new(0.0, 0.0),
            children: Vec::new(),
        }
    }

    pub fn horizontal() -> Self {
        Self::new(Axis::Horizontal, Axis::Vertical)
    }

    pub fn vertical() -> Self {
        Self::new(Axis::Vertical, Axis::Horizontal)
    }

    /// Set the maximum size for the dimension list.
    /// This is used when computing layouts which favor the space between
    /// elements.
    pub fn set_max_size(&mut self, max_size: &Dimensions) {
        self.max_size = *max_size;
    }

    pub fn add_child_dimensions(&mut self, child_dimensions: Dimensions) {
        self.children.push(child_dimensions);

        // Add the child's width or height to the total size based on the
        // current main axis
        self.total_children_size = self.main_axis.set(
            self.total_children_size,
            self.main_axis.get(self.total_children_size)
                + self.main_axis.get(child_dimensions),
        );

        // Set the width or height to the max of the current value and the
        // child's current value.
        self.total_children_size = self.off_axsis.set(
            self.total_children_size,
            self.off_axsis
                .get(self.total_children_size)
                .max(self.off_axsis.get(child_dimensions)),
        );
    }
}
