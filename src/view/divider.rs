use crate::{
    environment::LayoutEnvironment,
    event::EventResult,
    focus::{FocusEvent, FocusStateChange},
    layout::{LayoutDirection, ResolvedLayout},
    primitives::{Dimensions, Point, ProposedDimensions},
    transition::Opacity,
    view::{ViewLayout, ViewMarker},
};

/// Divider renders a horizontal or vertical line, depending on the context in which it is
/// used.
#[derive(Debug, Clone)]
pub struct Divider {
    /// The line width
    pub weight: u16,
}

impl Divider {
    #[allow(missing_docs)]
    #[must_use]
    pub const fn new(weight: u16) -> Self {
        Self { weight }
    }
}

impl Default for Divider {
    fn default() -> Self {
        Self::new(1)
    }
}

impl PartialEq for Divider {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl ViewMarker for Divider {
    type Renderables = crate::render::Rect;
    type Transition = Opacity;
}

impl<Captures: ?Sized> ViewLayout<Captures> for Divider {
    type State = ();
    type Sublayout = Dimensions;
    type FocusTree = ();

    fn priority(&self) -> i8 {
        i8::MAX
    }

    fn transition(&self) -> Self::Transition {
        Opacity
    }

    fn build_state(&self, _captures: &mut Captures) -> Self::State {}

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
        _captures: &mut Captures,
        _state: &mut <Self as ViewLayout<Captures>>::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        let size = match env.layout_direction() {
            LayoutDirection::Vertical => Dimensions {
                width: offer.width.resolve_most_flexible(0, 1),
                height: self.weight.into(),
            },
            LayoutDirection::Horizontal => Dimensions {
                width: self.weight.into(),
                height: offer.height.resolve_most_flexible(0, 1),
            },
        };
        ResolvedLayout {
            sublayouts: size,
            resolved_size: size,
        }
    }

    fn render_tree(
        &self,
        layout: &Self::Sublayout,
        origin: Point,
        _env: &impl LayoutEnvironment,
        _captures: &mut Captures,
        _state: &mut Self::State,
    ) -> Self::Renderables {
        crate::render::Rect {
            origin,
            size: (*layout).into(),
        }
    }

    fn handle_event(
        &self,
        _event: &crate::view::Event,
        _context: &crate::event::EventContext,
        _render_tree: &mut Self::Renderables,
        _captures: &mut Captures,
        _state: &mut Self::State,
    ) -> EventResult {
        EventResult::default()
    }

    fn focus(
        &self,
        _event: &FocusEvent,
        _context: &crate::event::EventContext,
        _render_tree: &mut Self::Renderables,
        _captures: &mut Captures,
        _state: &mut Self::State,
        _focus: &mut Self::FocusTree,
    ) -> FocusStateChange {
        FocusStateChange::Exhausted
    }
}
