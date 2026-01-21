use crate::{AppState, CleanSettings, color, font, spacing};
use buoyant::{
    match_view,
    transition::Move,
    view::{
        Paginate,
        paginate::{PageEvent, PageState},
        prelude::*,
    },
};
use embedded_graphics::prelude::WebColors;

pub fn clean_tab(_state: &crate::AppState) -> impl View<color::Space, AppState> + use<> {
    VStack::new((
        Text::new("Clean", &*font::FONT)
            .with_font_size(font::BODY_SIZE)
            .foreground_color(color::Space::CSS_ORANGE_RED)
            .padding(Edges::All, spacing::SECTION_MARGIN),
        Button::new(
            move |state: &mut AppState| {
                // Clone to edit a copy of the settings
                state.clean_overlay = Some(state.clean_settings.clone());
            },
            |_| {
                Text::new("Change Settings", &*font::FONT)
                    .with_font_size(font::HEADING_SIZE)
                    .foreground_color(color::BACKGROUND)
                    .padding(Edges::All, spacing::ELEMENT)
                    .background_color(color::Space::CSS_ORANGE_RED, RoundedRectangle::new(10))
            },
        ),
    ))
}

pub fn clean_overlay(settings: &CleanSettings) -> impl View<color::Space, AppState> + use<> {
    VStack::new((
        Text::new("Change Cleaning Settings", &*font::FONT).with_font_size(font::BODY_SIZE),
        Lens::new(select_amount(settings), |s: &mut AppState| {
            // TODO: Maybe there's a better abstraction buoyant can provide for this pattern?
            if let Some(settings) = &mut s.clean_overlay {
                settings
            } else {
                &mut s.clean_settings
            }
        }),
        HStack::new((
            Button::new(
                |state: &mut AppState| {
                    state.clean_overlay = None;
                },
                |_| Text::new("Cancel", &*font::FONT).with_font_size(font::BODY_SIZE),
            ),
            Button::new(
                |state: &mut AppState| {
                    if let Some(settings) = &state.clean_overlay {
                        state.clean_settings = settings.clone();
                    }
                    state.clean_overlay = None;
                },
                |_| Text::new("Save", &*font::FONT).with_font_size(font::BODY_SIZE),
            ),
        ))
        .with_spacing(spacing::COMPONENT),
    ))
    .with_spacing(spacing::SECTION)
    .foreground_color(color::ACCENT)
    .flex_frame()
    .with_infinite_max_dimensions()
    .background_color(color::BACKGROUND_SECONDARY, Rectangle.corner_radius(10))
    .overlay(
        Alignment::Center,
        Rectangle
            .corner_radius(15)
            .stroked_offset(5, StrokeOffset::Outer)
            .foreground_color(color::FOREGROUND_SECONDARY),
    )
    .padding(Edges::All, spacing::SECTION_MARGIN)
    .transition(Move::top())
}

pub fn select_amount(settings: &CleanSettings) -> impl View<color::Space, CleanSettings> + use<> {
    HStack::new((
        Text::new("Select Amount:", &*font::FONT).with_font_size(font::BODY_SIZE),
        Lens::new(int_selector(settings.time), |s: &mut CleanSettings| {
            &mut s.time
        }),
        Lens::new(int_selector(settings.frequency), |s: &mut CleanSettings| {
            &mut s.frequency
        }),
    ))
    .with_spacing(spacing::COMPONENT)
}

pub fn int_selector(value: u32) -> impl View<color::Space, u32> + use<> {
    Paginate::new(
        |value: &mut u32, event: &PageEvent| match event {
            PageEvent::Next => {
                *value = value.saturating_add(1);
            }
            PageEvent::Previous => {
                *value = value.saturating_sub(1);
            }
            PageEvent::Focused | PageEvent::Exit => {}
        },
        move |state| {
            Text::new(value.to_string(), &*font::FONT)
                .with_font_size(font::BODY_SIZE)
                .with_precise_bounds()
                .padding(Edges::All, 4)
                .foreground_color(if *state == PageState::Captive {
                    color::Space::CSS_ORANGE_RED
                } else {
                    color::ACCENT
                })
                .background(
                    Alignment::Center,
                    match_view!(state, {
                        PageState::UnFocused => EmptyView,
                        PageState::Focused => RoundedRectangle::new(4).stroked(2).foreground_color(color::ACCENT),
                        PageState::Captive => RoundedRectangle::new(4).stroked(2).foreground_color(color::Space::CSS_ORANGE_RED),
                        }
                    ),
                )
        },
    )
}
