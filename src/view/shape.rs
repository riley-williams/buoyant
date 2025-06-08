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
    layout::{Layout, ResolvedLayout},
    primitives::{Point, ProposedDimensions},
    render::{
        shape::{AsShapePrimitive, Inset},
        Renderable, StrokedShape,
    },
};

pub trait ShapeExt: Renderable
where
    Self::Renderables: Inset + AsShapePrimitive,
{
    /// Draws a shape with a stroke instead of filling it.
    #[must_use]
    fn stroked(self, line_width: u32) -> Stroked<Self> {
        Stroked::new(self, StrokeOffset::Inner, line_width)
    }

    /// Draws a shape with a stroke instead of filling it.
    ///
    /// Using an offset other than ``StrokeOffset::Inner`` will render outside the shape's bounds.
    #[must_use]
    fn stroked_offset(self, line_width: u32, offset: StrokeOffset) -> Stroked<Self> {
        Stroked::new(self, offset, line_width)
    }
}

impl<T: Renderable> ShapeExt for T where T::Renderables: Inset + AsShapePrimitive {}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum StrokeOffset {
    Outer,
    Center,
    #[default]
    Inner,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stroked<T> {
    shape: T,
    style: StrokeOffset,
    line_width: u32,
}

impl<T: Renderable> Stroked<T>
where
    T::Renderables: Inset,
{
    /// Creates a new stroked shape with the given style and line width.
    const fn new(shape: T, style: StrokeOffset, line_width: u32) -> Self {
        Self {
            shape,
            style,
            line_width,
        }
    }
}

impl<T: Layout> Layout for Stroked<T> {
    type Sublayout = T::Sublayout;

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        self.shape.layout(offer, env)
    }
}

impl<T: Renderable> Renderable for Stroked<T>
where
    T::Renderables: Inset,
{
    type Renderables = StrokedShape<T::Renderables>;

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl LayoutEnvironment,
    ) -> Self::Renderables {
        let inset = match self.style {
            StrokeOffset::Outer => -(self.line_width as i32 / 2),
            StrokeOffset::Inner => self.line_width as i32 / 2,
            StrokeOffset::Center => 0,
        };
        StrokedShape::new(
            self.shape.render_tree(layout, origin, env).inset(inset),
            self.line_width,
        )
    }
}
