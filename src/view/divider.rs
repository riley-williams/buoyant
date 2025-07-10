use crate::{
    environment::LayoutEnvironment,
    layout::{LayoutDirection, ResolvedLayout},
    primitives::{Dimensions, Point, ProposedDimensions},
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
}

impl<Captures: ?Sized> ViewLayout<Captures> for Divider {
    type State = ();
    type Sublayout = ();

    fn priority(&self) -> i8 {
        i8::MAX
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
            sublayouts: (),
            resolved_size: size,
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
        crate::render::Rect {
            origin,
            size: layout.resolved_size.into(),
        }
    }
}
