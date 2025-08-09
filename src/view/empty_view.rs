use crate::{
    environment::LayoutEnvironment,
    layout::ResolvedLayout,
    primitives::{Dimensions, Point, ProposedDimensions},
    transition::Opacity,
    view::{ViewLayout, ViewMarker},
};

/// A view that does not render anything and has a zero size.
///
/// In stacks, spacing is not added around this view which makes
/// it particularly useful for variants of [`match_view!`][crate::match_view!]
/// that should not render an item.
///
/// # Examples
///
/// This view maintains the expected 10 pixel spacing between the first and last
/// views when the state is `Nothing`.
///
/// ```
/// use buoyant::match_view;
/// use buoyant::font::CharacterBufferFont;
/// use buoyant::view::prelude::*;
/// use embedded_graphics::mono_font::ascii::FONT_9X15;
///
/// enum State {
///     Message(&'static str),
///     Nothing,
/// }
///
/// let view = |state| {
///     VStack::new((
///         Text::new("First", &FONT_9X15),
///         match_view!(state => {
///             State::Message(msg) => Text::new(msg, &FONT_9X15),
///             State::Nothing => EmptyView,
///         }),
///         Text::new("First", &FONT_9X15),
///     )).with_spacing(10)
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmptyView;

impl ViewMarker for EmptyView {
    type Renderables = ();
    type Transition = Opacity;
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

    fn transition(&self) -> Self::Transition {
        Opacity
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
