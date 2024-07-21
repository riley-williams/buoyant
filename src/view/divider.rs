use crate::{
    environment::{LayoutEnvironment, RenderEnvironment},
    layout::{Layout, LayoutDirection, ResolvedLayout},
    primitives::{Point, Size},
    render::Render,
    render_target::RenderTarget,
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
impl Render<embedded_graphics::pixelcolor::Rgb565> for Divider {
    fn render(
        &self,
        target: &mut impl RenderTarget<embedded_graphics::pixelcolor::Rgb565>,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl RenderEnvironment<embedded_graphics::pixelcolor::Rgb565>,
    ) {
        match env.layout_direction() {
            LayoutDirection::Horizontal => {
                for y in 0..layout.resolved_size.height as i16 {
                    let foreground_color =
                        env.foreground_style()
                            .shade_pixel(0, y as u16, layout.resolved_size);

                    target.draw(origin + Point::new(0, y), foreground_color);
                }
            }
            LayoutDirection::Vertical => {
                for x in 0..layout.resolved_size.width as i16 {
                    let foreground_color =
                        env.foreground_style()
                            .shade_pixel(x as u16, 0, layout.resolved_size);

                    target.draw(origin + Point::new(x, 0), foreground_color);
                }
            }
        }
    }
}

impl Render<char> for Divider {
    fn render(
        &self,
        target: &mut impl RenderTarget<char>,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl RenderEnvironment<char>,
    ) {
        match env.layout_direction() {
            LayoutDirection::Horizontal => {
                for y in origin.y..origin.y + layout.resolved_size.height as i16 {
                    target.draw(Point::new(origin.x, y), '|');
                }
            }
            LayoutDirection::Vertical => {
                for x in origin.x..origin.x + layout.resolved_size.width as i16 {
                    target.draw(Point::new(x, origin.y), '-');
                }
            }
        }
    }
}

#[cfg(feature = "crossterm")]
use crate::pixel::CrosstermColorSymbol;

#[cfg(feature = "crossterm")]
impl Render<CrosstermColorSymbol> for Divider {
    fn render(
        &self,
        target: &mut impl RenderTarget<CrosstermColorSymbol>,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl RenderEnvironment<CrosstermColorSymbol>,
    ) {
        // TODO: we can make this rainbow
        let mut color = env.foreground_style().shade_pixel(0, 0, Size::new(0, 0));

        match env.layout_direction() {
            LayoutDirection::Horizontal => {
                color.character = '|';
                for y in origin.y..origin.y + layout.resolved_size.height as i16 {
                    target.draw(Point::new(origin.x, y), color);
                }
            }
            LayoutDirection::Vertical => {
                color.character = '-';
                for x in origin.x..origin.x + layout.resolved_size.width as i16 {
                    target.draw(Point::new(x, origin.y), color);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::environment::mock::TestEnv;
    use crate::layout::{Layout, LayoutDirection};
    use crate::primitives::Size;
    use crate::render_target::FixedTextBuffer;

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
