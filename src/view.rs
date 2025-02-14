mod conditional_view;
mod divider;
mod empty_view;
mod foreach;
mod hstack;
pub mod match_view;
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

use modifier::{
    Animated, FixedFrame, FlexFrame, ForegroundStyle, GeometryGroup, Padding, Priority,
};

use crate::{
    environment::DefaultEnvironment,
    primitives::{Point, Size},
    render::{CharacterRender, Renderable},
    Animation,
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

    fn animated<T: PartialEq + Clone>(self, animation: Animation, value: T) -> Animated<Self, T> {
        Animated::new(self, animation, value)
    }

    fn geometry_group(self) -> GeometryGroup<Self> {
        GeometryGroup::new(self)
    }
}

pub trait RenderExtensions<C>: Sized {
    fn foreground_color(self, color: C) -> ForegroundStyle<Self, C> {
        ForegroundStyle::new(color, self)
    }
    fn foreign_color<U: Into<C>>(self, color: U) -> ForegroundStyle<Self, C> {
        ForegroundStyle::new(color.into(), self)
    }
}

impl<T: crate::layout::Layout> LayoutExtensions for T {}
impl<T: Renderable<C>, C> RenderExtensions<C> for T {}

// TODO: Remove this
pub fn make_render_tree<C, V>(view: &V, size: Size) -> V::Renderables
where
    V: Renderable<C>,
    V::Renderables: CharacterRender<C>,
{
    let env = DefaultEnvironment::default();
    let layout = view.layout(&size.into(), &env);
    view.render_tree(&layout, Point::default(), &env)
}
