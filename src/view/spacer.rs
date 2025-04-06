use crate::{
    environment::LayoutEnvironment,
    layout::{Layout, LayoutDirection, ResolvedLayout},
    primitives::{Dimensions, Point, ProposedDimensions},
    render::Renderable,
};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Spacer {
    min_length: u32,
}

impl Layout for Spacer {
    type Sublayout = ();

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
    ) -> ResolvedLayout<()> {
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

    fn priority(&self) -> i8 {
        // This view should take all the remaining space after other siblings have been laid out
        i8::MIN
    }
}

impl Renderable for Spacer {
    type Renderables = ();

    fn render_tree(
        &self,
        _layout: &ResolvedLayout<Self::Sublayout>,
        _origin: Point,
        _env: &impl LayoutEnvironment,
    ) {
    }
}
