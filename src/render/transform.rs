use crate::{
    primitives::{transform::LinearTransform, Interpolate as _},
    render::{AnimatedJoin, AnimationDomain, Render},
};

/// Applies the provided linear transform to the inner render tree
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Transform<T> {
    pub inner: T,
    pub transform: LinearTransform,
}

impl<T> Transform<T> {
    #[must_use]
    pub fn new(inner: T, transform: LinearTransform) -> Self {
        Self { inner, transform }
    }
}

impl<T: AnimatedJoin> AnimatedJoin for Transform<T> {
    fn join_from(&mut self, source: &Self, domain: &AnimationDomain) {
        self.transform = LinearTransform::interpolate(
            source.transform.clone(),
            self.transform.clone(),
            domain.factor,
        );
        self.inner.join_from(&source.inner, domain);
    }
}

impl<T: Render<C>, C: Copy> Render<C> for Transform<T> {
    fn render(
        &self,
        render_target: &mut impl crate::render_target::RenderTarget<ColorFormat = C>,
        style: &C,
    ) {
        render_target.with_layer(
            |l| l.transform(&self.transform),
            |target| {
                self.inner.render(target, style);
            },
        );
    }

    fn render_animated(
        render_target: &mut impl crate::render_target::RenderTarget<ColorFormat = C>,
        source: &Self,
        target: &Self,
        style: &C,
        domain: &AnimationDomain,
    ) {
        let transform = LinearTransform::interpolate(
            source.transform.clone(),
            target.transform.clone(),
            domain.factor,
        );
        render_target.with_layer(
            |l| l.transform(&transform),
            |render_target| {
                Render::render_animated(render_target, &source.inner, &target.inner, style, domain);
            },
        );
    }
}
