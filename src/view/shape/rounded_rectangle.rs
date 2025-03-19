use crate::{
    environment::LayoutEnvironment,
    layout::{Layout, ResolvedLayout},
    primitives::{Point, ProposedDimensions},
    render::Renderable,
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

impl Layout for RoundedRectangle {
    type Sublayout = ();

    fn layout(
        &self,
        offer: &ProposedDimensions,
        _: &impl crate::environment::LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        ResolvedLayout {
            sublayouts: (),
            resolved_size: offer.resolve_most_flexible(0, 10),
        }
    }
}

impl Renderable for RoundedRectangle {
    type Renderables = crate::render::RoundedRect;

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        _env: &impl LayoutEnvironment,
    ) -> Self::Renderables {
        crate::render::RoundedRect {
            origin,
            size: layout.resolved_size.into(),
            corner_radius: self.corner_radius,
        }
    }
}
