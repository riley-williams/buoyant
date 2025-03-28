use crate::{
    environment::LayoutEnvironment,
    layout::{Layout, ResolvedLayout},
    primitives::{Dimensions, Point, ProposedDimensions},
    render::Renderable,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmptyView;

impl Layout for EmptyView {
    type Sublayout = ();

    fn layout(
        &self,
        _: &ProposedDimensions,
        _: &impl crate::environment::LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        ResolvedLayout {
            sublayouts: (),
            resolved_size: Dimensions::zero(),
        }
    }

    fn priority(&self) -> i8 {
        i8::MIN
    }

    fn is_empty(&self) -> bool {
        true
    }
}

impl Renderable for EmptyView {
    type Renderables = ();

    fn render_tree(
        &self,
        _layout: &ResolvedLayout<Self::Sublayout>,
        _origin: Point,
        _env: &impl LayoutEnvironment,
    ) {
    }
}

// The empty tuple can be used as an equivalent to empty view

impl Layout for () {
    type Sublayout = ();

    fn layout(
        &self,
        _: &ProposedDimensions,
        _: &impl crate::environment::LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        ResolvedLayout {
            sublayouts: (),
            resolved_size: Dimensions::zero(),
        }
    }

    fn priority(&self) -> i8 {
        i8::MIN
    }

    fn is_empty(&self) -> bool {
        true
    }
}

impl Renderable for () {
    type Renderables = ();

    fn render_tree(
        &self,
        _layout: &ResolvedLayout<Self::Sublayout>,
        _origin: Point,
        _env: &impl LayoutEnvironment,
    ) {
    }
}
