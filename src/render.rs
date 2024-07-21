use crate::{
    environment::RenderEnvironment,
    layout::{Layout, ResolvedLayout},
    pixel::PixelColor,
    primitives::Point,
    render_target::RenderTarget,
};

/// A view that can be rendered to pixels
pub trait Render<Pixel: PixelColor>: PartialEq + Layout {
    /// Render the view to the screen
    fn render(
        &self,
        target: &mut impl RenderTarget<Color = Pixel>,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl RenderEnvironment<Pixel>,
    );
}
