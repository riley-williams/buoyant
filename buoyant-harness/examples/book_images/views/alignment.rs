use buoyant::view::prelude::*;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;

pub fn hstack_vertical_alignment() -> impl View<Rgb888, ()> {
    HStack::new((
        Circle.foreground_color(Rgb888::CSS_CORAL),
        Rectangle
            .corner_radius(25)
            .foreground_color(Rgb888::CSS_DARK_ORCHID),
    ))
    .with_alignment(VerticalAlignment::Top)
}
