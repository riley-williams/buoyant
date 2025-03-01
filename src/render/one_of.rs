use crate::{
    primitives::Point,
    render::{CharacterRender, CharacterRenderTarget},
};

use super::AnimatedJoin;

#[derive(Debug, Clone, PartialEq)]
pub enum OneOf2<V0, V1> {
    Variant0(V0),
    Variant1(V1),
}

#[derive(Debug, Clone, PartialEq)]
pub enum OneOf3<V0, V1, V2> {
    Variant0(V0),
    Variant1(V1),
    Variant2(V2),
}

impl<V0: AnimatedJoin, V1: AnimatedJoin> AnimatedJoin for OneOf2<V0, V1> {
    fn join(source: Self, target: Self, domain: &crate::render::AnimationDomain) -> Self {
        match (source, target) {
            (OneOf2::Variant0(source), OneOf2::Variant0(target)) => {
                OneOf2::Variant0(V0::join(source, target, domain))
            }
            (OneOf2::Variant1(source), OneOf2::Variant1(target)) => {
                OneOf2::Variant1(V1::join(source, target, domain))
            }
            (_, target) => target,
        }
    }
}

impl<V0: AnimatedJoin, V1: AnimatedJoin, V2: AnimatedJoin> AnimatedJoin for OneOf3<V0, V1, V2> {
    fn join(source: Self, target: Self, domain: &crate::render::AnimationDomain) -> Self {
        match (source, target) {
            (OneOf3::Variant0(source), OneOf3::Variant0(target)) => {
                OneOf3::Variant0(V0::join(source, target, domain))
            }
            (OneOf3::Variant1(source), OneOf3::Variant1(target)) => {
                OneOf3::Variant1(V1::join(source, target, domain))
            }
            (OneOf3::Variant2(source), OneOf3::Variant2(target)) => {
                OneOf3::Variant2(V2::join(source, target, domain))
            }
            (_, target) => target,
        }
    }
}

impl<V0, V1, C> CharacterRender<C> for OneOf2<V0, V1>
where
    V0: CharacterRender<C>,
    V1: CharacterRender<C>,
{
    fn render(&self, target: &mut impl CharacterRenderTarget<Color = C>, color: &C, offset: Point) {
        match self {
            OneOf2::Variant0(v0) => v0.render(target, color, offset),
            OneOf2::Variant1(v1) => v1.render(target, color, offset),
        }
    }

    fn render_animated(
        render_target: &mut impl CharacterRenderTarget<Color = C>,
        source: &Self,
        target: &Self,
        style: &C,
        offset: Point,

        domain: &crate::render::AnimationDomain,
    ) {
        match (source, target) {
            (OneOf2::Variant0(source), OneOf2::Variant0(target)) => {
                V0::render_animated(render_target, source, target, style, offset, domain);
            }
            (OneOf2::Variant1(source), OneOf2::Variant1(target)) => {
                V1::render_animated(render_target, source, target, style, offset, domain);
            }
            (_, target) => {
                target.render(render_target, style, offset);
            }
        }
    }
}

impl<V0, V1, V2, C> CharacterRender<C> for OneOf3<V0, V1, V2>
where
    V0: CharacterRender<C>,
    V1: CharacterRender<C>,
    V2: CharacterRender<C>,
{
    fn render(&self, target: &mut impl CharacterRenderTarget<Color = C>, color: &C, offset: Point) {
        match self {
            OneOf3::Variant0(v0) => v0.render(target, color, offset),
            OneOf3::Variant1(v1) => v1.render(target, color, offset),
            OneOf3::Variant2(v2) => v2.render(target, color, offset),
        }
    }

    fn render_animated(
        render_target: &mut impl CharacterRenderTarget<Color = C>,
        source: &Self,
        target: &Self,
        style: &C,
        offset: Point,

        domain: &crate::render::AnimationDomain,
    ) {
        match (source, target) {
            (OneOf3::Variant0(source), OneOf3::Variant0(target)) => {
                V0::render_animated(render_target, source, target, style, offset, domain);
            }
            (OneOf3::Variant1(source), OneOf3::Variant1(target)) => {
                V1::render_animated(render_target, source, target, style, offset, domain);
            }
            (OneOf3::Variant2(source), OneOf3::Variant2(target)) => {
                V2::render_animated(render_target, source, target, style, offset, domain);
            }
            (_, target) => {
                target.render(render_target, style, offset);
            }
        }
    }
}

#[cfg(feature = "embedded-graphics")]
mod embedded_graphics_render {
    use embedded_graphics::prelude::{DrawTarget, PixelColor};

    use crate::{primitives::Point, render::EmbeddedGraphicsRender};

    use super::{OneOf2, OneOf3};

    impl<V0, V1, C> EmbeddedGraphicsRender<C> for OneOf2<V0, V1>
    where
        V0: EmbeddedGraphicsRender<C>,
        V1: EmbeddedGraphicsRender<C>,
        C: PixelColor,
    {
        fn render(&self, target: &mut impl DrawTarget<Color = C>, color: &C, offset: Point) {
            match self {
                OneOf2::Variant0(v0) => v0.render(target, color, offset),
                OneOf2::Variant1(v1) => v1.render(target, color, offset),
            }
        }

        fn render_animated(
            render_target: &mut impl DrawTarget<Color = C>,
            source: &Self,
            target: &Self,
            style: &C,
            offset: Point,

            domain: &crate::render::AnimationDomain,
        ) {
            match (source, target) {
                (OneOf2::Variant0(source), OneOf2::Variant0(target)) => {
                    V0::render_animated(render_target, source, target, style, offset, domain);
                }
                (OneOf2::Variant1(source), OneOf2::Variant1(target)) => {
                    V1::render_animated(render_target, source, target, style, offset, domain);
                }
                (_, target) => {
                    target.render(render_target, style, offset);
                }
            }
        }
    }

    impl<V0, V1, V2, C> EmbeddedGraphicsRender<C> for OneOf3<V0, V1, V2>
    where
        V0: EmbeddedGraphicsRender<C>,
        V1: EmbeddedGraphicsRender<C>,
        V2: EmbeddedGraphicsRender<C>,
        C: PixelColor,
    {
        fn render(&self, target: &mut impl DrawTarget<Color = C>, color: &C, offset: Point) {
            match self {
                OneOf3::Variant0(v0) => v0.render(target, color, offset),
                OneOf3::Variant1(v1) => v1.render(target, color, offset),
                OneOf3::Variant2(v2) => v2.render(target, color, offset),
            }
        }

        fn render_animated(
            render_target: &mut impl DrawTarget<Color = C>,
            source: &Self,
            target: &Self,
            style: &C,
            offset: Point,

            domain: &crate::render::AnimationDomain,
        ) {
            match (source, target) {
                (OneOf3::Variant0(source), OneOf3::Variant0(target)) => {
                    V0::render_animated(render_target, source, target, style, offset, domain);
                }
                (OneOf3::Variant1(source), OneOf3::Variant1(target)) => {
                    V1::render_animated(render_target, source, target, style, offset, domain);
                }
                (OneOf3::Variant2(source), OneOf3::Variant2(target)) => {
                    V2::render_animated(render_target, source, target, style, offset, domain);
                }
                (_, target) => {
                    target.render(render_target, style, offset);
                }
            }
        }
    }
}
