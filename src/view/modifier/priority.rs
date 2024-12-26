use crate::{
    environment::{LayoutEnvironment, RenderEnvironment},
    layout::{Layout, ProposedDimensions, ResolvedLayout},
    primitives::Point,
    render::CharacterRender,
    render_target::CharacterRenderTarget,
};

/// A view that adds padding around a child view.
/// When the space offered to the padding is less than 2* the padding, the padding will
/// not be truncated and will return a size larger than the offer.
pub struct Priority<T> {
    priority: u16,
    child: T,
}

impl<T> Priority<T> {
    pub fn new(priority: u16, child: T) -> Self {
        Self { priority, child }
    }
}

impl<T> PartialEq for Priority<T> {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl<V: Layout> Layout for Priority<V> {
    type Sublayout = V::Sublayout;

    fn layout(
        &self,
        offer: ProposedDimensions,
        env: &impl LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        self.child.layout(offer, env)
    }
}

impl<Pixel: Copy, View: Layout> CharacterRender<Pixel> for Priority<View>
where
    View: CharacterRender<Pixel>,
{
    fn render(
        &self,
        target: &mut impl CharacterRenderTarget<Color = Pixel>,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl RenderEnvironment<Color = Pixel>,
    ) {
        self.child.render(target, layout, origin, env);
    }
}

#[cfg(feature = "embedded-graphics")]
use embedded_graphics::draw_target::DrawTarget;

#[cfg(feature = "embedded-graphics")]
impl<Pixel, View: Layout> crate::render::PixelRender<Pixel> for Priority<View>
where
    View: crate::render::PixelRender<Pixel>,
    Pixel: embedded_graphics_core::pixelcolor::PixelColor,
{
    fn render(
        &self,
        target: &mut impl DrawTarget<Color = Pixel>,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl RenderEnvironment<Color = Pixel>,
    ) {
        self.child.render(target, layout, origin, env);
    }
}
