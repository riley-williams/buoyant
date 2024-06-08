use crate::{
    layout::{Environment, PreRender},
    primitives::{Point, Size},
    render_target::RenderTarget,
};

/// A view that can be rendered to pixels
pub trait Render<Pixel, Cache> {
    /// Render the view to the screen
    fn render(
        &self,
        target: &mut impl RenderTarget<Pixel>,
        cache: &Cache,
        size: Size,
        env: &dyn Environment,
    );
}

impl<Pixel, Cache, View: Render<Pixel, Cache>> Render<Pixel, Cache> for PreRender<'_, View, Cache> {
    fn render(
        &self,
        target: &mut impl RenderTarget<Pixel>,
        cache: &Cache,
        size: Size,
        env: &dyn Environment,
    ) {
        self.source_view.render(target, cache, size, env)
    }
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
