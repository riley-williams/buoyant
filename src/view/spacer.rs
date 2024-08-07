use crate::{
    environment::{LayoutEnvironment, RenderEnvironment},
    layout::{Layout, LayoutDirection, ResolvedLayout},
    primitives::{Point, Size},
    render::CharacterRender,
    render_target::CharacterRenderTarget,
};

#[derive(Default, PartialEq)]
pub struct Spacer {
    min_length: u16,
}

impl Layout for Spacer {
    type Sublayout = ();
    fn layout(&self, offer: Size, env: &impl LayoutEnvironment) -> ResolvedLayout<()> {
        let size = match env.layout_direction() {
            LayoutDirection::Horizontal => {
                Size::new(core::cmp::max(offer.width, self.min_length), 0)
            }
            LayoutDirection::Vertical => {
                Size::new(0, core::cmp::max(offer.height, self.min_length))
            }
        };
        ResolvedLayout {
            sublayouts: (),
            resolved_size: size,
        }
    }

    fn priority(&self) -> i8 {
        // This view should take all the remaining space after other siblings have been laid out
        i8::MIN
    }
}

impl<Pixel: Copy> CharacterRender<Pixel> for Spacer {
    fn render(
        &self,
        _target: &mut impl CharacterRenderTarget<Color = Pixel>,
        _layout: &ResolvedLayout<Self::Sublayout>,
        _origin: Point,
        _env: &impl RenderEnvironment<Color = Pixel>,
    ) {
    }
}

#[cfg(feature = "embedded-graphics")]
impl<Pixel: embedded_graphics_core::pixelcolor::PixelColor> crate::render::PixelRender<Pixel>
    for Spacer
{
    fn render(
        &self,
        _target: &mut impl embedded_graphics_core::draw_target::DrawTarget<Color = Pixel>,
        _layout: &ResolvedLayout<Self::Sublayout>,
        _origin: Point,
        _env: &impl RenderEnvironment<Color = Pixel>,
    ) {
    }
}
