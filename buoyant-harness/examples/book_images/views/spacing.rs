use buoyant::view::prelude::*;
use embedded_graphics::{
    mono_font::ascii::{FONT_7X13, FONT_9X15, FONT_9X15_BOLD},
    pixelcolor::Rgb888,
    prelude::*,
};

mod constants {
    pub const ELEMENT: u32 = 6;
    pub const COMPONENT: u32 = 12;
    pub const SECTION: u32 = 18;
}

pub fn vstack_spacing() -> impl View<Rgb888, ()> {
    VStack::new((
        Circle.foreground_color(Rgb888::CSS_CORAL),
        Rectangle
            .corner_radius(25)
            .foreground_color(Rgb888::CSS_DARK_ORCHID),
        Capsule.foreground_color(Rgb888::CSS_GOLDENROD),
    ))
    .with_alignment(HorizontalAlignment::Trailing)
    .with_spacing(10)
}

pub fn spacing_hierarchy() -> impl View<Rgb888, ()> {
    VStack::new((
        VStack::new((
            Text::new("Parents", &FONT_9X15_BOLD),
            contact_row(Rgb888::CSS_CORAL, "Alice", "Mother"),
            contact_row(Rgb888::CSS_DARK_ORCHID, "Bob", "Father"),
        ))
        .with_alignment(HorizontalAlignment::Leading)
        .with_spacing(constants::COMPONENT),
        VStack::new((
            Text::new("Siblings", &FONT_9X15_BOLD),
            contact_row(Rgb888::CSS_GOLDENROD, "Clyde", "Brother"),
            contact_row(Rgb888::CSS_SKY_BLUE, "Denise", "Sister"),
        ))
        .with_alignment(HorizontalAlignment::Leading)
        .with_spacing(constants::COMPONENT),
    ))
    .with_alignment(HorizontalAlignment::Leading)
    .with_spacing(constants::SECTION)
    .padding(Edges::Horizontal, constants::COMPONENT)
    .padding(Edges::Vertical, constants::SECTION)
}

pub fn spacing_hierarchy_oops() -> impl View<Rgb888, ()> {
    VStack::new((
        VStack::new((
            Text::new("Parents", &FONT_9X15_BOLD),
            contact_row(Rgb888::CSS_CORAL, "Alice", "Mother"),
            contact_row(Rgb888::CSS_DARK_ORCHID, "Bob", "Father"),
        ))
        .with_spacing(constants::COMPONENT),
        VStack::new((
            Text::new("Siblings", &FONT_9X15_BOLD),
            contact_row(Rgb888::CSS_GOLDENROD, "Clyde", "Brother"),
            contact_row(Rgb888::CSS_SKY_BLUE, "Denise", "Sister"),
        ))
        .with_spacing(constants::COMPONENT),
    ))
    .with_alignment(HorizontalAlignment::Leading)
    .with_spacing(constants::SECTION)
    .padding(Edges::Horizontal, constants::COMPONENT)
    .padding(Edges::Vertical, constants::SECTION)
}

fn contact_row<'a>(
    color: Rgb888,
    name: &'a str,
    relationship: &'a str,
) -> impl View<Rgb888, ()> + use<'a> {
    HStack::new((
        Circle.foreground_color(color).frame().with_width(40),
        VStack::new((
            Text::new(name, &FONT_9X15),
            Text::new(relationship, &FONT_7X13),
        ))
        .with_alignment(HorizontalAlignment::Leading)
        .with_spacing(constants::ELEMENT)
        .foreground_color(Rgb888::WHITE),
    ))
    .with_alignment(VerticalAlignment::Top)
    .with_spacing(constants::ELEMENT)
}
