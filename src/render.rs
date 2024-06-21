use crate::{
    environment::Environment, layout::ResolvedLayout, pixel::RenderUnit,
    render_target::RenderTarget,
};

/// A view that can be rendered to pixels
pub trait Render<Pixel: RenderUnit, Sublayout: Clone>: PartialEq {
    /// Render the view to the screen
    fn render(
        &self,
        target: &mut impl RenderTarget<Pixel>,
        layout: &ResolvedLayout<Sublayout>,
        env: &dyn Environment,
    );
}
