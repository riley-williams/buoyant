use crate::definitions::{GoodPixelColor, HwCell, Palette, State};
use buoyant::view::prelude::*;
use embedded_graphics::mono_font::ascii::FONT_6X10;

pub fn hw_line<'a, C: GoodPixelColor>(
    cells: &'a [HwCell],
    palette: &'a Palette<C>,
    reverse: bool,
) -> impl View<C, State> + 'a {
    ForEach::<16>::new_horizontal(cells, move |cell| hw_cell(cell, palette, reverse))
        .frame()
        .flex_infinite_width(HorizontalAlignment::Center)
        .padding(Edges::Vertical, 1)
        .background(
            Alignment::Center,
            Rectangle.foreground_color(palette.dark_gray()),
        )
}

fn hw_cell<C: GoodPixelColor>(
    cell: &HwCell,
    palette: &Palette<C>,
    reverse: bool,
) -> impl View<C, State> {
    let name = match cell {
        HwCell::Digital(name, _) => *name,
        HwCell::DoubleDigital(name, _, _) => *name,
        HwCell::Analog(name, _) => *name,
    };

    let name = Text::new(name, &FONT_6X10).foreground_color(palette.light_gray());
    let indicator = buoyant::match_view!(cell, {
        HwCell::Digital(_, value) => digital_indicator(*value, palette),
        HwCell::DoubleDigital(_, a, b) => double_digital_indicator(*a, *b, palette) ,
        HwCell::Analog(_, value) => analog_indicator(*value, palette),
    });

    buoyant::if_view!((reverse) {
        VStack::new((indicator, name)).with_spacing(1).padding(Edges::Horizontal, 3)
    } else {
        VStack::new((name, indicator)).with_spacing(1).padding(Edges::Horizontal, 3)
    })
}

fn digital_indicator<C: GoodPixelColor>(value: bool, palette: &Palette<C>) -> impl View<C, State> {
    Circle
        .stroked(1)
        .foreground_color(palette.black())
        .background(
            Alignment::Center,
            Circle.foreground_color(if value {
                palette.green()
            } else {
                palette.dark_gray()
            }),
        )
        .frame()
        .with_width(6)
        .with_height(6)
}

fn double_digital_indicator<C: GoodPixelColor>(
    a: bool,
    b: bool,
    palette: &Palette<C>,
) -> impl View<C, State> {
    HStack::new((digital_indicator(a, palette), digital_indicator(b, palette))).with_spacing(2)
}

fn analog_indicator<C: GoodPixelColor>(percent: u8, palette: &Palette<C>) -> impl View<C, State> {
    let width = 10;
    let fill_width = u32::from(width as u16 * u16::from(percent) / 100);

    ZStack::new((
        // background
        Rectangle
            .foreground_color(palette.light_gray())
            .frame()
            .with_width(width),
        // fill
        Rectangle
            .foreground_color(palette.green())
            .frame()
            .with_width(fill_width),
    ))
    .with_horizontal_alignment(HorizontalAlignment::Leading)
    .frame()
    .with_width(width)
    .with_height(4)
}
