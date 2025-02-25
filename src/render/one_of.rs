use crate::{
    primitives::Point,
    render::{CharacterRender, CharacterRenderTarget},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OneOf2<V0, V1> {
    Variant0(V0),
    Variant1(V1),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OneOf3<V0, V1, V2> {
    Variant0(V0),
    Variant1(V1),
    Variant2(V2),
}

impl<V0, V1, C> CharacterRender<C> for OneOf2<V0, V1>
where
    V0: CharacterRender<C>,
    V1: CharacterRender<C>,
{
    fn render(&self, target: &mut impl CharacterRenderTarget<Color = C>, color: &C, offset: Point) {
        match self {
            Self::Variant0(v0) => v0.render(target, color, offset),
            Self::Variant1(v1) => v1.render(target, color, offset),
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
            (Self::Variant0(source), Self::Variant0(target)) => {
                V0::render_animated(render_target, source, target, style, offset, domain);
            }
            (Self::Variant1(source), Self::Variant1(target)) => {
                V1::render_animated(render_target, source, target, style, offset, domain);
            }
            (_, target) => {
                target.render(render_target, style, offset);
            }
        }
    }

    fn join(source: Self, target: Self, domain: &crate::render::AnimationDomain) -> Self {
        match (source, target) {
            (Self::Variant0(source), Self::Variant0(target)) => {
                Self::Variant0(V0::join(source, target, domain))
            }
            (Self::Variant1(source), Self::Variant1(target)) => {
                Self::Variant1(V1::join(source, target, domain))
            }
            (_, target) => target,
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
            Self::Variant0(v0) => v0.render(target, color, offset),
            Self::Variant1(v1) => v1.render(target, color, offset),
            Self::Variant2(v2) => v2.render(target, color, offset),
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
            (Self::Variant0(source), Self::Variant0(target)) => {
                V0::render_animated(render_target, source, target, style, offset, domain);
            }
            (Self::Variant1(source), Self::Variant1(target)) => {
                V1::render_animated(render_target, source, target, style, offset, domain);
            }
            (Self::Variant2(source), Self::Variant2(target)) => {
                V2::render_animated(render_target, source, target, style, offset, domain);
            }
            (_, target) => {
                target.render(render_target, style, offset);
            }
        }
    }

    fn join(source: Self, target: Self, domain: &crate::render::AnimationDomain) -> Self {
        match (source, target) {
            (Self::Variant0(source), Self::Variant0(target)) => {
                Self::Variant0(V0::join(source, target, domain))
            }
            (Self::Variant1(source), Self::Variant1(target)) => {
                Self::Variant1(V1::join(source, target, domain))
            }
            (Self::Variant2(source), Self::Variant2(target)) => {
                Self::Variant2(V2::join(source, target, domain))
            }
            (_, target) => target,
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
                Self::Variant0(v0) => v0.render(target, color, offset),
                Self::Variant1(v1) => v1.render(target, color, offset),
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
                (Self::Variant0(source), Self::Variant0(target)) => {
                    V0::render_animated(render_target, source, target, style, offset, domain);
                }
                (Self::Variant1(source), Self::Variant1(target)) => {
                    V1::render_animated(render_target, source, target, style, offset, domain);
                }
                (_, target) => {
                    target.render(render_target, style, offset);
                }
            }
        }

        fn join(source: Self, target: Self, domain: &crate::render::AnimationDomain) -> Self {
            match (source, target) {
                (Self::Variant0(source), Self::Variant0(target)) => {
                    Self::Variant0(V0::join(source, target, domain))
                }
                (Self::Variant1(source), Self::Variant1(target)) => {
                    Self::Variant1(V1::join(source, target, domain))
                }
                (_, target) => target,
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
                Self::Variant0(v0) => v0.render(target, color, offset),
                Self::Variant1(v1) => v1.render(target, color, offset),
                Self::Variant2(v2) => v2.render(target, color, offset),
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
                (Self::Variant0(source), Self::Variant0(target)) => {
                    V0::render_animated(render_target, source, target, style, offset, domain);
                }
                (Self::Variant1(source), Self::Variant1(target)) => {
                    V1::render_animated(render_target, source, target, style, offset, domain);
                }
                (Self::Variant2(source), Self::Variant2(target)) => {
                    V2::render_animated(render_target, source, target, style, offset, domain);
                }
                (_, target) => {
                    target.render(render_target, style, offset);
                }
            }
        }

        fn join(source: Self, target: Self, domain: &crate::render::AnimationDomain) -> Self {
            match (source, target) {
                (Self::Variant0(source), Self::Variant0(target)) => {
                    Self::Variant0(V0::join(source, target, domain))
                }
                (Self::Variant1(source), Self::Variant1(target)) => {
                    Self::Variant1(V1::join(source, target, domain))
                }
                (Self::Variant2(source), Self::Variant2(target)) => {
                    Self::Variant2(V2::join(source, target, domain))
                }
                (_, target) => target,
            }
        }
    }
}
