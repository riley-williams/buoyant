use crate::{
    layout::{Layout, ResolvedLayout},
    primitives::{Dimensions, ProposedDimensions},
    render::NullRender,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmptyView;

impl Layout for EmptyView {
    type Sublayout = ();

    #[inline]
    fn layout(
        &self,
        _: &ProposedDimensions,
        _: &impl crate::environment::LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        ResolvedLayout {
            sublayouts: (),
            resolved_size: Dimensions::zero(),
        }
    }

    #[inline]
    fn priority(&self) -> i8 {
        i8::MIN
    }

    #[inline]
    fn is_empty(&self) -> bool {
        true
    }
}

impl NullRender for EmptyView {}
