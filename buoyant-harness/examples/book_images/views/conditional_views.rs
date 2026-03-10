use buoyant::if_view;
use buoyant::match_view;
use buoyant::view::prelude::*;
use embedded_graphics::{mono_font::ascii::FONT_9X15, pixelcolor::Rgb888, prelude::*};

fn secret_message(message: &str, is_redacted: bool) -> impl View<Rgb888, ()> + use<'_> {
    if_view!((is_redacted) {
        RoundedRectangle::new(4)
            .frame()
            .with_width(9 * message.len() as u32)
            .with_height(15)
    } else {
        Text::new(message, &FONT_9X15)
    })
}

pub fn if_redacted() -> impl View<Rgb888, ()> {
    VStack::new((
        secret_message("Top secret message", true),
        secret_message("Hi Mom!", false),
        secret_message("hunter12", true),
        secret_message("Cats are cool", false),
    ))
    .with_spacing(10)
    .with_alignment(buoyant::layout::HorizontalAlignment::Leading)
    .padding(Edges::All, 10)
}

#[derive(Debug, Clone, Copy)]
enum Shape {
    Rectangle,
    RoundedRect(u16),
    None,
}

fn shape(s: Shape) -> impl View<Rgb888, ()> {
    match_view!(s, {
        Shape::Rectangle => {
            Rectangle
        },
        Shape::RoundedRect(radius) => {
            RoundedRectangle::new(radius)
        },
        Shape::None => {
            EmptyView
        }
    })
}

pub fn match_view_example() -> impl View<Rgb888, ()> {
    VStack::new((
        shape(Shape::Rectangle).foreground_color(Rgb888::CSS_PALE_GREEN),
        shape(Shape::RoundedRect(10)).foreground_color(Rgb888::CSS_MEDIUM_ORCHID),
        shape(Shape::None).foreground_color(Rgb888::WHITE),
        shape(Shape::RoundedRect(30)).foreground_color(Rgb888::CSS_INDIAN_RED),
    ))
    .with_spacing(10)
    .padding(Edges::All, 10)
}
