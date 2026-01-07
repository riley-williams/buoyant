use crate::{AppState, color, font, spacing};
use buoyant::view::prelude::*;
use buoyant::view::scroll_view::ScrollDirection;
use embedded_graphics::prelude::RgbColor;

pub fn brew_tab(_state: &AppState) -> impl View<color::Space, AppState> + use<> {
    ScrollView::new(
        VStack::new((
            Text::new("Good morning", &*font::FONT).with_font_size(font::HEADING_SIZE),
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
