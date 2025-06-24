use crate::{
    environment::LayoutEnvironment,
    layout::ResolvedLayout,
    primitives::{Dimensions, Point, ProposedDimensions},
    view::{ViewLayout, ViewMarker},
};

/// A circle primitive
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Circle;

impl ViewMarker for Circle {
    type Renderables = crate::render::Circle;
}

impl<Captures: ?Sized> ViewLayout<Captures> for Circle {
    type State = ();
    type Sublayout = ();

    fn build_state(&self, _captures: &mut Captures) -> Self::State {}

    fn layout(
        &self,
        offer: &ProposedDimensions,
        _: &impl LayoutEnvironment,
        _captures: &mut Captures,
        _state: &mut Self::State,
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

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        _env: &impl LayoutEnvironment,
        _captures: &mut Captures,
        _state: &mut Self::State,
    ) -> Self::Renderables {
        crate::render::Circle {
            origin,
            diameter: layout.resolved_size.width.into(),
        }
    }
}

impl Circle {
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}
