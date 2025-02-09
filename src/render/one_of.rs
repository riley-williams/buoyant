use crate::render::{CharacterRender, CharacterRenderTarget};

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

impl<V0, V1, C> CharacterRender<C> for OneOf2<V0, V1>
where
    V0: CharacterRender<C>,
    V1: CharacterRender<C>,
{
    fn render(&self, target: &mut impl CharacterRenderTarget<Color = C>, color: &C) {
        match self {
            OneOf2::Variant0(v0) => v0.render(target, color),
            OneOf2::Variant1(v1) => v1.render(target, color),
        }
    }

    fn join(_source: Self, target: Self, _domain: &crate::render::AnimationDomain) -> Self {
        // jump to target
        target
    }
}

impl<V0, V1, V2, C> CharacterRender<C> for OneOf3<V0, V1, V2>
where
    V0: CharacterRender<C>,
    V1: CharacterRender<C>,
    V2: CharacterRender<C>,
{
    fn render(&self, target: &mut impl CharacterRenderTarget<Color = C>, color: &C) {
        match self {
            OneOf3::Variant0(v0) => v0.render(target, color),
            OneOf3::Variant1(v1) => v1.render(target, color),
            OneOf3::Variant2(v2) => v2.render(target, color),
        }
    }

    fn join(_source: Self, target: Self, _domain: &crate::render::AnimationDomain) -> Self {
        // jump to target
        target
    }
}

#[cfg(feature = "embedded-graphics")]
mod embedded_graphics_render {
    use embedded_graphics::prelude::{DrawTarget, PixelColor};

    use crate::render::EmbeddedGraphicsRender;

    use super::{OneOf2, OneOf3};

    impl<V0, V1, C> EmbeddedGraphicsRender<C> for OneOf2<V0, V1>
    where
        V0: EmbeddedGraphicsRender<C>,
        V1: EmbeddedGraphicsRender<C>,
        C: PixelColor,
    {
        fn render(&self, target: &mut impl DrawTarget<Color = C>, color: &C) {
            match self {
                OneOf2::Variant0(v0) => v0.render(target, color),
                OneOf2::Variant1(v1) => v1.render(target, color),
            }
        }

        fn join(_source: Self, target: Self, _domain: &crate::render::AnimationDomain) -> Self {
            // jump to target
            target
        }
    }

    impl<V0, V1, V2, C> EmbeddedGraphicsRender<C> for OneOf3<V0, V1, V2>
    where
        V0: EmbeddedGraphicsRender<C>,
        V1: EmbeddedGraphicsRender<C>,
        V2: EmbeddedGraphicsRender<C>,
        C: PixelColor,
    {
        fn render(&self, target: &mut impl DrawTarget<Color = C>, color: &C) {
            match self {
                OneOf3::Variant0(v0) => v0.render(target, color),
                OneOf3::Variant1(v1) => v1.render(target, color),
                OneOf3::Variant2(v2) => v2.render(target, color),
            }
        }

        fn join(_source: Self, target: Self, _domain: &crate::render::AnimationDomain) -> Self {
            // jump to target
            target
        }
    }
}
