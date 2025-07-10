use crate::{
    environment::LayoutEnvironment,
    layout::{LayoutDirection, ResolvedLayout},
    primitives::{Dimensions, Point, ProposedDimensions},
    view::{ViewLayout, ViewMarker},
};

/// An invisible spacer that greedily takes space in the layout.
///
/// Depending on the layout direction, it will take up space either horizontally or vertically.
/// Spacer has the minimum layout priority so it will be offered space by stacks only after
/// all other siblings have been laid out.
#[derive(Debug, Default, PartialEq, Eq)]
pub struct Spacer {
    min_length: u32,
}

impl ViewMarker for Spacer {
    type Renderables = ();
}

impl<Captures: ?Sized> ViewLayout<Captures> for Spacer {
    type State = ();
    type Sublayout = ();

    fn priority(&self) -> i8 {
        // This view should take all the remaining space after other siblings have been laid out
        i8::MIN
    }

    fn build_state(&self, _captures: &mut Captures) -> Self::State {}

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
        _captures: &mut Captures,
        _state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        let size = match env.layout_direction() {
            LayoutDirection::Horizontal => Dimensions {
                width: offer.width.resolve_most_flexible(0, self.min_length),
                height: 0u32.into(),
            },
            LayoutDirection::Vertical => Dimensions {
                width: 0u32.into(),
                height: offer.height.resolve_most_flexible(0, self.min_length),
            },
        };
        ResolvedLayout {
            sublayouts: (),
            resolved_size: size,
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
