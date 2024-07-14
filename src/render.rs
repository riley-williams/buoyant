use crate::{
    environment::RenderEnvironment, layout::ResolvedLayout, pixel::ColorValue, primitives::Point,
    render_target::RenderTarget,
};

/// A view that can be rendered to pixels
pub trait Render<Pixel: ColorValue, Sublayout: Clone>: PartialEq {
    /// Render the view to the screen
    fn render(
        &self,
        target: &mut impl RenderTarget<Pixel>,
        layout: &ResolvedLayout<Sublayout>,
        origin: Point,
        env: &impl RenderEnvironment<Pixel>,
    );
}
