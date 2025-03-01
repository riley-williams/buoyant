use crate::primitives::Point;

use super::{AnimationDomain, CharacterRender, CharacterRenderTarget};

impl<C> CharacterRender<C> for () {
    fn render(
        &self,
        _render_target: &mut impl CharacterRenderTarget<Color = C>,
        _style: &C,
        _offset: Point,
    ) {
    }

    fn render_animated(
        _render_target: &mut impl CharacterRenderTarget<Color = C>,
        _source: &Self,
        _target: &Self,
        _style: &C,
        _offset: Point,
        _domain: &AnimationDomain,
    ) {
    }

    fn join(_source: Self, _target: Self, _domain: &AnimationDomain) -> Self {}
}

#[cfg(feature = "embedded-graphics")]
mod embedded_graphics_rendering {
    use embedded_graphics::prelude::{DrawTarget, PixelColor};

    use crate::{
        primitives::Point,
        render::{AnimationDomain, EmbeddedGraphicsRender},
    };

    impl<C: PixelColor> EmbeddedGraphicsRender<C> for () {
        fn render(
            &self,
            _render_target: &mut impl DrawTarget<Color = C>,
            _style: &C,
            _offset: Point,
        ) {
        }

        fn render_animated(
            _render_target: &mut impl DrawTarget<Color = C>,
            _source: &Self,
            _target: &Self,
            _style: &C,
            _offset: Point,
            _domain: &AnimationDomain,
        ) {
        }

        fn join(_source: Self, _target: Self, _domain: &AnimationDomain) -> Self {}
    }
}
