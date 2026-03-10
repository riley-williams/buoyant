use buoyant::view::prelude::*;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;

pub fn split_alignment() -> impl View<Rgb888, ()> {
    VStack::new((
        Circle.foreground_color(Rgb888::CSS_CORAL),
        Circle
            .foreground_color(Rgb888::CSS_DARK_ORCHID)
            .flex_frame()
            .with_infinite_max_width()
            .with_horizontal_alignment(HorizontalAlignment::Leading),
        Circle.foreground_color(Rgb888::CSS_GOLDENROD),
    ))
    .with_alignment(HorizontalAlignment::Trailing)
    .with_spacing(10)
}
