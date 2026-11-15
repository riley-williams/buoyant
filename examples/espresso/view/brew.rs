use crate::{AppState, color, font, spacing};
use buoyant::view::prelude::*;
use buoyant::view::scroll_view::ScrollDirection;
use embedded_graphics::prelude::RgbColor;

pub fn brew_tab(_state: &AppState) -> impl View<color::Space, AppState> + use<> {
    ScrollView::new(
        VStack::new((
            Text::new("Good morning", &*font::FONT).with_font_size(font::HEADING_SIZE),
            HStack::new((
                gauge("Temperature", "°C", 93, 100),
                gauge("Pressure", "bar", 9, 12),
            ))
            .with_spacing(spacing::SECTION),
            Text::new(
                "You can't brew coffee in a simulator, but you can pretend.",
                &font::MYSTERY_QUEST_28,
            )
            .multiline_text_alignment(HorizontalTextAlignment::Center),
        ))
        .with_spacing(spacing::COMPONENT)
        .with_alignment(HorizontalAlignment::Center)
        .flex_infinite_width(HorizontalAlignment::Center)
        .padding(Edges::All, spacing::SECTION_MARGIN)
        .foreground_color(color::Space::WHITE),
    )
    .with_direction(ScrollDirection::Both)
}

/// A 270° gauge showing `value` as a proportion of `max`
fn gauge(
    label: &'static str,
    unit: &'static str,
    value: u8,
    max: u8,
) -> impl View<color::Space, AppState> + use<> {
    const START: i32 = 135;
    const SWEEP: i32 = 270;
    let end = START + SWEEP * i32::from(value.min(max)) / i32::from(max.max(1));

    VStack::new((
        ZStack::new((
            Arc::new(Angle::degrees(START), Angle::degrees(START + SWEEP))
                .stroked(10)
                .foreground_color(color::BACKGROUND_SECONDARY),
            Arc::new(Angle::degrees(START), Angle::degrees(end))
                .stroked(10)
                .foreground_color(color::ACCENT),
            VStack::new((
                Text::new_fmt::<4>(format_args!("{value}"), &*font::FONT)
                    .with_font_size(font::HEADING_SIZE),
                Text::new(unit, &*font::FONT)
                    .with_font_size(font::CAPTION_SIZE)
                    .foreground_color(color::FOREGROUND_SECONDARY),
            ))
            .with_alignment(HorizontalAlignment::Center),
        ))
        .frame_sized(120, 120),
        Text::new(label, &*font::FONT).with_font_size(font::CAPTION_SIZE),
    ))
    .with_spacing(spacing::ELEMENT)
    .with_alignment(HorizontalAlignment::Center)
}
