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
        _: &impl crate::environment::Environment,
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
        _: &impl crate::environment::Environment,
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
use crossterm::style::Stylize;

#[cfg(feature = "crossterm")]
impl<'a> Render<crossterm::style::StyledContent<&'a str>, ()> for Rectangle {
    fn render(
        &self,
        target: &mut impl RenderTarget<crossterm::style::StyledContent<&'a str>>,
        layout: &ResolvedLayout<()>,
        env: &impl crate::environment::Environment,
    ) {
        let width = layout.resolved_size.width;
        let height = layout.resolved_size.height;
        for y in 0..height {
            for x in 0..width {
                let foreground_color =
                    env.foreground_style()
                        .shade_pixel(x, y, layout.resolved_size);
                let color = crossterm::style::Color::Rgb {
                    r: foreground_color.r,
                    g: foreground_color.g,
                    b: foreground_color.b,
                };

                let c = "#".with(color);
                target.draw(Point::new(x as i16, y as i16), c);
            }
        }
    }
}
