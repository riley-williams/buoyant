use crate::{
    environment::LayoutEnvironment,
    layout::{Layout, ResolvedLayout},
    primitives::Point,
};
use embedded_graphics::{prelude::PixelColor, primitives::PrimitiveStyle};
use embedded_graphics_core::draw_target::DrawTarget;

pub mod collections;
pub mod primitives;

pub trait Renderable<Color: PixelColor>: Layout
where
    Self::Renderables: Render<Color>,
{
    type Renderables;
    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl LayoutEnvironment,
    ) -> Self::Renderables;
}

pub trait NullRender {}

impl<C: PixelColor, T: NullRender + Layout> Renderable<C> for T {
    type Renderables = ();

    fn render_tree(
        &self,
        _layout: &ResolvedLayout<Self::Sublayout>,
        _origin: Point,
        _env: &impl LayoutEnvironment,
    ) {
    }
}

/// A view that can be rendered to a target
pub trait Render<Color: PixelColor>: Sized + Clone {
    /// Render the view to the screen
    fn render(
        &self,
        render_target: &mut impl DrawTarget<Color = Color>,
        style: &PrimitiveStyle<Color>,
    );

    /// Render view and all subviews, animating from a source view to a target view
    fn render_animated(
        render_target: &mut impl DrawTarget<Color = Color>,
        source: &Self,
        _source_style: &PrimitiveStyle<Color>,
        target: &Self,
        target_style: &PrimitiveStyle<Color>,
        config: &AnimationDomain,
    ) {
        let intermediate = Self::join(source.clone(), target.clone(), config);
        // TODO: interpolate styles
        intermediate.render(render_target, target_style);
    }

    /// Produces a new tree by consuming and interpolating between two partially animated trees
    fn join(source: Self, target: Self, config: &AnimationDomain) -> Self;
}

impl<C: PixelColor> Render<C> for () {
    /// Render the view to the screen
    fn render(&self, _render_target: &mut impl DrawTarget<Color = C>, _style: &PrimitiveStyle<C>) {}

    /// Render view and all subviews, animating from a source view to a target view
    fn render_animated(
        _render_target: &mut impl DrawTarget<Color = C>,
        _source: &Self,
        _source_style: &PrimitiveStyle<C>,
        _target: &Self,
        _target_style: &PrimitiveStyle<C>,
        _config: &AnimationDomain,
    ) {
    }

    /// Produces a new tree by consuming and interpolating between two partially animated trees
    fn join(_source: Self, _target: Self, _config: &AnimationDomain) -> Self {}
}

#[derive(Debug, Clone, PartialEq)]
pub struct AnimationDomain {
    /// The animation factor of this domain, ranging from 0 to 255
    pub factor: u8,
    /// The time elapsed in milliseconds from when the animation started
    /// This is primarily useful for creating a subdomain with a different speed
    pub offset_ms: u64,
}
