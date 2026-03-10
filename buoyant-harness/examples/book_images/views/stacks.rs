use buoyant::view::prelude::*;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;

pub fn hstack() -> impl View<Rgb888, ()> {
    HStack::new((
        Circle.stroked(15).foreground_color(Rgb888::CSS_CORAL),
        Rectangle
            .corner_radius(25)
            .foreground_color(Rgb888::CSS_DARK_ORCHID),
    ))
}

pub fn zstack() -> impl View<Rgb888, ()> {
    ZStack::new((
        Rectangle
            .corner_radius(50)
            .foreground_color(Rgb888::CSS_DARK_ORCHID),
        Circle.foreground_color(Rgb888::CSS_CORAL),
        Circle
            .foreground_color(Rgb888::CSS_GOLDENROD)
            .padding(Edges::All, 25),
    ))
}

pub fn mixed_stacks() -> impl View<Rgb888, ()> {
    HStack::new((
        VStack::new((
            Circle.foreground_color(Rgb888::CSS_GOLDENROD),
            Circle.foreground_color(Rgb888::CSS_GHOST_WHITE),
        )),
        ZStack::new((
            Rectangle
                .corner_radius(50)
                .foreground_color(Rgb888::CSS_DARK_ORCHID),
            Rectangle
                .corner_radius(25)
                .foreground_color(Rgb888::CSS_CORAL)
                .padding(Edges::All, 25),
        )),
    ))
}
