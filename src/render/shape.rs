mod capsule;
mod circle;
mod rect;
mod rounded_rect;

pub use capsule::Capsule;
pub use circle::Circle;
pub use rect::Rect;
pub use rounded_rect::RoundedRect;

use crate::{
    primitives::{geometry::Shape, Interpolate, Point},
    render::{AnimatedJoin, AnimationDomain, Render},
    render_target::{RenderTarget, SolidBrush, Stroke},
};

pub trait Inset {
    /// Returns the inset version of the shape.
    #[must_use]
    fn inset(self, amount: i32) -> Self;
}

pub trait AsShapePrimitive {
    type Primitive: Shape;
    fn as_shape(&self) -> Self::Primitive;
}

// Implements fill for all shape primitive types
impl<T: AnimatedJoin + Clone + AsShapePrimitive, C: Copy> Render<C> for T {
    fn render(
        &self,
        render_target: &mut impl RenderTarget<ColorFormat = C>,
        style: &C,
        offset: Point,
    ) {
        render_target.fill(offset, &SolidBrush::new(*style), None, &self.as_shape());
    }

    fn render_animated(
        render_target: &mut impl RenderTarget<ColorFormat = C>,
        source: &Self,
        target: &Self,
        style: &C,
        offset: Point,
        domain: &AnimationDomain,
    ) {
        AnimatedJoin::join(source.clone(), target.clone(), domain).render(
            render_target,
            style,
            offset,
        );
    }
}

/// A shape that is stroked with a specified line width.
#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StrokedShape<T> {
    shape: T,
    line_width: u32,
}

impl<T> StrokedShape<T> {
    #[must_use]
    pub const fn new(shape: T, line_width: u32) -> Self {
        Self { shape, line_width }
    }
}

impl<T: AnimatedJoin> AnimatedJoin for StrokedShape<T> {
    fn join(source: Self, target: Self, domain: &AnimationDomain) -> Self {
        let shape = AnimatedJoin::join(source.shape, target.shape, domain);
        let line_width = u32::interpolate(source.line_width, target.line_width, domain.factor);
        Self { shape, line_width }
    }
}

impl<T: AnimatedJoin + Clone + AsShapePrimitive, C: Copy> Render<C> for StrokedShape<T> {
    fn render(
        &self,
        render_target: &mut impl RenderTarget<ColorFormat = C>,
        style: &C,
        offset: Point,
    ) {
        render_target.stroke(
            &Stroke {
                width: self.line_width,
            },
            offset,
            &SolidBrush::new(*style),
            None,
            &self.shape.as_shape(),
        );
    }

    fn render_animated(
        render_target: &mut impl RenderTarget<ColorFormat = C>,
        source: &Self,
        target: &Self,
        style: &C,
        offset: Point,
        domain: &AnimationDomain,
    ) {
        AnimatedJoin::join(source.clone(), target.clone(), domain).render(
            render_target,
            style,
            offset,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::render::Circle;

    #[test]
    fn join_stroked_shape() {
        let shape1 = StrokedShape::new(
            Circle {
                origin: Point::new(0, 0),
                diameter: 10,
            },
            2,
        );
        let shape2 = StrokedShape::new(
            Circle {
                origin: Point::new(10, 10),
                diameter: 20,
            },
            4,
        );
        let domain = AnimationDomain::new(128, core::time::Duration::from_millis(100));

        let joined_shape = AnimatedJoin::join(shape1, shape2, &domain);
        assert_eq!(
            joined_shape,
            StrokedShape::new(
                Circle {
                    origin: Point::new(5, 5),
                    diameter: 15,
                },
                3
            )
        );
    }
}
