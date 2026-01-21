use crate::{AppState, color, font, spacing};
use buoyant::animation::Animation;
use buoyant::if_view;
use buoyant::primitives::UnitPoint;
use buoyant::transition::{Edge, Move};
use buoyant::view::prelude::*;
use embedded_graphics::prelude::{RgbColor, WebColors};
use std::time::Duration;

pub fn settings_tab(state: &AppState) -> impl View<color::Space, AppState> + use<> {
    ScrollView::new(
        VStack::new((
            Text::new("Settings", &*font::FONT)
                .with_font_size(font::HEADING_SIZE)
                .foreground_color(color::Space::WHITE),
            toggle_text(
                "Auto brew",
                state.auto_brew,
                "Automatically brew coffee at 7am",
                true,
                |state: &mut AppState| {
                    state.auto_brew = !state.auto_brew;
                },
            ),
            Rectangle
                .frame_sized(10, 100)
                .foreground_color(color::Space::CSS_LIGHT_GRAY)
                .opacity(128),
            toggle_text(
                "Stop on weight",
                state.stop_on_weight,
                "Stop the machine automatically when the target weight is reached",
                false,
                |state: &mut AppState| {
                    state.stop_on_weight = !state.stop_on_weight;
                },
            ),
            toggle_text(
                "Auto off",
                state.auto_off,
                "The display will go to sleep after 5 minutes of inactivity",
                true,
                |state: &mut AppState| {
                    state.auto_off = !state.auto_off;
                },
            ),
        ))
        .with_spacing(spacing::COMPONENT)
        .with_alignment(HorizontalAlignment::Trailing)
        .padding(Edges::All, spacing::SECTION_MARGIN)
        .animated(Animation::linear(Duration::from_millis(200)), state.clone()),
    )
    .with_overlapping_bar(true) // we already applied padding
}

fn toggle_text<C>(
    label: &'static str,
    is_on: bool,
    description: &'static str,
    hides_description: bool,
    action: fn(&mut C),
) -> impl View<color::Space, C> + use<C> {
    VStack::new((
        HStack::new((
            Text::new(label, &*font::FONT)
                .with_font_size(font::BODY_SIZE)
                .foreground_color(color::Space::WHITE),
            toggle_button(is_on, action),
        ))
        .with_spacing(spacing::ELEMENT),
        if_view!((is_on || !hides_description) {
            Text::new(description, &*font::FONT)
                .with_font_size(font::CAPTION_SIZE)
                .multiline_text_alignment(HorizontalTextAlignment::Trailing)
                .foreground_color(color::Space::WHITE)
                .transition(Move::new(Edge::Trailing))
        }),
    ))
    .with_spacing(spacing::ELEMENT)
    .with_alignment(HorizontalAlignment::Trailing)
    .flex_infinite_width(HorizontalAlignment::Trailing)
}

fn toggle_button<C>(is_on: bool, on_tap: fn(&mut C)) -> impl View<color::Space, C> + use<C> {
    let (color, alignment) = if is_on {
        (color::ACCENT, HorizontalAlignment::Trailing)
    } else {
        (color::Space::CSS_LIGHT_GRAY, HorizontalAlignment::Leading)
    };

    Button::new(on_tap, move |is_pressed: bool| {
        ZStack::new((
            buoyant::view::shape::Capsule.foreground_color(color),
            buoyant::view::shape::Circle
                .foreground_color(if is_pressed {
                    color::Space::CSS_LIGHT_GRAY
                } else {
                    color::Space::CSS_WHITE
                })
                .scale_effect(if is_pressed { 1.5 } else { 1.0 }, UnitPoint::center())
                .padding(Edges::All, 2)
                .animated(Animation::linear(Duration::from_millis(125)), is_on),
        ))
        .with_horizontal_alignment(alignment)
        .frame_sized(50, 25)
        .geometry_group()
    })
}
