use buoyant::{primitives::Point, view::prelude::*};
use embedded_graphics::{
    Drawable,
    geometry::Point as EgPoint,
    mock_display::MockDisplay,
    mono_font::{MonoTextStyle, ascii::FONT_7X13},
    pixelcolor::Rgb888,
    prelude::WebColors,
    text::Text as EgText,
};
use u8g2_fonts::{FontRenderer, fonts, types::FontColor};

use super::render_to_mock;

mod precise_bounds;

#[test]
fn embedded_graphics_mono_font() {
    let view = Text::new("Test.\n12 3", &FONT_7X13).foreground_color(Rgb888::CSS_OLD_LACE);

    let display = render_to_mock(&view, false);

    let mut display_2 = MockDisplay::new();
    let style = MonoTextStyle::new(&FONT_7X13, Rgb888::CSS_OLD_LACE);
    EgText::new("Test.\n12 3", EgPoint::new(0, 10), style)
        .draw(&mut display_2)
        .unwrap();
    display.assert_eq(&display_2);
}

#[test]
fn u8g2_font() {
    let text = "Test.\n12 3";
    let font = FontRenderer::new::<fonts::u8g2_font_haxrcorp4089_t_cyrillic>();
    let view = Text::new(text, &font)
        .foreground_color(Rgb888::CSS_SPRING_GREEN)
        .padding(Edges::All, 1);

    let display = render_to_mock(&view, true);

    let mut display_2 = MockDisplay::new();
    display_2.set_allow_overdraw(true);
    font.render(
        text,
        Point::new(1, 1).into(),
        u8g2_fonts::types::VerticalPosition::Top,
        FontColor::Transparent(Rgb888::CSS_SPRING_GREEN),
        &mut display_2,
    )
    .unwrap();

    display.assert_eq(&display_2);
}
