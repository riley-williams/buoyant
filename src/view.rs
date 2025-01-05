mod conditional_view;
mod divider;
mod empty_view;
mod foreach;
mod hstack;
mod modifier;
pub mod shape;
mod spacer;
mod text;
mod vstack;
mod zstack;

pub use conditional_view::ConditionalView;
pub use divider::Divider;
pub use empty_view::EmptyView;
pub use foreach::ForEach;
pub use hstack::HStack;
pub use spacer::Spacer;
pub(crate) use text::WhitespaceWrap;
pub use text::{HorizontalTextAlignment, Text};
pub use vstack::VStack;
pub use zstack::ZStack;

use modifier::{FixedFrame, FlexFrame, ForegroundStyle, Padding, Priority};

use crate::{
    environment::DefaultEnvironment,
    primitives::Size,
    render::{shade::ShadeSolid, Render, Renderable},
};

pub trait LayoutExtensions: Sized {
    fn padding(self, amount: u16) -> Padding<Self> {
        Padding::new(amount, self)
    }

    fn frame(
        self,
        width: Option<u16>,
        height: Option<u16>,
        horizontal_alignment: Option<crate::layout::HorizontalAlignment>,
        vertical_alignment: Option<crate::layout::VerticalAlignment>,
    ) -> FixedFrame<Self> {
        FixedFrame::new(
            self,
            width,
            height,
            horizontal_alignment,
            vertical_alignment,
        )
    }

    fn flex_frame(self) -> FlexFrame<Self> {
        FlexFrame::new(self)
    }

    fn priority(self, priority: u16) -> Priority<Self> {
        Priority::new(priority, self)
    }
}

pub trait RenderExtensions<C: ShadeSolid + Clone>: Sized {
    fn foreground_color(self, color: C) -> ForegroundStyle<Self, C> {
        ForegroundStyle::new(color, self)
    }
}

impl<T: crate::layout::Layout> LayoutExtensions for T {}
impl<T: Renderable<C>, C: ShadeSolid + Clone> RenderExtensions<C> for T {}

pub fn make_render_tree<C, V: Renderable<C>>(view: &V, size: Size) -> impl Render<C>
where
    V::Renderables: Render<C>,
{
    let env = DefaultEnvironment;
    let layout = view.layout(&size.into(), &env);
    view.render_tree(&layout, Default::default(), &env)
}
