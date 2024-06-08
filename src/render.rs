use crate::{
    layout::{Environment, ResolvedLayout},
    primitives::{Point, Size},
    render_target::RenderTarget,
};

/// A view that can be rendered to pixels
pub trait Render<Pixel, Sublayout> {
    /// Render the view to the screen
    fn render(
        &self,
        target: &mut impl RenderTarget<Pixel>,
        layout: &ResolvedLayout<Sublayout>,
        env: &dyn Environment,
    );
}

pub struct RenderProxy<'a, T> {
    target: &'a mut T,
    pub origin: Point,
}

impl<'a, T> RenderProxy<'a, T> {
    pub fn new(target: &'a mut T, origin: Point) -> Self {
        RenderProxy { target, origin }
    }
}

impl<'a, T: RenderTarget<I>, I> RenderTarget<I> for RenderProxy<'a, T> {
    fn size(&self) -> Size {
        self.target.size()
    }
    fn clear(&mut self) {
        self.target.clear()
    }

    fn draw(&mut self, point: Point, item: I) {
        self.target.draw(point + self.origin, item)
    }
}

pub struct ClippingRenderProxy<'a, T> {
    target: &'a mut T,
    pub origin: Point,
    pub size: Size,
}

impl<'a, T> ClippingRenderProxy<'a, T> {
    pub fn new(target: &'a mut T, origin: Point, size: Size) -> Self {
        Self {
            target,
            origin,
            size,
        }
    }
}

impl<'a, T: RenderTarget<I>, I> RenderTarget<I> for ClippingRenderProxy<'a, T> {
    fn size(&self) -> Size {
        self.target.size()
    }
    fn clear(&mut self) {
        self.target.clear()
    }

    fn draw(&mut self, point: Point, item: I) {
        if point.x < self.size.width as i16 && point.y < self.size.height as i16 {
            self.target.draw(point + self.origin, item)
        }
    }
}
