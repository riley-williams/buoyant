use crate::primitives::geometry::Rectangle;
use crate::primitives::{Interpolate, Point};
use crate::render::{AnimatedJoin, AnimationDomain, ContentShape, IntrinsicShape, Render};
use crate::render_target::{RenderTarget, SolidBrush, Stroke};
use crate::primitives::geometry::Line;
use crate::primitives::transform::LinearTransform;

/// A rendered line series storing computed pixel coordinates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineRenderable<const N: usize> {
    /// The computed pixel coordinates of each data point.
    pub points: heapless::Vec<(i16, i16), N>,
    /// The line width in pixels.
    pub line_width: u32,
    /// The bounding frame of the chart area.
    pub frame: Rectangle,
}

impl<const N: usize> AnimatedJoin for LineRenderable<N> {
    fn join_from(&mut self, source: &Self, domain: &AnimationDomain) {
        let len = self.points.len().min(source.points.len());
        for i in 0..len {
            self.points[i].0 =
                i16::interpolate(source.points[i].0, self.points[i].0, domain.factor);
            self.points[i].1 =
                i16::interpolate(source.points[i].1, self.points[i].1, domain.factor);
        }
        self.line_width = u32::interpolate(source.line_width, self.line_width, domain.factor);
        self.frame = Rectangle::interpolate(source.frame.clone(), self.frame.clone(), domain.factor);
    }
}

impl<const N: usize, C: Copy> Render<C> for LineRenderable<N> {
    fn render(&self, render_target: &mut impl RenderTarget<ColorFormat = C>, style: &C) {
        let brush = SolidBrush::new(*style);
        let stroke = Stroke::new(self.line_width);

        for pair in self.points.windows(2) {
            let start = Point::new(i32::from(pair[0].0), i32::from(pair[0].1));
            let end = Point::new(i32::from(pair[1].0), i32::from(pair[1].1));
            render_target.stroke(
                &stroke,
                LinearTransform::default(),
                &brush,
                None,
                &Line::new(start, end),
            );
        }
    }

    fn render_animated(
        render_target: &mut impl RenderTarget<ColorFormat = C>,
        source: &Self,
        target: &Self,
        style: &C,
        domain: &AnimationDomain,
    ) {
        let mut joined = target.clone();
        joined.join_from(source, domain);
        joined.render(render_target, style);
    }
}

impl<const N: usize> IntrinsicShape for LineRenderable<N> {
    fn content_shape(&self) -> ContentShape {
        ContentShape::Rectangle(self.frame.clone())
    }
}
