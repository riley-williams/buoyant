use crate::{
    environment::LayoutEnvironment,
    layout::{Layout, ResolvedLayout},
    primitives::{Dimensions, Point, ProposedDimensions},
    render::Renderable,
};

/// A circle primitive
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Circle;

impl Circle {
    #[must_use]
    pub const fn new() -> Self {
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
        let minimum_dimension = offer.width.min(offer.height).resolve_most_flexible(0, 1);
        ResolvedLayout {
            sublayouts: (),
            resolved_size: Dimensions {
                width: minimum_dimension,
                height: minimum_dimension,
            },
        }
    }
}

impl Renderable for Circle {
    type Renderables = crate::render::Circle;

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        _env: &impl LayoutEnvironment,
    ) -> Self::Renderables {
        crate::render::Circle {
            origin,
            diameter: layout.resolved_size.width.into(),
        }
    }
}
