use crate::{
    layout::{Layout, ResolvedLayout},
    primitives::{Point, Size},
    render::Render,
    render_target::RenderTarget,
};

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Rectangle;

impl Layout for Rectangle {
    type Sublayout = ();

    fn layout(
        &self,
        offer: Size,
        _: &impl crate::environment::LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        ResolvedLayout {
            sublayouts: (),
            resolved_size: offer,
        }
    }
}

impl Render<char, ()> for Rectangle {
    fn render(
        &self,
        target: &mut impl RenderTarget<char>,
        layout: &ResolvedLayout<()>,
        _: &impl crate::environment::LayoutEnvironment,
    ) {
        let width = layout.resolved_size.width;
        let height = layout.resolved_size.height;
        for y in 0..height {
            for x in 0..width {
                let c = if x == 0 || x == width - 1 || y == 0 || y == height - 1 {
                    '#'
                } else {
                    ' '
                };
                target.draw(Point::new(x as i16, y as i16), c);
            }
        }
    }
}

#[cfg(feature = "crossterm")]
use crate::style::color_style::ColorStyle;

#[cfg(feature = "crossterm")]
impl Render<crate::pixel::CrosstermColorSymbol, ()> for Rectangle {
    fn render(
        &self,
        target: &mut impl RenderTarget<crate::pixel::CrosstermColorSymbol>,
        layout: &ResolvedLayout<()>,
        env: &impl crate::environment::RenderEnvironment<crate::pixel::CrosstermColorSymbol>,
    ) {
        let width = layout.resolved_size.width;
        let height = layout.resolved_size.height;
        for y in 0..height {
            for x in 0..width {
                let mut foreground_color =
                    env.foreground_style()
                        .shade_pixel(x, y, layout.resolved_size);
                foreground_color.character = '#';
                target.draw(Point::new(x as i16, y as i16), foreground_color);
            }
        }
    }
}

#[cfg(feature = "embedded-graphics")]
use embedded_graphics::pixelcolor::BinaryColor;

#[cfg(feature = "embedded-graphics")]
impl Render<BinaryColor, ()> for Rectangle {
    fn render(
        &self,
        target: &mut impl RenderTarget<BinaryColor>,
        layout: &ResolvedLayout<()>,
        _: &impl crate::environment::RenderEnvironment<BinaryColor>,
    ) {
        let width = layout.resolved_size.width;
        let height = layout.resolved_size.height;
        for y in 0..height {
            for x in 0..width {
                target.draw(Point::new(x as i16, y as i16), BinaryColor::On);
            }
        }
    }
}
