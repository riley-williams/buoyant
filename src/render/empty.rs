use crate::{primitives::Point, render_target::RenderTarget};

use super::{AnimatedJoin, AnimationDomain, Render};

impl AnimatedJoin for () {
    fn join(_source: Self, _target: Self, _: &AnimationDomain) -> Self {}
}

impl<C> Render<C> for () {
    fn render(
        &self,
        _render_target: &mut impl RenderTarget<ColorFormat = C>,
        _style: &C,
        _offset: Point,
    ) {
    }

    fn render_animated(
        _render_target: &mut impl RenderTarget<ColorFormat = C>,
        _source: &Self,
        _target: &Self,
        _style: &C,
        _offset: Point,
        _domain: &AnimationDomain,
    ) {
    }
}
