use crate::primitives::geometry::Rectangle;
use crate::primitives::{Interpolate, Point, Size};
use crate::render::{AnimatedJoin, AnimationDomain, ContentShape, IntrinsicShape, Render};
use crate::render_target::{RenderTarget, SolidBrush};
use crate::primitives::transform::LinearTransform;

/// A single bar's pixel-space geometry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChartBar {
    pub x: i16,
    pub y: i16,
    pub width: i16,
    pub height: i16,
}

impl ChartBar {
    fn to_rectangle(self) -> Rectangle {
        Rectangle::new(
            Point::new(i32::from(self.x), i32::from(self.y)),
            Size::new(self.width.max(0) as u32, self.height.max(0) as u32),
        )
    }
}

/// A rendered bar series storing computed pixel rectangles.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BarRenderable<const N: usize> {
    /// The computed pixel rectangles for each bar.
    pub bars: heapless::Vec<ChartBar, N>,
    /// The bounding frame of the chart area.
    pub frame: Rectangle,
}

impl<const N: usize> AnimatedJoin for BarRenderable<N> {
    fn join_from(&mut self, source: &Self, domain: &AnimationDomain) {
        let len = self.bars.len().min(source.bars.len());
        for i in 0..len {
            self.bars[i].x = i16::interpolate(source.bars[i].x, self.bars[i].x, domain.factor);
            self.bars[i].y = i16::interpolate(source.bars[i].y, self.bars[i].y, domain.factor);
            self.bars[i].width =
                i16::interpolate(source.bars[i].width, self.bars[i].width, domain.factor);
            self.bars[i].height =
                i16::interpolate(source.bars[i].height, self.bars[i].height, domain.factor);
        }
        self.frame = Rectangle::interpolate(source.frame.clone(), self.frame.clone(), domain.factor);
    }
}

impl<const N: usize, C: Copy> Render<C> for BarRenderable<N> {
    fn render(&self, render_target: &mut impl RenderTarget<ColorFormat = C>, style: &C) {
        let brush = SolidBrush::new(*style);

        for bar in &self.bars {
            render_target.fill(
                LinearTransform::default(),
                &brush,
                None,
                &bar.to_rectangle(),
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

impl<const N: usize> IntrinsicShape for BarRenderable<N> {
    fn content_shape(&self) -> ContentShape {
        ContentShape::Rectangle(self.frame.clone())
    }
}
