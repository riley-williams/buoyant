use crate::{
    environment::LayoutEnvironment,
    layout::ResolvedLayout,
    primitives::{Point, ProposedDimensions},
    view::{ViewLayout, ViewMarker},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct RoundedRectangle {
    corner_radius: u16,
}

impl RoundedRectangle {
    #[must_use]
    pub const fn new(corner_radius: u16) -> Self {
        Self { corner_radius }
    }
}

impl ViewMarker for RoundedRectangle {
    type Renderables = crate::render::RoundedRect;
}

impl<Captures: ?Sized> ViewLayout<Captures> for RoundedRectangle {
    type Sublayout = ();
    type State = ();

    fn build_state(&self, _captures: &mut Captures) -> Self::State {}

    fn layout(
        &self,
        offer: &ProposedDimensions,
        _: &impl crate::environment::LayoutEnvironment,
        _captures: &mut Captures,
        _state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        ResolvedLayout {
            sublayouts: (),
            resolved_size: offer.resolve_most_flexible(0, 1),
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
        crate::render::RoundedRect {
            origin,
            size: layout.resolved_size.into(),
            corner_radius: self.corner_radius,
        }
    }
}
