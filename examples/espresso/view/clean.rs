use crate::{color, font, spacing};
use buoyant::view::prelude::*;
use embedded_graphics::prelude::WebColors;

pub fn clean_tab<C>(_state: &crate::AppState) -> impl View<color::Space, C> + use<C> {
    Text::new("Clean", &*font::FONT)
        .with_font_size(font::BODY_SIZE)
        .foreground_color(color::Space::CSS_ORANGE_RED)
        .padding(Edges::All, spacing::SECTION_MARGIN)
}
