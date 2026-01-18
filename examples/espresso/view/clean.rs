use crate::{AppState, color, font, spacing};
use buoyant::view::prelude::*;
use embedded_graphics::prelude::WebColors;

pub fn clean_overlay(_i: &u32) -> impl View<color::Space, AppState> + use<> {
    VStack::new((
        Text::new("Cleaning in progress...", &*font::FONT).with_font_size(font::BODY_SIZE),
        Text::new("Please wait.", &*font::FONT).with_font_size(font::CAPTION_SIZE),
        Button::new(
            |state: &mut AppState| {
                state.clean_overlay = None;
            },
            |_| Text::new("Dismiss", &*font::FONT).with_font_size(font::HEADING_SIZE),
        ),
    ))
    .with_spacing(spacing::ELEMENT)
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
}

pub fn clean_tab(_state: &crate::AppState) -> impl View<color::Space, AppState> + use<> {
    VStack::new((
        Text::new("Clean", &*font::FONT)
            .with_font_size(font::BODY_SIZE)
            .foreground_color(color::Space::CSS_ORANGE_RED)
            .padding(Edges::All, spacing::SECTION_MARGIN),
        Button::new(
            |state: &mut AppState| {
                state.clean_overlay = Some(12);
            },
            |_| {
                Text::new("Start Cleaning", &*font::FONT)
                    .with_font_size(font::HEADING_SIZE)
                    .foreground_color(color::BACKGROUND)
                    .padding(Edges::All, spacing::ELEMENT)
                    .background_color(color::Space::CSS_ORANGE_RED, RoundedRectangle::new(10))
            },
        ),
    ))
}
