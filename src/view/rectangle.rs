use crate::{
    layout::{Layout, ResolvedLayout},
    pixel::ColorValue,
    primitives::{Point, Size},
    render::Render,
    render_target::RenderTarget,
    style::color_style::ColorStyle,
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

impl<P: ColorValue> Render<P, ()> for Rectangle {
    default fn render(
        &self,
        target: &mut impl RenderTarget<P>,
        layout: &ResolvedLayout<()>,
        origin: Point,
        env: &impl crate::environment::RenderEnvironment<P>,
    ) {
        let width = layout.resolved_size.width;
        let height = layout.resolved_size.height;
        for y in origin.y..origin.y + height as i16 {
            for x in origin.x..origin.x + width as i16 {
                let foreground_color =
                    env.foreground_style()
                        .shade_pixel(x as u16, y as u16, layout.resolved_size);
                target.draw(Point::new(x, y), foreground_color);
            }
        }
    }
}
