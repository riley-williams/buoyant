use crate::{
    environment::LayoutEnvironment,
    layout::{Layout, ResolvedLayout},
    primitives::{Point, ProposedDimensions},
    render::Renderable,
};

use super::RoundedRectangle;

/// A rectangle
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Rectangle;

impl Rectangle {
    /// Adds a corner radius to all corners. The radius will be capped to half the shorter
    /// dimension
    #[must_use]
    pub const fn corner_radius(self, radius: u16) -> RoundedRectangle {
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

impl Renderable for Rectangle {
    type Renderables = crate::render::Rect;

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        _env: &impl LayoutEnvironment,
    ) -> Self::Renderables {
        crate::render::Rect {
            origin,
            size: layout.resolved_size.into(),
        }
    }
}
