mod divider;
mod hstack;
mod modifier;
mod rectangle;
mod spacer;
mod text;
mod vstack;
mod zstack;

pub use divider::Divider;
pub use hstack::HStack;
pub use rectangle::Rectangle;
pub use spacer::Spacer;
pub use text::{HorizontalTextAlignment, Text};
pub use vstack::VStack;
pub use zstack::ZStack;

use modifier::ForegroundStyle;
use modifier::Padding;

pub trait View: Sized {
    fn padding(self, amount: u16) -> Padding<Self> {
        Padding::new(amount, self)
    }

    fn foreground_style<Style: crate::style::color_style::ColorStyle>(
        self,
        style: Style,
    ) -> ForegroundStyle<Self, Style> {
        ForegroundStyle::new(style, self)
    }
}

impl<T: crate::layout::Layout> View for T {}
