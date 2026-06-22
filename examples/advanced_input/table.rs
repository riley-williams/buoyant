use crate::FONT;
use crate::definitions::{GoodPixelColor, IeName, MAX_COLS, MAX_ROWS, Palette, RenderData, State};
use buoyant::event::Event;
use buoyant::view::popover::Dismissal;
use buoyant::view::prelude::*;
use buoyant::view::table::{Table, TableIndex};
use heapless::String;

use std::fmt::Write;

mod ie_editor;

struct Items<'a> {
    names: &'a [Option<IeName<'a>>],
    ie: &'a [Option<f32>],
    eu: &'a [Option<&'static str>],
    table_dimensions: (usize, usize),
}

impl<'a> TableIndex<'a> for Items<'a> {
    type Output = (usize, Option<IeName<'a>>, Option<f32>, Option<&'static str>);

    fn cols(&self) -> usize {
        self.table_dimensions.0
    }

    fn rows(&self) -> usize {
        self.table_dimensions.1
    }

    fn index(&self, x: usize, y: usize) -> Self::Output {
        let i = x + y * self.table_dimensions.0;
        (i, self.names[i], self.ie[i], self.eu[i])
    }
}

fn funnel<F: Fn(&mut State)>(f: F) -> F {
    f
}

pub fn table<'a, C: GoodPixelColor>(
    data: RenderData<'a, C>,
    state: &'_ State,
    table_dimensions: (usize, usize),
    names: &'a [Option<IeName<'a>>],
    ie: &'a [Option<f32>],
    eu: &'a [Option<&'static str>],
) -> impl View<C, State> + use<'a, C> {
    let enter_ie = move |index: u8| {
        let ie = ie[index as usize];
        funnel(move |s: &mut State| {
            if let Some(ie) = ie {
                s.opened_cell_input = Some(index);
                s.temporary_ie = ie.into();
            }
        })
    };
    let set_ie = move |index: u8| {
        funnel(move |s| {
            s.ie_value_update = Some((index, s.temporary_ie.into()));
            s.opened_cell_input = None;
        })
    };
    let overlay = state.opened_cell_input.map(|i| (i, state.temporary_ie));
    let items = Items {
        names,
        ie,
        eu,
        table_dimensions,
    };

    Table::<MAX_COLS, MAX_ROWS>::new(items, move |(index, name, ie, eu)| {
        Button::new(enter_ie(index as u8), move |s| {
            let white = data.palette.white();
            let background = if s.is_focused() | s.is_pressed() {
                Rectangle.stroked(4).foreground_color(white)
            } else {
                Rectangle.stroked(0).foreground_color(white)
            };
            buoyant::match_view!((name, ie), {
                (Some(name), Some(ie)) => ie_cell(name, ie, eu, data.palette),
                (None, Some(ie)) => ie_cell(IeName::Known(""), ie, eu, data.palette),
                (_, None) => EmptyView,
            })
            .flex_infinite_height(Default::default())
            .flex_infinite_width(Default::default())
            .background(Alignment::Center, background)
        })
    })
    .with_stroke(1)
    .padding(Edges::All, 1)
    .background(
        Alignment::Center,
        Rectangle.stroked(1).foreground_color(data.palette.white()),
    )
    .background(
        Alignment::Center,
        Rectangle.foreground_color(data.palette.dark_gray()),
    )
    .popover(overlay.as_ref(), |&(i, ie)| {
        ie_editor::ie_editor(ie, set_ie(i), data.palette)
    })
    .on_blur(|state: &mut State| {
        state.opened_cell_input = None;
        Dismissal::Dismiss
    })
}

fn ie_cell<C: GoodPixelColor>(
    name: IeName<'_>,
    ie: f32,
    eu: Option<&'static str>,
    palette: &'static Palette<C>,
) -> impl View<C, State> {
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
