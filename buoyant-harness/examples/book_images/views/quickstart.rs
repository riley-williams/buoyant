use buoyant::view::prelude::*;
use embedded_graphics::{mono_font::ascii::FONT_10X20, pixelcolor::Rgb888, prelude::*};

pub fn hello_view() -> impl View<Rgb888, ()> {
    HStack::new((
        Text::new("Hello", &FONT_10X20).foreground_color(Rgb888::GREEN),
        Spacer::default(),
        Text::new("World", &FONT_10X20).foreground_color(Rgb888::YELLOW),
    ))
    .padding(Edges::All, 20)
}
