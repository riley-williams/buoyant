use crate::definitions::{GoodPixelColor, Palette, State, TemporaryIe};
use crate::{FONT, G2};
use PaginationAction as A;
use buoyant::view::{Pagination, PaginationAction, prelude::*};

#[derive(Copy, Clone)]
struct Digit(u8);

fn digit_editor<C: GoodPixelColor>(
    value: u8,
    set: impl Fn(&mut State, u8) + 'static,
    palette: &'static Palette<C>,
) -> impl View<C, State> {
    let on_action = move |a, s: &mut State| match a {
        A::Previous => set(s, if value == 9 { 0 } else { value + 1 }),
        A::Next => set(s, if value == 0 { 9 } else { value - 1 }),
        _ => (),
    };

    Pagination::new_vertical::<_, _, State>(G2, on_action, move |i| {
        button_gut(i, palette, Text::new(Digit(value), &FONT))
    })
}

fn flag_editor<C: GoodPixelColor>(
    label: &'static str,
    label_false: &'static str,
    value: bool,
    set: impl Fn(&mut State, bool) + 'static,
    palette: &'static Palette<C>,
) -> impl View<C, State> {
    let on_action = move |a, s: &mut State| match a {
        A::Previous | A::Next => set(s, !value),
        _ => (),
    };

    Pagination::new_vertical::<_, _, State>(G2, on_action, move |i| {
        let text = if value { label } else { label_false };
        button_gut(i, palette, Text::new(text, &FONT))
    })
}

fn button_gut<C: GoodPixelColor>(
    i: buoyant::event::input::Interaction,
    palette: &'static Palette<C>,
    inner: impl View<C, State>,
) -> impl View<C, State> {
    let inner = inner.padding(Edges::All, 3);
    buoyant::if_view!((i.is_focused()) {
        inner.background(
            Alignment::Center,
            RoundedRectangle::new(3)
                .stroked(1)
                .foreground_color(palette.light_gray())
        )
    } else {
        inner
    })
}

pub fn ie_editor<C: GoodPixelColor>(
    TemporaryIe { sign, int, frac }: TemporaryIe,
    submit: impl Fn(&mut State),
    cancel: impl Fn(&mut State),
    palette: &'static Palette<C>,
) -> impl View<C, State> {
    fn set_int<const I: usize>(s: &mut State, v: u8) {
        s.temporary_ie.int[I] = v;
    }
    fn set_frac<const I: usize>(s: &mut State, v: u8) {
        s.temporary_ie.frac[I] = v;
    }
    fn set_sign(s: &mut State, v: bool) {
        s.temporary_ie.sign = v;
    }

    let sign_view = flag_editor("+", "-", sign, set_sign, palette);

    let int_view = HStack::new((
        digit_editor(int[0], set_int::<0>, palette),
        digit_editor(int[1], set_int::<1>, palette),
        digit_editor(int[2], set_int::<2>, palette),
    ));

    let frac_view = HStack::new((
        digit_editor(frac[0], set_frac::<0>, palette),
        digit_editor(frac[1], set_frac::<1>, palette),
        digit_editor(frac[2], set_frac::<2>, palette),
    ));

    let value_row =
        HStack::new((sign_view, int_view, Text::new(".", &FONT), frac_view)).with_spacing(2);

    // let vcolor = super::value_color(Some(&t), false, palette);

    VStack::new((
        value_row, /* .foreground_color(vcolor) */
        Spacer::default().frame().with_height(8),
        Spacer::default().frame().with_height(8),
        Button::new_with_groups(submit, G2, |i| {
            button_gut(i, palette, Text::new("Submit", &FONT))
        }),
    ))
    .on_cancel(cancel)
    .padding(Edges::Vertical, 6)
    .padding(Edges::Horizontal, 10)
    .background(
        Alignment::Center,
        RoundedRectangle::new(5)
            .stroked(1)
            .foreground_color(palette.white()),
    )
    .background_color(palette.dark_gray(), RoundedRectangle::new(5))
}

impl AsRef<str> for Digit {
    fn as_ref(&self) -> &str {
        match self.0 {
            0 => "0",
            1 => "1",
            2 => "2",
            3 => "3",
            4 => "4",
            5 => "5",
            6 => "6",
            7 => "7",
            8 => "8",
            9 => "9",
            _ => "?",
        }
    }
}
