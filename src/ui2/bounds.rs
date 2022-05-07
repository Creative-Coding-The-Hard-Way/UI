use crate::ui2::primitives::Rect;

/// A type which implements this trait can compute it's own bounding region in
/// UI Screen Space.
pub trait Bounds {
    /// Compute this instance's bounding box as a UI screen space rectangle.
    fn bounds(&self) -> Rect;
}

impl<T> Bounds for &[T]
where
    T: Bounds,
{
    /// Compute the minimal bounds which contain every region in the list.
    fn bounds(&self) -> Rect {
        if self.is_empty() {
            Rect::new(0.0, 0.0, 0.0, 0.0)
        } else {
            let mut result = self[0].bounds();
            for thing in *self {
                result = result.expand(thing.bounds());
            }
            result
        }
    }
}

impl<T> Bounds for Vec<T>
where
    T: Bounds,
{
    /// Compute the minimal bounds which contain every region in the list.
    fn bounds(&self) -> Rect {
        self.as_slice().bounds()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::ui2::primitives::Rect;

    #[test]
    fn test_bounds_for_empty_slice() {
        let my_things: Vec<Rect> = vec![];

        let bounds = my_things.bounds();

        assert_eq!(bounds.top(), 0.0);
        assert_eq!(bounds.left(), 0.0);
        assert_eq!(bounds.bottom(), 0.0);
        assert_eq!(bounds.right(), 0.0);
    }

    #[test]
    fn test_bounds_for_a_single_rect() {
        let my_things: Vec<Rect> = vec![Rect::new(1.0, 2.0, 3.0, 4.0)];

        let bounds = my_things.bounds();

        assert_eq!(bounds.top(), 1.0);
        assert_eq!(bounds.left(), 2.0);
        assert_eq!(bounds.bottom(), 3.0);
        assert_eq!(bounds.right(), 4.0);
    }

    #[test]
    fn test_bounds_for_multiple_rects() {
        let my_things: Vec<Rect> = vec![
            Rect::new(1.0, 2.0, 3.0, 4.0),
            Rect::new(-4.0, -3.0, -2.0, -1.0),
        ];

        let bounds = my_things.bounds();

        assert_eq!(bounds.top(), -4.0);
        assert_eq!(bounds.left(), -3.0);
        assert_eq!(bounds.bottom(), 3.0);
        assert_eq!(bounds.right(), 4.0);
    }
}
