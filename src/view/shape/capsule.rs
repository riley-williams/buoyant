use crate::{
    environment::LayoutEnvironment,
    layout::{Layout, ResolvedLayout},
    primitives::{Point, ProposedDimensions},
    render::Renderable,
};

/// A capsule primitive, oriented horizontally
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Capsule;

impl Layout for Capsule {
    type Sublayout = ();

    #[inline]
    fn layout(
        &self,
        offer: &ProposedDimensions,
        _: &impl crate::environment::LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        ResolvedLayout {
            sublayouts: (),
            resolved_size: offer.resolve_most_flexible(0, 10),
        }
    }
}

impl<C> Renderable<C> for Capsule {
    type Renderables = crate::render::Capsule;

    #[inline]
    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        _env: &impl LayoutEnvironment,
    ) -> Self::Renderables {
        crate::render::Capsule::new(origin, layout.resolved_size.into())
    }
}
