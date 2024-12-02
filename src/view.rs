mod conditional_view;
mod divider;
mod empty_view;
mod foreach;
mod hstack;
mod modifier;
mod shape;
mod spacer;
mod text;
mod vstack;
mod zstack;

pub use conditional_view::ConditionalView;
pub use divider::Divider;
pub use empty_view::EmptyView;
pub use foreach::ForEach;
pub use hstack::HStack;
pub use shape::style;
pub use shape::Circle;
pub use shape::Rectangle;
pub use spacer::Spacer;
pub use text::{HorizontalTextAlignment, Text};
pub use vstack::VStack;
pub use zstack::ZStack;

use modifier::{FixedFrame, FlexFrame, ForegroundStyle, Padding, Priority};

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

    fn flex_frame(
        self,
        min_width: Option<u16>,
        max_width: Option<u16>,
        min_height: Option<u16>,
        max_height: Option<u16>,
        horizontal_alignment: Option<crate::layout::HorizontalAlignment>,
        vertical_alignment: Option<crate::layout::VerticalAlignment>,
    ) -> FlexFrame<Self> {
        FlexFrame::new(
            self,
            min_width,
            max_width,
            min_height,
            max_height,
            horizontal_alignment,
            vertical_alignment,
        )
    }

    fn priority(self, priority: u16) -> Priority<Self> {
        Priority::new(priority, self)
    }
}

impl<T: crate::layout::Layout> LayoutExtensions for T {}

pub trait CharacterRenderExtensions<Color: Copy>: Sized {
    fn foreground_color(self, color: Color) -> ForegroundStyle<Self, Color> {
        ForegroundStyle::new(color, self)
    }
}

impl<Color: Copy, T: crate::render::CharacterRender<Color>> CharacterRenderExtensions<Color> for T {}

#[cfg(feature = "embedded-graphics")]
pub trait PixelRenderExtensions<Color: Copy>: Sized {
    fn foreground_color(self, color: Color) -> ForegroundStyle<Self, Color> {
        ForegroundStyle::new(color, self)
    }
}

#[cfg(feature = "embedded-graphics")]
impl<
        Color: embedded_graphics_core::pixelcolor::PixelColor,
        T: crate::render::PixelRender<Color>,
    > PixelRenderExtensions<Color> for T
{
}
