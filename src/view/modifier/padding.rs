use crate::{
    environment::{LayoutEnvironment, RenderEnvironment},
    layout::{Layout, ResolvedLayout},
    pixel::PixelColor,
    primitives::{Point, Size},
    render::CharacterRender,
    render_target::CharacterRenderTarget,
};

/// A view that adds padding around a child view.
/// When the space offered to the padding is less than 2* the padding, the padding will
/// not be truncated and will return a size larger than the offer.
pub struct Padding<T> {
    padding: u16,
    child: T,
}

impl<T> Padding<T> {
    pub fn new(padding: u16, child: T) -> Self {
        Self { padding, child }
    }
}

impl<T> PartialEq for Padding<T> {
    fn eq(&self, other: &Self) -> bool {
        self.padding == other.padding
    }
}

impl<V: Layout> Layout for Padding<V> {
    type Sublayout = ResolvedLayout<V::Sublayout>;

    fn layout(&self, offer: Size, env: &impl LayoutEnvironment) -> ResolvedLayout<Self::Sublayout> {
        let padded_offer = Size::new(
            offer.width.saturating_sub(2 * self.padding),
            offer.height.saturating_sub(2 * self.padding),
        );
        let child_layout = self.child.layout(padded_offer, env);
        let padding_size =
            child_layout.resolved_size + Size::new(2 * self.padding, 2 * self.padding);
        ResolvedLayout {
            sublayouts: child_layout,
            resolved_size: padding_size,
        }
    }
}

impl<Pixel, View: Layout> CharacterRender<Pixel> for Padding<View>
where
    View: CharacterRender<Pixel>,
    Pixel: PixelColor,
{
    fn render(
        &self,
        target: &mut impl CharacterRenderTarget<Color = Pixel>,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl RenderEnvironment<Pixel>,
    ) {
        let offset_origin = origin + Point::new(self.padding as i16, self.padding as i16);
        self.child
            .render(target, &layout.sublayouts, offset_origin, env);
    }
}

#[cfg(feature = "embedded-graphics")]
use embedded_graphics::draw_target::DrawTarget;

#[cfg(feature = "embedded-graphics")]
impl<Pixel, View: Layout> crate::render::EmbeddedRender<Pixel> for Padding<View>
where
    View: crate::render::EmbeddedRender<Pixel>,
    Pixel: PixelColor,
{
    fn render(
        &self,
        target: &mut impl DrawTarget<Color = Pixel>,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl RenderEnvironment<Pixel>,
    ) {
        let offset_origin = origin + Point::new(self.padding as i16, self.padding as i16);
        self.child
            .render(target, &layout.sublayouts, offset_origin, env);
    }
}
