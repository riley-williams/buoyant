use crate::{
    environment::LayoutEnvironment,
    layout::{Layout, LayoutDirection, ResolvedLayout},
    primitives::{Dimensions, ProposedDimensions},
    render::NullRender,
};

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Spacer {
    min_length: u16,
}

impl Layout for Spacer {
    type Sublayout = ();

    #[inline]
    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
    ) -> ResolvedLayout<()> {
        let size = match env.layout_direction() {
            LayoutDirection::Horizontal => Dimensions {
                width: offer.width.resolve_most_flexible(0, self.min_length),
                height: 0.into(),
            },
            LayoutDirection::Vertical => Dimensions {
                width: 0.into(),
                height: offer.height.resolve_most_flexible(0, self.min_length),
            },
        };
        ResolvedLayout {
            sublayouts: (),
            resolved_size: size,
        }
    }

    #[inline]
    fn priority(&self) -> i8 {
        // This view should take all the remaining space after other siblings have been laid out
        i8::MIN
    }
}

impl NullRender for Spacer {}
