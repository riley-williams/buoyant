use crate::{
    environment::{LayoutEnvironment, RenderEnvironment},
    layout::{Layout, ResolvedLayout},
    primitives::{Point, ProposedDimensions},
    render::{AnimationConfiguration, CharacterRender},
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
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        self.child.layout(offer, env)
    }

    fn place_subviews(
        &self,
        layout: &mut ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl LayoutEnvironment,
    ) {
        self.child.place_subviews(layout, origin, env);
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
        env: &impl RenderEnvironment<Color = Pixel>,
    ) {
        self.child.render(target, layout, env);
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
        env: &impl RenderEnvironment<Color = Pixel>,
    ) {
        self.child.render(target, layout, env);
    }

    fn render_animated(
        target: &mut impl embedded_graphics_core::draw_target::DrawTarget<Color = Pixel>,
        source_view: &Self,
        source_layout: &ResolvedLayout<Self::Sublayout>,
        target_view: &Self,
        target_layout: &ResolvedLayout<Self::Sublayout>,
        source_env: &impl RenderEnvironment<Color = Pixel>,
        target_env: &impl RenderEnvironment<Color = Pixel>,
        config: &AnimationConfiguration,
    ) {
        crate::render::PixelRender::render_animated(
            target,
            &source_view.child,
            source_layout,
            &target_view.child,
            target_layout,
            source_env,
            target_env,
            config,
        );
    }
}
