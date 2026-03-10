use crate::primitives::geometry::Rectangle;
use crate::primitives::{Interpolate, Point, Size};
use crate::render::{AnimatedJoin, AnimationDomain, ContentShape, IntrinsicShape, Render};
use crate::render_target::{RenderTarget, SolidBrush};
use crate::primitives::transform::LinearTransform;

/// A rendered point/scatter series storing computed pixel coordinates.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PointRenderable<const N: usize> {
    /// The computed pixel coordinates of each data point.
    pub points: heapless::Vec<(i16, i16), N>,
    /// The point diameter in pixels.
    pub point_size: u32,
    /// The bounding frame of the chart area.
    pub frame: Rectangle,
}

impl<const N: usize> AnimatedJoin for PointRenderable<N> {
    fn join_from(&mut self, source: &Self, domain: &AnimationDomain) {
        let len = self.points.len().min(source.points.len());
        for i in 0..len {
            self.points[i].0 =
                i16::interpolate(source.points[i].0, self.points[i].0, domain.factor);
            self.points[i].1 =
                i16::interpolate(source.points[i].1, self.points[i].1, domain.factor);
        }
        self.point_size = u32::interpolate(source.point_size, self.point_size, domain.factor);
        self.frame = Rectangle::interpolate(source.frame.clone(), self.frame.clone(), domain.factor);
    }
}

impl<const N: usize, C: Copy> Render<C> for PointRenderable<N> {
    fn render(&self, render_target: &mut impl RenderTarget<ColorFormat = C>, style: &C) {
        let brush = SolidBrush::new(*style);
        let half = (self.point_size / 2) as i32;

        for &(px, py) in &self.points {
            let rect = Rectangle::new(
                Point::new(i32::from(px) - half, i32::from(py) - half),
                Size::new(self.point_size, self.point_size),
            );
            render_target.fill(LinearTransform::default(), &brush, None, &rect);
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

impl<const N: usize> IntrinsicShape for PointRenderable<N> {
    fn content_shape(&self) -> ContentShape {
        ContentShape::Rectangle(self.frame.clone())
    }
}
