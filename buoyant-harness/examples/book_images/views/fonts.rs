use buoyant::view::prelude::*;
use embedded_graphics::{
    mono_font::ascii::{FONT_6X10, FONT_9X15, FONT_10X20},
    pixelcolor::Rgb888,
    prelude::*,
};
use u8g2_fonts::{FontRenderer, fonts};

pub fn monospace() -> impl View<Rgb888, ()> {
    VStack::new((
        Text::new("Small (6x10)", &FONT_6X10).foreground_color(Rgb888::CSS_PALE_GREEN),
        Text::new("Medium (9x15)", &FONT_9X15).foreground_color(Rgb888::CSS_LIGHT_SKY_BLUE),
        Text::new("Large (10x20)", &FONT_10X20).foreground_color(Rgb888::CSS_LIGHT_CORAL),
    ))
    .with_spacing(20)
    .with_alignment(HorizontalAlignment::Center)
    .flex_infinite_width(HorizontalAlignment::Center)
    .padding(Edges::All, 20)
}

static HELVETICA: FontRenderer = FontRenderer::new::<fonts::u8g2_font_helvR12_tr>();
static HELVETICA_BOLD: FontRenderer = FontRenderer::new::<fonts::u8g2_font_helvB12_tr>();
static PROFONT_22: FontRenderer = FontRenderer::new::<fonts::u8g2_font_profont22_mr>();
static MYSTERY_QUEST_28: FontRenderer = FontRenderer::new::<fonts::u8g2_font_mystery_quest_28_tr>();
static GREENBLOOD: FontRenderer = FontRenderer::new::<fonts::u8g2_font_greenbloodserif2_tr>();
static TOM_THUMB: FontRenderer = FontRenderer::new::<fonts::u8g2_font_tom_thumb_4x6_mr>();

pub fn u8g2() -> impl View<Rgb888, ()> {
    VStack::new((
        Text::new("Helvetica 12pt", &HELVETICA).foreground_color(Rgb888::CSS_ORANGE_RED),
        Text::new("Helvetica 12pt Bold", &HELVETICA_BOLD).foreground_color(Rgb888::CSS_ORANGE),
        Text::new("ProFont 22pt", &PROFONT_22).foreground_color(Rgb888::CSS_LIGHT_SKY_BLUE),
        Text::new("Mystery Quest 28pt", &MYSTERY_QUEST_28)
            .foreground_color(Rgb888::CSS_LIGHT_CORAL),
        Text::new("Green Blood 16pt", &GREENBLOOD).foreground_color(Rgb888::CSS_PALE_GREEN),
        Text::new("Tom Thumb (tiny)", &TOM_THUMB).foreground_color(Rgb888::CSS_YELLOW),
    ))
    .with_spacing(20)
    .with_alignment(HorizontalAlignment::Center)
    .flex_infinite_width(HorizontalAlignment::Center)
    .padding(Edges::All, 20)
}
