//! Shape primitives

mod capsule;
mod circle;
mod rectangle;
mod rounded_rectangle;

pub use capsule::Capsule;
pub use circle::Circle;
pub use rectangle::Rectangle;
pub use rounded_rectangle::RoundedRectangle;

use crate::{
    environment::LayoutEnvironment,
    layout::ResolvedLayout,
    primitives::{Point, ProposedDimensions},
    render::{
        shape::{AsShapePrimitive, Inset},
        StrokedShape,
    },
    transition::Opacity,
    view::{ViewLayout, ViewMarker},
};

/// Extension trait for shapes that provides methods to draw them with a stroke.
pub trait ShapeExt: ViewMarker<Renderables: Inset + AsShapePrimitive> {
    /// Draws a shape with a stroke instead of filling it.
    #[must_use]
    fn stroked(self, line_width: u32) -> Stroked<Self> {
        Stroked::new(self, StrokeOffset::Inner, line_width)
    }

    /// Draws a shape with a stroke instead of filling it.
    ///
    /// Using an offset other than [`StrokeOffset::Inner`] will render outside the shape's bounds.
    #[must_use]
    fn stroked_offset(self, line_width: u32, offset: StrokeOffset) -> Stroked<Self> {
        Stroked::new(self, offset, line_width)
    }
}

impl<T: ViewMarker<Renderables: Inset + AsShapePrimitive>> ShapeExt for T {}

/// How the stroke should be drawn relative to the shape bounds.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum StrokeOffset {
    /// Draws the stroke outside the shape bounds
    Outer,
    /// Draws the stroke centered on the shape bounds
    Center,
    /// Draws the stroke inside the shape bounds
    #[default]
    Inner,
}

/// A stroked variant of a shape.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stroked<T> {
    shape: T,
    style: StrokeOffset,
    line_width: u32,
}

impl<T> Stroked<T> {
    /// Creates a new stroked shape with the given style and line width.
    const fn new(shape: T, style: StrokeOffset, line_width: u32) -> Self {
        Self {
            shape,
            style,
            line_width,
        }
    }
}

impl<T: ViewMarker> ViewMarker for Stroked<T> {
    type Renderables = StrokedShape<T::Renderables>;
    type Transition = Opacity;
}

impl<T, Captures: ?Sized> ViewLayout<Captures> for Stroked<T>
where
    T: ViewLayout<Captures>,
    T::Renderables: Inset + crate::render::AnimatedJoin + Clone + AsShapePrimitive,
{
    type Sublayout = T::Sublayout;
    type State = T::State;

    fn transition(&self) -> Self::Transition {
        Opacity
    }

    fn build_state(&self, captures: &mut Captures) -> Self::State {
        self.shape.build_state(captures)
    }

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        self.shape.layout(offer, env, captures, state)
    }

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> Self::Renderables {
        let inset = match self.style {
            StrokeOffset::Outer => -(self.line_width as i32 / 2),
            StrokeOffset::Inner => self.line_width as i32 / 2,
            StrokeOffset::Center => 0,
        };
        StrokedShape::new(
            self.shape
                .render_tree(layout, origin, env, captures, state)
                .inset(inset),
            self.line_width,
        )
    }
}
