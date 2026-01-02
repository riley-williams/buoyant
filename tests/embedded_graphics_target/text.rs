use buoyant::{primitives::Point, view::prelude::*};
use embedded_graphics::{
    Drawable,
    geometry::Point as EgPoint,
    mock_display::MockDisplay,
    mono_font::{MonoTextStyle, ascii::FONT_7X13},
    pixelcolor::Rgb888,
    prelude::{RgbColor, WebColors},
    text::{Text as EgText, renderer::TextRenderer},
};
use embedded_ttf::FontTextStyleBuilder;
use u8g2_fonts::{FontRenderer, fonts, types::FontColor};

use super::render_to_mock;

use std::sync::LazyLock;

mod precise_bounds_character_wrap;
mod precise_bounds_word_wrap;

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

#[test]
fn rusttype_font() {
    static SNIGLET_FONT: LazyLock<rusttype::Font<'static>> = LazyLock::new(|| {
        let bytes = include_bytes!("assets/fonts/Sniglet Regular.otf");
        rusttype::Font::try_from_bytes(bytes).unwrap()
    });

    let text = "T";
    let view = Text::new(text, &*SNIGLET_FONT)
        .with_font_size(12)
        .foreground_color(Rgb888::CSS_TOMATO);

    let display = render_to_mock(&view, true);

    let mut display_2 = MockDisplay::new();
    let font_style = FontTextStyleBuilder::new(SNIGLET_FONT.clone())
        .font_size(12)
        .text_color(Rgb888::CSS_TOMATO)
        .anti_aliasing_color(Rgb888::BLACK)
        .build();

    font_style
        .draw_string(
            text,
            EgPoint::new(0, 0),
            embedded_graphics::text::Baseline::Top,
            &mut display_2,
        )
        .unwrap();

    display.assert_eq(&display_2);
}

#[ignore = "Special spacing between characters is not respected, but could be supported in the future"]
#[test]
fn rusttype_font_kerning() {
    static SNIGLET_FONT: LazyLock<rusttype::Font<'static>> = LazyLock::new(|| {
        let bytes = include_bytes!("assets/fonts/Sniglet Regular.otf");
        rusttype::Font::try_from_bytes(bytes).unwrap()
    });

    let text = "Test";
    let view = Text::new(text, &*SNIGLET_FONT)
        .with_font_size(12)
        .foreground_color(Rgb888::CSS_TOMATO);

    let display = render_to_mock(&view, true);

    let mut display_2 = MockDisplay::new();
    let font_style = FontTextStyleBuilder::new(SNIGLET_FONT.clone())
        .font_size(12)
        .text_color(Rgb888::CSS_TOMATO)
        .anti_aliasing_color(Rgb888::BLACK)
        .build();

    font_style
        .draw_string(
            text,
            EgPoint::new(0, 0),
            embedded_graphics::text::Baseline::Top,
            &mut display_2,
        )
        .unwrap();

    display.assert_eq(&display_2);
}

/// this test only works with some font sizes 🤔 maybe due to the rusttype advance being a float?
#[test]
fn rusttype_font_lines_non_overlapping() {
    static FONT: LazyLock<rusttype::Font<'static>> = LazyLock::new(|| {
        let bytes = include_bytes!("./assets/fonts/Sniglet Regular.otf");
        rusttype::Font::try_from_bytes(bytes).unwrap()
    });

    let text = "pg\nhI";
    let view = Text::new(text, &*FONT)
        .with_font_size(24)
        .foreground_color(Rgb888::CSS_MEDIUM_AQUAMARINE);

    // Panics if there is overlapping drawing
    let _ = render_to_mock(&view, false);
}
