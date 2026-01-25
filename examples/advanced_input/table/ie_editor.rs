use crate::FONT;
use crate::definitions::{GoodPixelColor, Palette, State, TemporaryIe};
use buoyant::view::prelude::*;
use buoyant::view::rotary::{Rotary, RotaryEvent, RotaryState};

#[derive(Copy, Clone)]
struct Digit(u8);

fn digit_editor<C: GoodPixelColor>(
    value: u8,
    set: impl Fn(&mut State, u8) + 'static,
    palette: &'static Palette<C>,
) -> impl View<C, State> {
    let on_action = move |s: &mut State, event: RotaryEvent| match event {
        RotaryEvent::Previous => set(s, if value == 9 { 0 } else { value + 1 }),
        RotaryEvent::Next => set(s, if value == 0 { 9 } else { value - 1 }),
        _ => (),
    };

    Rotary::new(on_action, move |page_state: RotaryState| {
        let is_focused = page_state == RotaryState::Focused || page_state == RotaryState::Captive;
        button_gut(is_focused, palette, Text::new(Digit(value), &FONT))
    })
}

fn flag_editor<C: GoodPixelColor>(
    label: &'static str,
    label_false: &'static str,
    value: bool,
    set: impl Fn(&mut State, bool) + 'static,
    palette: &'static Palette<C>,
) -> impl View<C, State> {
    let on_action = move |s: &mut State, event: RotaryEvent| match event {
        RotaryEvent::Previous | RotaryEvent::Next => set(s, !value),
        _ => (),
    };

    Rotary::new(on_action, move |page_state: RotaryState| {
        let is_focused = page_state == RotaryState::Focused || page_state == RotaryState::Captive;
        let text = if value { label } else { label_false };
        button_gut(is_focused, palette, Text::new(text, &FONT))
    })
}

fn button_gut<C: GoodPixelColor>(
    is_focused: bool,
    palette: &'static Palette<C>,
    inner: impl View<C, State>,
) -> impl View<C, State> {
    let inner = inner.padding(Edges::All, 3);
    buoyant::if_view!((is_focused) {
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
    _cancel: impl Fn(&mut State),
    palette: &'static Palette<C>,
) -> impl View<C, State> {
    fn set_int<const I: usize>(s: &mut State, v: u8) {
        const { assert!(I < 3) };

        s.temporary_ie.int[I] = v;
    }
    fn set_frac<const I: usize>(s: &mut State, v: u8) {
        const { assert!(I < 3) };

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

    VStack::new((
        value_row,
        Spacer::default().frame().with_height(8),
        Spacer::default().frame().with_height(8),
        Button::new(submit, move |s| {
            button_gut(s.is_focused(), palette, Text::new("Submit", &FONT))
        }),
    ))
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
