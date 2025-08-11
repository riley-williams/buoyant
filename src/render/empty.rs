use crate::render_target::RenderTarget;

use super::{AnimatedJoin, AnimationDomain, Render};

impl AnimatedJoin for () {
    fn join_from(&mut self, _source: &Self, _domain: &AnimationDomain) {}
}

impl<C> Render<C> for () {
    fn render(&self, _render_target: &mut impl RenderTarget<ColorFormat = C>, _style: &C) {}

    fn render_animated(
        _render_target: &mut impl RenderTarget<ColorFormat = C>,
        _source: &Self,
        _target: &Self,
        _style: &C,
        _domain: &AnimationDomain,
    ) {
    }
}
