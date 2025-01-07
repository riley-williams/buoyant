use embedded_graphics::prelude::PixelColor;

use crate::{
    environment::LayoutEnvironment,
    layout::{Layout, ResolvedLayout},
    primitives::{Point, ProposedDimensions},
    render::Renderable,
};

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct RoundedRectangle {
    corner_radius: u16,
}

impl RoundedRectangle {
    pub fn new(corner_radius: u16) -> Self {
        Self { corner_radius }
    }
}

impl Layout for RoundedRectangle {
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

impl<C: PixelColor> Renderable<C> for RoundedRectangle {
    type Renderables = crate::render::primitives::RoundedRect;

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        _env: &impl LayoutEnvironment,
    ) -> Self::Renderables {
        crate::render::primitives::RoundedRect {
            origin,
            size: layout.resolved_size.into(),
            corner_radius: self.corner_radius,
        }
    }
}
