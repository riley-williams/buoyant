use crate::{
    environment::RenderEnvironment,
    layout::{Layout, ResolvedLayout},
    render_target::CharacterRenderTarget,
};

/// A view that can be rendered to colored characters
pub trait CharacterRender<Pixel: Copy>: Layout {
    /// Render the view to the screen
    fn render(
        &self,
        target: &mut impl CharacterRenderTarget<Color = Pixel>,
        layout: &ResolvedLayout<Self::Sublayout>,
        env: &impl RenderEnvironment<Color = Pixel>,
    );
}

#[cfg(feature = "embedded-graphics")]
/// A view that can be rendered to an embedded-graphics render target
pub trait PixelRender<Pixel: embedded_graphics_core::pixelcolor::PixelColor>: Layout {
    /// Render the view to the screen
    fn render(
        &self,
        target: &mut impl embedded_graphics_core::draw_target::DrawTarget<Color = Pixel>,
        layout: &ResolvedLayout<Self::Sublayout>,
        env: &impl RenderEnvironment<Color = Pixel>,
    );

    // fn render_animated(
    //     target: &mut impl embedded_graphics_core::draw_target::DrawTarget,
    //     source_view: &Self,
    //     source_layout: &ResolvedLayout<Self::Sublayout>,
    //     target_view: &Self,
    //     target_layout: &ResolvedLayout<Self::Sublayout>,
    //     env: &impl RenderEnvironment,
    //     factor: Factor,
    // ) {
    // }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Factor {
    Source,
    Interpolated(u16, f32),
    Target,
}
