use crate::{
    environment::LayoutEnvironment,
    layout::{Layout, LayoutDirection, ResolvedLayout},
    primitives::{Dimensions, Point, ProposedDimensions},
    render::Renderable,
};

#[derive(Debug, Clone)]
pub struct Divider {
    pub weight: u16,
}

impl Divider {
    #[inline]
    #[must_use]
    pub const fn new(weight: u16) -> Self {
        Self { weight }
    }
}

impl Default for Divider {
    #[inline]
    fn default() -> Self {
        Self::new(1)
    }
}

impl PartialEq for Divider {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl Layout for Divider {
    type Sublayout = ();

    #[inline]
    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
    ) -> ResolvedLayout<()> {
        let size = match env.layout_direction() {
            LayoutDirection::Vertical => Dimensions {
                width: offer.width.resolve_most_flexible(0, 10),
                height: self.weight.into(),
            },
            LayoutDirection::Horizontal => Dimensions {
                width: self.weight.into(),
                height: offer.height.resolve_most_flexible(0, 10),
            },
        };
        ResolvedLayout {
            sublayouts: (),
            resolved_size: size,
        }
    }

    #[inline]
    fn priority(&self) -> i8 {
        i8::MAX
    }
}

impl<C> Renderable<C> for Divider {
    type Renderables = crate::render::Rect;

    #[inline]
    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        _env: &impl LayoutEnvironment,
    ) -> Self::Renderables {
        crate::render::Rect {
            origin,
            size: layout.resolved_size.into(),
        }
    }
}
