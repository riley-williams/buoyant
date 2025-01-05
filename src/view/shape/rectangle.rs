use crate::{
    environment::LayoutEnvironment,
    layout::{Layout, ResolvedLayout},
    primitives::{Point, ProposedDimensions},
    render::Renderable,
};

use super::RoundedRectangle;

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Rectangle;

impl Rectangle {
    pub fn corner_radius(self, radius: u16) -> RoundedRectangle {
        RoundedRectangle::new(radius)
    }
}

impl Layout for Rectangle {
    type Sublayout = ();

    fn layout(
        &self,
        offer: &ProposedDimensions,
        _: &impl LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        ResolvedLayout {
            sublayouts: (),
            resolved_size: offer.resolve_most_flexible(0, 10),
        }
    }
}

impl<C> Renderable<C> for Rectangle {
    type Renderables = crate::render::primitives::Rect;

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        _env: &impl LayoutEnvironment,
    ) -> Self::Renderables {
        crate::render::primitives::Rect {
            origin,
            size: layout.resolved_size.into(),
        }
    }
}
