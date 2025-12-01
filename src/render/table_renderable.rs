use heapless::Vec;

use crate::{
    primitives::{
        Dimensions, Interpolate, Point, Size, geometry::Rectangle, transform::LinearTransform,
    },
    render::{AnimatedJoin, AnimationDomain, Render},
    render_target::{RenderTarget, SolidBrush},
};

#[derive(Debug, Clone)]
pub struct TableRenderable<T, const R: usize, const C: usize> {
    pub renderables: Vec<Vec<T, C>, R>,
    pub origin: Point,
    pub resolved_size: Dimensions,
    pub width: usize,
    pub height: usize,
    pub col_widths: [u32; C],
    pub row_heights: [u32; R],
    pub col_stroke: u32,
    pub row_stroke: u32,
}

impl<const R: usize, const C: usize, V: AnimatedJoin> AnimatedJoin for TableRenderable<V, R, C> {
    fn join_from(&mut self, source: &Self, domain: &AnimationDomain) {
        self.renderables.join_from(&source.renderables, domain);
        self.col_stroke = u32::interpolate(source.col_stroke, self.col_stroke, domain.factor);
        self.row_stroke = u32::interpolate(source.row_stroke, self.row_stroke, domain.factor);
    }
}

impl<const R: usize, const C: usize, Color: Copy, T> Render<Color> for TableRenderable<T, R, C>
where
    T: Clone + Render<Color>,
{
    fn render(&self, render_target: &mut impl RenderTarget<ColorFormat = Color>, style: &Color) {
        let mut x = self.origin.x + self.col_widths[0] as i32;
        for c in 1..self.width {
            render_target.fill(
                LinearTransform::default(),
                &SolidBrush::new(*style),
                None,
                &Rectangle::new(
                    Point::new(x, self.origin.y),
                    Size::new(self.col_stroke, self.resolved_size.height.0),
                ),
            );
            x += self.col_stroke as i32 + self.col_widths[c] as i32;
        }

        let mut y = self.origin.y + self.row_heights[0] as i32;
        for r in 1..self.height {
            render_target.fill(
                LinearTransform::default(),
                &SolidBrush::new(*style),
                None,
                &Rectangle::new(
                    Point::new(self.origin.x, y),
                    Size::new(self.resolved_size.width.0, self.row_stroke),
                ),
            );
            y += self.row_stroke as i32 + self.row_heights[r] as i32;
        }

        self.renderables.render(render_target, style);
    }

    fn render_animated(
        render_target: &mut impl RenderTarget<ColorFormat = Color>,
        source: &Self,
        target: &Self,
        style: &Color,
        domain: &AnimationDomain,
    ) {
        let mut joined_shape = target.clone();
        joined_shape.join_from(source, domain);
        joined_shape.render(render_target, style);
    }
}
