use shade::Shader;

use crate::{
    environment::LayoutEnvironment,
    layout::{Layout, ResolvedLayout},
    primitives::Point,
    render_target::RenderTarget,
};

pub mod collections;
pub mod primitives;
pub mod shade;

pub trait Renderable<Color>: Layout
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

impl<C, T: NullRender + Layout> Renderable<C> for T {
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
pub trait Render<Color>: Sized + Clone {
    /// Render the view to the screen
    fn render(
        &self,
        render_target: &mut impl RenderTarget<Color = Color>,
        shader: &impl Shader<Color = Color>,
    );

    /// Render view and all subviews, animating from a source view to a target view
    fn render_animated(
        render_target: &mut impl RenderTarget<Color = Color>,
        source: &Self,
        _source_shader: &impl Shader<Color = Color>,
        target: &Self,
        target_shader: &impl Shader<Color = Color>,
        config: &AnimationDomain,
    ) {
        let intermediate = Self::join(source.clone(), target.clone(), config);
        // TODO: interpolate shaders
        intermediate.render(render_target, target_shader);
    }

    /// Produces a new tree by consuming and interpolating between two partially animated trees
    fn join(source: Self, target: Self, config: &AnimationDomain) -> Self;
}

impl<C> Render<C> for () {
    fn render(
        &self,
        _render_target: &mut impl RenderTarget<Color = C>,
        _shader: &impl Shader<Color = C>,
    ) {
    }

    fn render_animated(
        _render_target: &mut impl RenderTarget<Color = C>,
        _source: &Self,
        _source_shader: &impl Shader<Color = C>,
        _target: &Self,
        _target_shader: &impl Shader<Color = C>,
        _config: &AnimationDomain,
    ) {
    }

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
