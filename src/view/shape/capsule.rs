use embedded_graphics::prelude::PixelColor;

use crate::{
    environment::LayoutEnvironment,
    layout::{Layout, ResolvedLayout},
    primitives::{Point, ProposedDimensions},
    render::Renderable,
};

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Capsule;

impl Layout for Capsule {
    type Sublayout = ();

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

impl<C: PixelColor> Renderable<C> for Capsule {
    type Renderables = crate::render::primitives::Capsule;

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        _env: &impl LayoutEnvironment,
    ) -> Self::Renderables {
        crate::render::primitives::Capsule::new(origin, layout.resolved_size.into())
    }
}
