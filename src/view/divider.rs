use crate::{
    environment::{LayoutEnvironment, RenderEnvironment},
    layout::{Layout, LayoutDirection, ResolvedLayout},
    primitives::{Dimensions, Point, ProposedDimensions},
    render::CharacterRender,
    render_target::CharacterRenderTarget,
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
    fn layout(
        &self,
        offer: ProposedDimensions,
        env: &impl LayoutEnvironment,
    ) -> ResolvedLayout<()> {
        let size = match env.layout_direction() {
            LayoutDirection::Vertical => Dimensions {
                width: offer.width.resolve_most_flexible(0, 10),
                height: self.weight.into(),
            },
            LayoutDirection::Horizontal => Dimensions {
                width: self.weight.into(),
                height: offer.height.resolve_most_flexible(0, 10),
            },
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
use embedded_graphics::{draw_target::DrawTarget, primitives::Rectangle};

#[cfg(feature = "embedded-graphics")]
impl<C: embedded_graphics_core::pixelcolor::PixelColor> crate::render::PixelRender<C> for Divider {
    fn render(
        &self,
        target: &mut impl DrawTarget<Color = C>,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl RenderEnvironment<Color = C>,
    ) {
        let color = env.foreground_color();
        _ = target.fill_solid(
            &Rectangle {
                top_left: origin.into(),
                size: layout.resolved_size.into(),
            },
            color,
        );
    }
}

impl<C: Copy> CharacterRender<C> for Divider {
    fn render(
        &self,
        target: &mut impl CharacterRenderTarget<Color = C>,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl RenderEnvironment<Color = C>,
    ) {
        let color = env.foreground_color();
        match env.layout_direction() {
            LayoutDirection::Horizontal => {
                let height: i16 = layout.resolved_size.height.into();
                for y in origin.y..origin.y + height {
                    target.draw(Point::new(origin.x, y), '|', color);
                }
            }
            LayoutDirection::Vertical => {
                let width: i16 = layout.resolved_size.width.into();
                for x in origin.x..origin.x + width {
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
        let offer = Size::new(100, 100).into();
        let env = TestEnv::<()>::default().with_direction(LayoutDirection::Horizontal);
        let layout = divider.layout(offer, &env);
        assert_eq!(layout.resolved_size, Size::new(2, 100).into());
    }

    #[test]
    fn test_vertical_layout() {
        let divider = Divider::new(2);
        let offer = Size::new(100, 100).into();
        let env = TestEnv::<char>::default().with_direction(LayoutDirection::Vertical);
        let layout = divider.layout(offer, &env);
        assert_eq!(layout.resolved_size, Size::new(100, 2).into());
    }

    #[test]
    fn test_horizontal_render() {
        let divider = Divider::new(1);
        let mut buffer = FixedTextBuffer::<5, 5>::default();
        let env = TestEnv::default().with_direction(LayoutDirection::Horizontal);
        let layout = divider.layout(buffer.size().into(), &env);
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
        let layout = divider.layout(buffer.size().into(), &env);
        divider.render(&mut buffer, &layout, Point::zero(), &env);
        assert_eq!(buffer.text[0][0], '-');
        assert_eq!(buffer.text[0][4], '-');
        assert_eq!(buffer.text[1][0], ' ');
    }
}
