use crate::{
    primitives::Point,
    render::{Render, RenderTarget},
};

use super::AnimatedJoin;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OneOf4<V0, V1, V2, V3> {
    Variant0(V0),
    Variant1(V1),
    Variant2(V2),
    Variant3(V3),
}

impl<V0: AnimatedJoin, V1: AnimatedJoin> AnimatedJoin for OneOf2<V0, V1> {
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

impl<V0: AnimatedJoin, V1: AnimatedJoin, V2: AnimatedJoin> AnimatedJoin for OneOf3<V0, V1, V2> {
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

impl<V0: AnimatedJoin, V1: AnimatedJoin, V2: AnimatedJoin, V3: AnimatedJoin> AnimatedJoin
    for OneOf4<V0, V1, V2, V3>
{
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
            (Self::Variant3(source), Self::Variant3(target)) => {
                Self::Variant3(V3::join(source, target, domain))
            }
            (_, target) => target,
        }
    }
}

impl<V0, V1, C> Render<C> for OneOf2<V0, V1>
where
    V0: Render<C>,
    V1: Render<C>,
{
    fn render(&self, target: &mut impl RenderTarget<ColorFormat = C>, color: &C, offset: Point) {
        match self {
            Self::Variant0(v0) => v0.render(target, color, offset),
            Self::Variant1(v1) => v1.render(target, color, offset),
        }
    }

    fn render_animated(
        render_target: &mut impl RenderTarget<ColorFormat = C>,
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
}

impl<V0, V1, V2, C> Render<C> for OneOf3<V0, V1, V2>
where
    V0: Render<C>,
    V1: Render<C>,
    V2: Render<C>,
{
    fn render(&self, target: &mut impl RenderTarget<ColorFormat = C>, color: &C, offset: Point) {
        match self {
            Self::Variant0(v0) => v0.render(target, color, offset),
            Self::Variant1(v1) => v1.render(target, color, offset),
            Self::Variant2(v2) => v2.render(target, color, offset),
        }
    }

    fn render_animated(
        render_target: &mut impl RenderTarget<ColorFormat = C>,
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
}

impl<V0, V1, V2, V3, C> Render<C> for OneOf4<V0, V1, V2, V3>
where
    V0: Render<C>,
    V1: Render<C>,
    V2: Render<C>,
    V3: Render<C>,
{
    fn render(&self, target: &mut impl RenderTarget<ColorFormat = C>, color: &C, offset: Point) {
        match self {
            Self::Variant0(v0) => v0.render(target, color, offset),
            Self::Variant1(v1) => v1.render(target, color, offset),
            Self::Variant2(v2) => v2.render(target, color, offset),
            Self::Variant3(v3) => v3.render(target, color, offset),
        }
    }

    fn render_animated(
        render_target: &mut impl RenderTarget<ColorFormat = C>,
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
            (Self::Variant3(source), Self::Variant3(target)) => {
                V3::render_animated(render_target, source, target, style, offset, domain);
            }
            (_, target) => {
                target.render(render_target, style, offset);
            }
        }
    }
}
