use crate::{
    environment::{LayoutEnvironment, RenderEnvironment},
    layout::{Layout, LayoutDirection, ResolvedLayout},
    pixel::PixelColor,
    primitives::{Point, Size},
    render::CharacterRender,
    render_target::CharacterRenderTarget,
    style::color_style::ColorStyle,
};

pub struct Divider {
    pub weight: u16,
}

impl Divider {
    pub fn new(weight: u16) -> Self {
        Self { weight }
    }
}

impl Default for Divider {
    fn default() -> Self {
        Self::new(1)
    }
}

impl PartialEq for Divider {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl Layout for Divider {
    type Sublayout = ();
    fn layout(&self, offer: Size, env: &impl LayoutEnvironment) -> ResolvedLayout<()> {
        let size = match env.layout_direction() {
            LayoutDirection::Horizontal => Size::new(self.weight, offer.height),
            LayoutDirection::Vertical => Size::new(offer.width, self.weight),
        };
        ResolvedLayout {
            sublayouts: (),
            resolved_size: size,
        }
    }

    fn priority(&self) -> i8 {
        i8::MAX
    }
}

#[cfg(feature = "embedded-graphics")]
use embedded_graphics::draw_target::DrawTarget;

#[cfg(feature = "embedded-graphics")]
impl<C: PixelColor + embedded_graphics_core::pixelcolor::PixelColor> crate::render::PixelRender<C>
    for Divider
{
    fn render(
        &self,
        target: &mut impl DrawTarget<Color = C>,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl RenderEnvironment<C>,
    ) {
        match env.layout_direction() {
            LayoutDirection::Horizontal => {
                for y in 0..layout.resolved_size.height as i16 {
                    let foreground_color =
                        env.foreground_style()
                            .shade_pixel(0, y as u16, layout.resolved_size);

                    let point = origin + Point::new(0, y);
                    _ = target.draw_iter(core::iter::once(embedded_graphics::Pixel(
                        point.into(),
                        foreground_color,
                    )));
                }
            }
            LayoutDirection::Vertical => {
                for x in 0..layout.resolved_size.width as i16 {
                    let foreground_color =
                        env.foreground_style()
                            .shade_pixel(x as u16, 0, layout.resolved_size);

                    let point = origin + Point::new(x, 0);
                    _ = target.draw_iter(core::iter::once(embedded_graphics::Pixel(
                        point.into(),
                        foreground_color,
                    )));
                }
            }
        }
    }
}

impl<C: PixelColor> CharacterRender<C> for Divider {
    fn render(
        &self,
        target: &mut impl CharacterRenderTarget<Color = C>,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl RenderEnvironment<C>,
    ) {
        match env.layout_direction() {
            LayoutDirection::Horizontal => {
                for y in origin.y..origin.y + layout.resolved_size.height as i16 {
                    let color = env.foreground_style().shade_pixel(
                        0,
                        y as u16 - origin.y as u16,
                        Size::new(1, layout.resolved_size.height),
                    );

                    target.draw(Point::new(origin.x, y), '|', color);
                }
            }
            LayoutDirection::Vertical => {
                for x in origin.x..origin.x + layout.resolved_size.width as i16 {
                    let color = env.foreground_style().shade_pixel(
                        x as u16 - origin.x as u16,
                        0,
                        Size::new(layout.resolved_size.width, 1),
                    );

                    target.draw(Point::new(x, origin.y), '-', color);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Divider;
    use crate::environment::mock::TestEnv;
    use crate::layout::{Layout, LayoutDirection};
    use crate::primitives::{Point, Size};
    use crate::render::CharacterRender;
    use crate::render_target::{CharacterRenderTarget, FixedTextBuffer};

    #[test]
    fn test_horizontal_layout() {
        let divider = Divider::new(2);
        let offer = Size::new(100, 100);
        let env = TestEnv::<char>::default().with_direction(LayoutDirection::Horizontal);
        let layout = divider.layout(offer, &env);
        assert_eq!(layout.resolved_size, Size::new(2, 100));
    }

    #[test]
    fn test_vertical_layout() {
        let divider = Divider::new(2);
        let offer = Size::new(100, 100);
        let env = TestEnv::<char>::default().with_direction(LayoutDirection::Vertical);
        let layout = divider.layout(offer, &env);
        assert_eq!(layout.resolved_size, Size::new(100, 2));
    }

    #[test]
    fn test_horizontal_render() {
        let divider = Divider::new(1);
        let mut buffer = FixedTextBuffer::<5, 5>::default();
        let env = TestEnv::default().with_direction(LayoutDirection::Horizontal);
        let layout = divider.layout(buffer.size(), &env);
        divider.render(&mut buffer, &layout, Point::zero(), &env);
        assert_eq!(buffer.text[0][0], '|');
        assert_eq!(buffer.text[4][0], '|');
        assert_eq!(buffer.text[0][1], ' ');
    }

    #[test]
    fn test_vertical_render() {
        let divider = Divider::new(1);
        let mut buffer = FixedTextBuffer::<5, 5>::default();
        let env = TestEnv::default().with_direction(LayoutDirection::Vertical);
        let layout = divider.layout(buffer.size(), &env);
        divider.render(&mut buffer, &layout, Point::zero(), &env);
        assert_eq!(buffer.text[0][0], '-');
        assert_eq!(buffer.text[0][4], '-');
        assert_eq!(buffer.text[1][0], ' ');
    }
}
