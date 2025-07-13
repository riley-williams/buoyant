use crate::{
    environment::LayoutEnvironment,
    layout::ResolvedLayout,
    primitives::{Dimensions, Point, ProposedDimensions},
    view::{ViewLayout, ViewMarker},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmptyView;

impl ViewMarker for EmptyView {
    type Renderables = ();
}

impl<Captures: ?Sized> ViewLayout<Captures> for EmptyView {
    type State = ();
    type Sublayout = ();

    fn priority(&self) -> i8 {
        i8::MIN
    }

    fn is_empty(&self) -> bool {
        true
    }

    fn build_state(&self, _captures: &mut Captures) -> Self::State {}

    fn layout(
        &self,
        _: &ProposedDimensions,
        _: &impl crate::environment::LayoutEnvironment,
        _captures: &mut Captures,
        _state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        ResolvedLayout {
            sublayouts: (),
            resolved_size: Dimensions::zero(),
        }
    }

    fn render_tree(
        &self,
        _layout: &ResolvedLayout<Self::Sublayout>,
        _origin: Point,
        _env: &impl LayoutEnvironment,
        _captures: &mut Captures,
        _state: &mut Self::State,
    ) -> Self::Renderables {
    }
}
