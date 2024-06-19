use crate::{environment::Environment, layout::ResolvedLayout, render_target::RenderTarget};

/// A view that can be rendered to pixels
pub trait Render<Pixel, Sublayout> {
    /// Render the view to the screen
    fn render(
        &self,
        target: &mut impl RenderTarget<Pixel>,
        layout: &ResolvedLayout<Sublayout>,
        env: &dyn Environment,
    );
}
