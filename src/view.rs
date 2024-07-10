mod conditional_view;
mod divider;
mod empty_view;
mod hstack;
mod modifier;
mod rectangle;
mod spacer;
mod text;
mod vstack;
mod zstack;

pub use conditional_view::ConditionalView;
pub use divider::Divider;
pub use hstack::HStack;
pub use rectangle::Rectangle;
pub use spacer::Spacer;
pub use text::{HorizontalTextAlignment, Text};
pub use vstack::VStack;
pub use zstack::ZStack;

use modifier::{FixedFrame, FlexFrame, ForegroundStyle, Padding};

pub trait ViewExtensions: Sized {
    fn padding(self, amount: u16) -> Padding<Self> {
        Padding::new(amount, self)
    }

    fn foreground_style<Style: crate::style::color_style::ColorStyle>(
        self,
        style: Style,
    ) -> ForegroundStyle<Self, Style> {
        ForegroundStyle::new(style, self)
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
}

impl<T: crate::layout::Layout> ViewExtensions for T {}
