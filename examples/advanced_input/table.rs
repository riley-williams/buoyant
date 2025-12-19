use crate::FONT;
use crate::definitions::{GoodPixelColor, IeName, MAX_COLS, MAX_ROWS, Palette, RenderData, State};
use buoyant::view::prelude::*;
use heapless::String;

use std::fmt::Write;

pub fn table<'a, C: GoodPixelColor>(
    data: RenderData<'a, C>,
    table_dimensions: (usize, usize),
    names: &'a [Option<IeName<'a>>],
    ie: &'a [Option<f32>],
    eu: &'a [Option<&'static str>],
) -> impl View<C, State> + use<'a, C> {
    const fn index_array<const N: usize>() -> [usize; N] {
        let mut arr = [0; N];
        let mut i = 0;
        while i < N {
            arr[i] = i;
            i += 1;
        }
        arr
    }

    const WIDTH: [usize; MAX_COLS] = index_array();
    const HEIGHT: [usize; MAX_ROWS] = index_array();

    let (c, r) = table_dimensions;

    // Not a table in this example
    ForEach::<MAX_COLS>::new_horizontal(&WIDTH[..c], move |i| {
        ForEach::<MAX_ROWS>::new_vertical(&HEIGHT[..r], move |j| {
            let index = j * c + i;
            let (name, ie, eu) = (names[index], ie[index], eu[index]);
            buoyant::match_view!((name, ie), {
                (Some(name), Some(ie)) => ie_cell(name, ie, eu, data.palette),
                (None, Some(ie)) => ie_cell(IeName::Known(""), ie, eu, data.palette),
                (_, None) => EmptyView,
            })
            .background(Alignment::Center, Rectangle.stroked(1))
            .frame_sized(321 / c as u32, 204 / r as u32)
        })
    })
    .background(Alignment::Center, Rectangle.stroked(2))
}
fn ie_cell<C: GoodPixelColor>(
    name: IeName<'_>,
    ie: f32,
    eu: Option<&'static str>,
    palette: &'static Palette<C>,
) -> impl View<C, State> + use<C> {
    let mut name_buf = String::<16, u8>::new();
    let mut value_buf = String::<8, u8>::new();
    let eu = eu.unwrap_or("");

    // --- name ---
    let overflow = match name {
        IeName::Known(name) => name_buf.push_str(&name[..name.len().min(16)]).is_err(),
        IeName::Addr((c1, c2), (i1, i2, i3)) => {
            write!(name_buf, "{c1}.{c2} {i1}.{i2}.{i3}").is_err()
        }
    };

    // --- value ---
    let value = ((ie * 100.0) / 100.0).trunc();
    let overflow = write!(&mut value_buf, "{value}").is_err() || overflow;

    assert!(!overflow, "Overflow of cell");

    // --- quality ---
    // let vcolor = value_color(ie.try_get_qds(), overflow, palette);

    HStack::new((
        Text::new(name_buf, &FONT),
        Spacer::default(),
        Text::new(value_buf, &FONT).foreground_color(palette.green()),
        Text::new(eu, &FONT),
    ))
    .with_spacing(4)
    .padding(Edges::Vertical, 2)
    .padding(Edges::Horizontal, 6)
    .flex_infinite_width(HorizontalAlignment::Center)
    .flex_infinite_height(VerticalAlignment::Center)
}
