use core::fmt::{Display, Formatter, Result};

use crate::{
    layout::Environment,
    primitives::{Point, Size},
};

/// A target that accepts pixels
pub trait RenderTarget<C> {
    /// The size of the render target
    fn size(&self) -> Size;

    /// Clear the render target
    fn clear(&mut self);

    /// Draw a pixel to the render target
    fn draw(&mut self, point: Point, item: C);
}

/// A view that can be rendered to pixels
pub trait Render<C> {
    /// Render the view to the screen
    fn render(&self, target: &mut impl RenderTarget<C>, env: &dyn Environment);
}

pub struct FixedTextBuffer<const W: usize, const H: usize> {
    pub text: [[char; W]; H],
}

impl<const W: usize, const H: usize> Display for FixedTextBuffer<W, H> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for line in self.text.iter() {
            for c in line.iter() {
                write!(f, "{}", c)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<const W: usize, const H: usize> Default for FixedTextBuffer<W, H> {
    fn default() -> Self {
        Self {
            text: [[' '; W]; H],
        }
    }
}

impl<const W: usize, const H: usize> RenderTarget<char> for FixedTextBuffer<W, H> {
    fn size(&self) -> Size {
        Size::new(W as u16, H as u16)
    }

    fn clear(&mut self) {
        for line in self.text.iter_mut() {
            for c in line.iter_mut() {
                *c = ' ';
            }
        }
    }

    fn draw(&mut self, point: Point, item: char) {
        let y = point.y as usize;
        let x = point.x as usize;
        if y < H && x < W {
            self.text[y][x] = item;
        }
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
