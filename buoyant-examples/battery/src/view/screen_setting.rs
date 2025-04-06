use std::time::Duration;

use buoyant::{
    animation::Animation,
    if_view,
    layout::Alignment,
    view::{
        padding::Edges,
        shape::{Rectangle, RoundedRectangle},
        HStack, HorizontalTextAlignment, Text, VStack, View, ViewExt, ZStack,
    },
};
use embedded_graphics::mono_font::ascii::{FONT_10X20, FONT_6X10, FONT_9X15_BOLD};

use crate::color::{self, ColorFormat};

#[must_use]
pub fn view(auto_off: bool) -> impl View<ColorFormat> {
    VStack::new((
        Text::new("Auto Screen Off", &FONT_9X15_BOLD)
            .multiline_text_alignment(HorizontalTextAlignment::Center),
        Text::new("Long press for 1s to switch", &FONT_6X10)
            .multiline_text_alignment(HorizontalTextAlignment::Center),
        toggle(auto_off),
    ))
    .with_spacing(5)
}

fn toggle(is_on: bool) -> impl View<ColorFormat> {
    let (txt_lhs, txt_rhs) = if is_on {
        (color::GREEN, color::BLACK)
    } else {
        (color::BLACK, color::RED)
    };

    ZStack::new((
        // This is a...creative...way of moving the rectangle between the
        // left and right side while taking half the space. I'm delaying
        // introducing API for reading layout. It's surprisingly hard,
        // and good performance depends on caching.
        HStack::new((
            if_view!((!is_on) {
                Rectangle.hidden()
            }),
            RoundedRectangle::new(5).foreground_color(color::CONTENT),
            if_view!((is_on) {
                Rectangle.hidden()
            }),
        )),
        HStack::new((
            Text::new("On", &FONT_10X20)
                .foreground_color(txt_lhs)
                .flex_frame()
                .with_infinite_max_width()
                .with_infinite_max_height(),
            Text::new("Off", &FONT_10X20)
                .foreground_color(txt_rhs)
                .flex_frame()
                .with_infinite_max_width()
                .with_infinite_max_height(),
        )),
    ))
    .with_alignment(if is_on {
        Alignment::Leading
    } else {
        Alignment::Trailing
    })
    .animated(Animation::ease_in_out(Duration::from_millis(120)), is_on)
    .flex_frame()
    .with_max_height(70)
    .padding(Edges::All, 5)
    .background(Alignment::Center, || {
        RoundedRectangle::new(10).foreground_color(color::SECONDARY_BACKGROUND)
    })
}
