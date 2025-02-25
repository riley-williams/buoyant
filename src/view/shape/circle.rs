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
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Layout for Circle {
    type Sublayout = ();

    #[inline]
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

impl<C> Renderable<C> for Circle {
    type Renderables = crate::render::Circle;

    #[inline]
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
