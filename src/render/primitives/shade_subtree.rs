use crate::{
    render::{shade::Shader, AnimationDomain, Render},
    render_target::RenderTarget,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShadeSubtree<S, T> {
    shader: S,
    subtree: T,
}

impl<S, T> ShadeSubtree<S, T> {
    pub fn new(shader: S, subtree: T) -> Self {
        Self { shader, subtree }
    }
}

impl<C, S: Shader<Color = C> + Clone, T: Render<C>> Render<C> for ShadeSubtree<S, T> {
    fn render(&self, render_target: &mut impl RenderTarget<Color = C>, _: &impl Shader<Color = C>) {
        self.subtree.render(render_target, &self.shader);
    }

    fn render_animated(
        render_target: &mut impl RenderTarget<Color = C>,
        source: &Self,
        _: &impl Shader<Color = C>,
        target: &Self,
        _: &impl Shader<Color = C>,
        config: &AnimationDomain,
    ) {
        T::render_animated(
            render_target,
            &source.subtree,
            &source.shader,
            &target.subtree,
            &target.shader,
            config,
        );
    }

    fn join(source: Self, target: Self, config: &AnimationDomain) -> Self {
        Self {
            // TODO: This "jumps" to the target shader, should be an interpolated intermetiate
            shader: target.shader,
            subtree: T::join(source.subtree, target.subtree, config),
        }
    }
}
