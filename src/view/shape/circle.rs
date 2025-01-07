use embedded_graphics::prelude::PixelColor;

use crate::{
    environment::LayoutEnvironment,
    layout::{Layout, ResolvedLayout},
    primitives::{Dimensions, Point, ProposedDimensions},
    render::Renderable,
};

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Circle;

impl Circle {
    pub fn new() -> Self {
        Self
    }
}

impl Layout for Circle {
    type Sublayout = ();

    fn layout(
        &self,
        offer: &ProposedDimensions,
        _: &impl crate::environment::LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        let minimum_dimension = offer.width.min(offer.height).resolve_most_flexible(0, 10);
        ResolvedLayout {
            sublayouts: (),
            resolved_size: Dimensions {
                width: minimum_dimension,
                height: minimum_dimension,
            },
        }
    }
}

impl<C: PixelColor> Renderable<C> for Circle {
    type Renderables = crate::render::primitives::Circle;

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        _env: &impl LayoutEnvironment,
    ) -> Self::Renderables {
        crate::render::primitives::Circle {
            origin,
            diameter: layout.resolved_size.width.into(),
        }
    }
}
