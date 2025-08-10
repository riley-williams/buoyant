use crate::primitives::{Interpolate, Point, Size};

/// The default transition used for all views.
pub static DEFAULT_TRANSITION: Opacity = Opacity::new();

/// Frame and opacity transforms to apply when a view is added or removed.
///
/// Among others, [`if_view!`], [`match_view!`], and [`ForEach`] cause their
/// subviews to be transitioned.
pub trait Transition: Clone {
    /// The offset of the transitioning view at the given animation factor.
    fn transform(&self, direction: Direction, factor: u8, bounds: Size) -> Point;

    /// The opacity of the transitioning view at the given animation factor.
    fn opacity(&self, direction: Direction, factor: u8) -> u8;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    In,
    Out,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Opacity;

impl Opacity {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Transition for Opacity {
    fn transform(&self, _direction: Direction, _factor: u8, _bounds: Size) -> Point {
        Point::zero()
    }

    fn opacity(&self, direction: Direction, factor: u8) -> u8 {
        if direction == Direction::In {
            factor
        } else {
            255 - factor
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Edge {
    Top,
    Bottom,
    Leading,
    Trailing,
}

/// A symmetric transition that moves the view to the specified edge
/// when transitioning in, and back to the same edge when transitioning out.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Move {
    pub edge: Edge,
}

impl Move {
    #[must_use]
    pub const fn new(edge: Edge) -> Self {
        Self { edge }
    }

    /// Constructs a `Move` transition that transitions in and out from the leading edge.
    #[must_use]
    pub const fn leading() -> Self {
        Self::new(Edge::Leading)
    }

    /// Constructs a `Move` transition that transitions in and out from the trailing edge.
    #[must_use]
    pub const fn trailing() -> Self {
        Self::new(Edge::Trailing)
    }

    /// Constructs a `Move` transition that transitions in and out from the top edge.
    #[must_use]
    pub const fn top() -> Self {
        Self::new(Edge::Top)
    }

    /// Constructs a `Move` transition that transitions in and out from the bottom edge.
    #[must_use]
    pub const fn bottom() -> Self {
        Self::new(Edge::Bottom)
    }
}

impl Transition for Move {
    fn transform(&self, direction: Direction, factor: u8, bounds: Size) -> Point {
        let transform = match self.edge {
            Edge::Top => Point::new(0, -(bounds.height as i32)),
            Edge::Bottom => Point::new(0, bounds.height as i32),
            Edge::Leading => Point::new(-(bounds.width as i32), 0),
            Edge::Trailing => Point::new(bounds.width as i32, 0),
        };

        if direction == Direction::In {
            Interpolate::interpolate(transform, Point::zero(), factor)
        } else {
            Interpolate::interpolate(Point::zero(), transform, factor)
        }
    }

    fn opacity(&self, _direction: Direction, _factor: u8) -> u8 {
        255
    }
}

/// An asymmetric transition that moves the view to the specified edge
/// when transitioning in, and to the opposite edge when transitioning out.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Slide {
    pub edge: Edge,
}

impl Slide {
    #[must_use]
    pub const fn new(edge: Edge) -> Self {
        Self { edge }
    }

    /// Constructs a `Slide` transition that transitions in from the leading edge
    /// and out to the trailing edge.
    #[must_use]
    pub const fn leading() -> Self {
        Self::new(Edge::Leading)
    }

    /// Constructs a `Slide` transition that transitions in from the trailing edge
    /// and out to the leading edge.
    #[must_use]
    pub const fn trailing() -> Self {
        Self::new(Edge::Trailing)
    }

    /// Constructs a `Slide` transition that transitions in from the top edge
    /// and out to the bottom edge.
    #[must_use]
    pub const fn top() -> Self {
        Self::new(Edge::Top)
    }

    /// Constructs a `Slide` transition that transitions in from the bottom edge
    /// and out to the top edge.
    #[must_use]
    pub const fn bottom() -> Self {
        Self::new(Edge::Bottom)
    }
}

impl Transition for Slide {
    fn transform(&self, direction: Direction, factor: u8, bounds: Size) -> Point {
        let transform = match self.edge {
            Edge::Top => Point::new(0, -(bounds.height as i32)),
            Edge::Bottom => Point::new(0, bounds.height as i32),
            Edge::Leading => Point::new(-(bounds.width as i32), 0),
            Edge::Trailing => Point::new(bounds.width as i32, 0),
        };

        if direction == Direction::In {
            Interpolate::interpolate(transform, Point::zero(), factor)
        } else {
            Interpolate::interpolate(Point::zero(), -transform, factor)
        }
    }

    fn opacity(&self, _direction: Direction, _factor: u8) -> u8 {
        255
    }
}

#[cfg(test)]
mod tests {
    use super::Direction::{In, Out};
    use super::*;
    use crate::primitives::{Point, Size};

    const TEST_SIZE: Size = Size::new(100, 200);

    /// Helper function to test that transitions result in `Point::zero()` at rest positions
    fn assert_at_rest_position<T: Transition>(transition: &T, bounds: Size) {
        // Direction::In at factor 255 should be at rest (Point::zero())
        assert_eq!(
            transition.transform(In, 255, bounds),
            Point::zero(),
            "Direction::In with factor 255 should be at Point::zero()"
        );
        // Out at factor 0 should be at rest (Point::zero())
        assert_eq!(
            transition.transform(Out, 0, bounds),
            Point::zero(),
            "Out with factor 0 should be at Point::zero()"
        );
    }

    /// Helper function to test that transitions maintain full opacity (except Opacity transition)
    fn assert_full_opacity<T: Transition>(transition: &T) {
        for direction in [In, Out] {
            for factor in [0, 127, 255] {
                assert_eq!(
                    transition.opacity(direction, factor),
                    255,
                    "Transition should maintain full opacity for {direction:?} at factor {factor}"
                );
            }
        }
    }

    #[test]
    fn test_opacity_transition() {
        let transition = Opacity::new();

        // Transform should always be Point::zero()
        for direction in [In, Out] {
            for factor in [0, 127, 255] {
                assert_eq!(
                    transition.transform(direction, factor, TEST_SIZE),
                    Point::zero(),
                    "Opacity transition should never transform position"
                );
            }
        }

        // Test opacity values
        assert_eq!(transition.opacity(In, 0), 0);
        assert_eq!(transition.opacity(In, 128), 128);
        assert_eq!(transition.opacity(In, 255), 255);
        assert_eq!(transition.opacity(Out, 0), 255);
        assert_eq!(transition.opacity(Out, 128), 127);
        assert_eq!(transition.opacity(Out, 255), 0);
    }

    #[test]
    fn test_move_transition_top_edge() {
        let transition = Move::new(Edge::Top);
        let expected_offset = Point::new(0, -(TEST_SIZE.height as i32));

        assert_eq!(transition.transform(In, 0, TEST_SIZE), expected_offset);
        assert_eq!(transition.transform(In, 255, TEST_SIZE), Point::zero());

        assert_eq!(transition.transform(Out, 0, TEST_SIZE), Point::zero());
        assert_eq!(transition.transform(Out, 255, TEST_SIZE), expected_offset);

        assert_full_opacity(&transition);
    }

    #[test]
    fn test_move_transition_bottom_edge() {
        let transition = Move::new(Edge::Bottom);
        let expected_offset = Point::new(0, TEST_SIZE.height as i32);

        assert_eq!(transition.transform(In, 0, TEST_SIZE), expected_offset);
        assert_eq!(transition.transform(In, 255, TEST_SIZE), Point::zero());

        assert_eq!(transition.transform(Out, 0, TEST_SIZE), Point::zero());
        assert_eq!(transition.transform(Out, 255, TEST_SIZE), expected_offset);

        assert_full_opacity(&transition);
    }

    #[test]
    fn test_move_transition_leading_edge() {
        let transition = Move::new(Edge::Leading);
        let expected_offset = Point::new(-(TEST_SIZE.width as i32), 0);

        assert_eq!(transition.transform(In, 0, TEST_SIZE), expected_offset);
        assert_eq!(transition.transform(In, 255, TEST_SIZE), Point::zero());

        assert_eq!(transition.transform(Out, 0, TEST_SIZE), Point::zero());
        assert_eq!(transition.transform(Out, 255, TEST_SIZE), expected_offset);

        assert_full_opacity(&transition);
    }

    #[test]
    fn test_move_transition_trailing_edge() {
        let transition = Move::new(Edge::Trailing);
        let expected_offset = Point::new(TEST_SIZE.width as i32, 0);

        assert_eq!(transition.transform(In, 0, TEST_SIZE), expected_offset);
        assert_eq!(transition.transform(In, 255, TEST_SIZE), Point::zero());

        assert_eq!(transition.transform(Out, 0, TEST_SIZE), Point::zero());
        assert_eq!(transition.transform(Out, 255, TEST_SIZE), expected_offset);

        assert_full_opacity(&transition);
    }

    #[test]
    fn test_move_transition_intermediate_values() {
        let transition = Move::new(Edge::Top);
        let start_offset = Point::new(0, -(TEST_SIZE.height as i32));

        let in_halfway = transition.transform(In, 127, TEST_SIZE);
        let out_halfway = transition.transform(Out, 127, TEST_SIZE);

        assert_ne!(in_halfway, start_offset);
        assert_ne!(in_halfway, Point::zero());
        assert_ne!(out_halfway, start_offset);
        assert_ne!(out_halfway, Point::zero());
    }

    #[test]
    fn test_slide_transition_top_edge() {
        let transition = Slide::new(Edge::Top);
        let start_offset = Point::new(0, -(TEST_SIZE.height as i32));
        let opposite_offset = Point::new(0, TEST_SIZE.height as i32); // Bottom edge

        assert_eq!(transition.transform(In, 0, TEST_SIZE), start_offset);
        assert_eq!(transition.transform(In, 255, TEST_SIZE), Point::zero());

        assert_eq!(transition.transform(Out, 0, TEST_SIZE), Point::zero());
        assert_eq!(transition.transform(Out, 255, TEST_SIZE), opposite_offset);

        assert_full_opacity(&transition);
    }

    #[test]
    fn test_slide_transition_bottom_edge() {
        let transition = Slide::new(Edge::Bottom);
        let start_offset = Point::new(0, TEST_SIZE.height as i32);
        let opposite_offset = Point::new(0, -(TEST_SIZE.height as i32)); // Top edge

        assert_eq!(transition.transform(In, 0, TEST_SIZE), start_offset);
        assert_eq!(transition.transform(In, 255, TEST_SIZE), Point::zero());

        assert_eq!(transition.transform(Out, 0, TEST_SIZE), Point::zero());
        assert_eq!(transition.transform(Out, 255, TEST_SIZE), opposite_offset);

        assert_full_opacity(&transition);
    }

    #[test]
    fn test_slide_transition_leading_edge() {
        let transition = Slide::new(Edge::Leading);
        let start_offset = Point::new(-(TEST_SIZE.width as i32), 0);
        let opposite_offset = Point::new(TEST_SIZE.width as i32, 0); // Trailing edge

        assert_eq!(transition.transform(In, 0, TEST_SIZE), start_offset);
        assert_eq!(transition.transform(In, 255, TEST_SIZE), Point::zero());

        assert_eq!(transition.transform(Out, 0, TEST_SIZE), Point::zero());
        assert_eq!(transition.transform(Out, 255, TEST_SIZE), opposite_offset);

        assert_full_opacity(&transition);
    }

    #[test]
    fn test_slide_transition_trailing_edge() {
        let transition = Slide::new(Edge::Trailing);
        let start_offset = Point::new(TEST_SIZE.width as i32, 0);
        let opposite_offset = Point::new(-(TEST_SIZE.width as i32), 0); // Leading edge

        assert_eq!(transition.transform(In, 0, TEST_SIZE), start_offset);
        assert_eq!(transition.transform(In, 255, TEST_SIZE), Point::zero());

        assert_eq!(transition.transform(Out, 0, TEST_SIZE), Point::zero());
        assert_eq!(transition.transform(Out, 255, TEST_SIZE), opposite_offset);

        assert_full_opacity(&transition);
    }

    #[test]
    fn test_slide_transition_intermediate_values() {
        let transition = Slide::new(Edge::Leading);

        let in_halfway = transition.transform(In, 127, TEST_SIZE);
        let out_halfway = transition.transform(Out, 127, TEST_SIZE);

        assert_ne!(in_halfway, Point::new(-(TEST_SIZE.width as i32), 0));
        assert_ne!(in_halfway, Point::zero());
        assert_ne!(out_halfway, Point::new(TEST_SIZE.width as i32, 0));
        assert_ne!(out_halfway, Point::zero());
    }

    #[test]
    fn test_all_transitions_at_rest_positions() {
        // Test each transition type individually
        assert_at_rest_position(&Opacity::new(), TEST_SIZE);
        assert_at_rest_position(&Move::new(Edge::Top), TEST_SIZE);
        assert_at_rest_position(&Move::new(Edge::Bottom), TEST_SIZE);
        assert_at_rest_position(&Move::new(Edge::Leading), TEST_SIZE);
        assert_at_rest_position(&Move::new(Edge::Trailing), TEST_SIZE);
        assert_at_rest_position(&Slide::new(Edge::Top), TEST_SIZE);
        assert_at_rest_position(&Slide::new(Edge::Bottom), TEST_SIZE);
        assert_at_rest_position(&Slide::new(Edge::Leading), TEST_SIZE);
        assert_at_rest_position(&Slide::new(Edge::Trailing), TEST_SIZE);
    }
}
