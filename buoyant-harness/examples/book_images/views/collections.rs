use buoyant::view::prelude::*;
use embedded_graphics::{mono_font::ascii::FONT_9X15, pixelcolor::Rgb888, prelude::*};

struct Swatch {
    name: &'static str,
    color: Rgb888,
}

mod constants {
    pub const ELEMENT: u32 = 6;
    pub const COMPONENT: u32 = 12;
}

static SWATCHES: [Swatch; 4] = [
    Swatch {
        name: "Indigo",
        color: Rgb888::CSS_INDIGO,
    },
    Swatch {
        name: "Indian Red",
        color: Rgb888::CSS_INDIAN_RED,
    },
    Swatch {
        name: "Dark Orange",
        color: Rgb888::CSS_DARK_ORANGE,
    },
    Swatch {
        name: "Mint Cream",
        color: Rgb888::CSS_MINT_CREAM,
    },
];

pub fn foreach() -> impl View<Rgb888, ()> {
    ForEach::<10>::new_vertical(&SWATCHES, |swatch| {
        HStack::new((
            RoundedRectangle::new(8)
                .foreground_color(swatch.color)
                .frame_sized(40, 40),
            Text::new(swatch.name, &FONT_9X15).foreground_color(Rgb888::WHITE),
        ))
        .with_spacing(constants::ELEMENT)
    })
    .with_alignment(HorizontalAlignment::Leading)
    .with_spacing(constants::COMPONENT)
    .padding(Edges::All, constants::COMPONENT)
}
