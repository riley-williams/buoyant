use crate::primitives::Point;

use super::{AnimationDomain, CharacterRender, CharacterRenderTarget};

impl<C> CharacterRender<C> for () {
    /// Render the view to the screen
    fn render(
        &self,
        _render_target: &mut impl CharacterRenderTarget<Color = C>,
        _style: &C,
        _offset: Point,
    ) {
    }

    /// Render view and all subviews, animating from a source view to a target view
    fn render_animated(
        _render_target: &mut impl CharacterRenderTarget<Color = C>,
        _source: &Self,
        _target: &Self,
        _style: &C,
        _offset: Point,
        _domain: &AnimationDomain,
    ) {
    }

    /// Produces a new tree by consuming and interpolating between two partially animated trees
    fn join(_source: Self, _target: Self, _domain: &AnimationDomain) -> Self {}
}

#[cfg(feature = "embedded-graphics")]
mod embedded_graphics_rendering {
    use embedded_graphics::prelude::{DrawTarget, PixelColor};

    use crate::{primitives::Point, render::{AnimationDomain, EmbeddedGraphicsRender}};

    impl<C: PixelColor> EmbeddedGraphicsRender<C> for () {
        /// Render the view to the screen
        fn render(
            &self,
            _render_target: &mut impl DrawTarget<Color = C>,
            _style: &C,
            _offset: Point,
        ) {
        }

        /// Render view and all subviews, animating from a source view to a target view
        fn render_animated(
            _render_target: &mut impl DrawTarget<Color = C>,
            _source: &Self,
            _target: &Self,
            _style: &C,
            _offset: Point,
            _domain: &AnimationDomain,
        ) {
        }

        /// Produces a new tree by consuming and interpolating between two partially animated trees
        fn join(_source: Self, _target: Self, _domain: &AnimationDomain) -> Self {}
    }
}
