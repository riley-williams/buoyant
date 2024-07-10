use crate::{
    environment::Environment,
    layout::{Layout, LayoutDirection, ResolvedLayout},
    primitives::{Point, Size},
    render::Render,
    render_target::RenderTarget,
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
    fn layout(&self, offer: Size, env: &impl Environment) -> ResolvedLayout<()> {
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

impl Render<char, ()> for Divider {
    fn render(
        &self,
        target: &mut impl RenderTarget<char>,
        layout: &ResolvedLayout<()>,
        env: &impl Environment,
    ) {
        match env.layout_direction() {
            LayoutDirection::Horizontal => {
                for y in 0..layout.resolved_size.height {
                    target.draw(Point::new(0, y as i16), '|');
                }
            }
            LayoutDirection::Vertical => {
                for x in 0..layout.resolved_size.width {
                    target.draw(Point::new(x as i16, 0), '-');
                }
            }
        }
    }
}

#[cfg(feature = "crossterm")]
use crossterm::style::{StyledContent, Stylize};

#[cfg(feature = "crossterm")]
use crate::style::color_style::ColorStyle;

#[cfg(feature = "crossterm")]
impl<'a> Render<StyledContent<&'a str>, ()> for Divider {
    fn render(
        &self,
        target: &mut impl RenderTarget<StyledContent<&'a str>>,
        layout: &ResolvedLayout<()>,
        env: &impl Environment,
    ) {
        let foreground_color = env.foreground_style().shade_pixel(0, 0, Size::new(0, 0));
        let color = crossterm::style::Color::Rgb {
            r: foreground_color.r,
            g: foreground_color.g,
            b: foreground_color.b,
        };

        match env.layout_direction() {
            LayoutDirection::Horizontal => {
                for y in 0..layout.resolved_size.height {
                    target.draw(Point::new(0, y as i16), "|".with(color));
                }
            }
            LayoutDirection::Vertical => {
                for x in 0..layout.resolved_size.width {
                    target.draw(Point::new(x as i16, 0), "-".with(color));
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
        let env = TestEnv::default().with_direction(LayoutDirection::Horizontal);
        let layout = divider.layout(offer, &env);
        assert_eq!(layout.resolved_size, Size::new(2, 100));
    }

    #[test]
    fn test_vertical_layout() {
        let divider = Divider::new(2);
        let offer = Size::new(100, 100);
        let env = TestEnv::default().with_direction(LayoutDirection::Vertical);
        let layout = divider.layout(offer, &env);
        assert_eq!(layout.resolved_size, Size::new(100, 2));
    }

    #[test]
    fn test_horizontal_render() {
        let divider = Divider::new(1);
        let mut buffer = FixedTextBuffer::<5, 5>::default();
        let env = TestEnv::default().with_direction(LayoutDirection::Horizontal);
        let layout = divider.layout(buffer.size(), &env);
        divider.render(&mut buffer, &layout, &env);
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
        divider.render(&mut buffer, &layout, &env);
        assert_eq!(buffer.text[0][0], '-');
        assert_eq!(buffer.text[0][4], '-');
        assert_eq!(buffer.text[1][0], ' ');
    }
}
