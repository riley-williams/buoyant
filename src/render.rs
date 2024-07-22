use crate::{
    environment::RenderEnvironment,
    layout::{Layout, ResolvedLayout},
    pixel::PixelColor,
    primitives::Point,
    render_target::CharacterRenderTarget,
};

/// A view that can be rendered to colored characters
pub trait CharacterRender<Pixel: PixelColor>: PartialEq + Layout {
    /// Render the view to the screen
    fn render(
        &self,
        target: &mut impl CharacterRenderTarget<Color = Pixel>,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl RenderEnvironment<Pixel>,
    );
}

#[cfg(feature = "embedded-graphics")]
/// A view that can be rendered to an embedded-graphics render target
pub trait EmbeddedRender<Pixel: PixelColor>: PartialEq + Layout {
    /// Render the view to the screen
    fn render(
        &self,
        target: &mut impl embedded_graphics_core::draw_target::DrawTarget<Color = Pixel>,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl RenderEnvironment<Pixel>,
    );
}
