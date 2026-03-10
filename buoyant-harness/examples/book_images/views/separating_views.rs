use buoyant::view::prelude::*;
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::*;

pub fn spacer() -> impl View<Rgb888, ()> {
    VStack::new((
        HStack::new((
            Circle.foreground_color(Rgb888::CSS_CORAL),
            Spacer::default(),
            Circle.foreground_color(Rgb888::CSS_CORAL),
        )),
        Rectangle
            .corner_radius(25)
            .foreground_color(Rgb888::CSS_DARK_ORCHID),
        Capsule.foreground_color(Rgb888::CSS_GOLDENROD),
    ))
    .with_alignment(HorizontalAlignment::Trailing)
    .with_spacing(10)
}
