use crate::definitions::{GoodPixelColor, IeName, MAX_COLS, MAX_ROWS, Palette, RenderData, State};
use crate::table::movement_proxy::{TableMovementProxy, TableProxyAction};
use crate::{FONT, G0, G1, G2};
use buoyant::event::input::Interaction;
use buoyant::view::prelude::*;
use heapless::String;
use ie_editor::ie_editor;

mod ie_editor;
mod movement_proxy;

use std::fmt::Write;

fn funnel_proxy<F: Fn(T, &mut State), T>(f: F) -> F {
    f
}
fn funnel<F: Fn(&mut State)>(f: F) -> F {
    f
}

pub fn table<'a, 'b, C: GoodPixelColor>(
    data: RenderData<'a, C>,
    state: &'b State,
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
    let i = data.input;

    let action = funnel_proxy(move |a, s| match a {
        TableProxyAction::Enter => {
            s.focused_table = Some(i.replace(G0, G1));
            s.artificial_event = Some(buoyant::event::Event::Keyboard(
                buoyant::event::keyboard::KeyboardEvent {
                    groups: G1,
                    kind: buoyant::event::keyboard::KeyboardEventKind::Right,
                },
            ));
        }
        TableProxyAction::Escape => _ = s.focused_table.take_guard(i),
    });

    let enter_ie = move |index: u8| {
        let ie = ie[index as usize];
        funnel(move |s: &mut State| {
            if let Some(ie) = ie {
                s.opened_cell_input = Some((index, i.replace(G1, G2)));
                s.temporary_ie = ie.into()
            }
        })
    };
    let set_ie = move |index: u8| {
        funnel(move |s| {
            s.ie_value_update = Some((index, s.temporary_ie.into()));
            s.opened_cell_input.take_guard(i);
        })
    };
    let cancel_ie = funnel(move |s: &mut State| _ = s.opened_cell_input.take_guard(i));

    let overlay = state.opened_cell_input.as_ref().map(|(i, _)| *i);
    let overlay = overlay.map(|i| (i, state.temporary_ie));
    let overlay = overlay.map(|(i, ie)| ie_editor(ie, set_ie(i), cancel_ie, data.palette));

    // Not a table in this example
    TableMovementProxy::new(G1, action, {
        ForEach::<MAX_COLS>::new_horizontal(&WIDTH[..c], move |i| {
            ForEach::<MAX_ROWS>::new_vertical(&HEIGHT[..r], move |j| {
                let index = j * c + i;
                Button::new_with_groups(enter_ie(index as u8), G1, move |i: Interaction| {
                    let (name, ie, eu) = (names[index], ie[index], eu[index]);
                    let width = if i.is_focused() { 4 } else { 0 };
                    let white = data.palette.white();
                    let background = Rectangle.stroked(width).foreground_color(white);
                    buoyant::match_view!((name, ie), {
                        (Some(name), Some(ie)) => ie_cell(name, ie, eu, data.palette),
                        (None, Some(ie)) => ie_cell(IeName::Known(""), ie, eu, data.palette),
                        (_, None) => EmptyView,
                    })
                    .background(Alignment::Center, Rectangle.stroked(1))
                    .frame_sized(321 / c as u32, 204 / r as u32)
                    .background(Alignment::Center, background)
                })
            })
        })
        .background(Alignment::Center, Rectangle.stroked(2))
        .overlay(Alignment::Center, overlay)
    })
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
