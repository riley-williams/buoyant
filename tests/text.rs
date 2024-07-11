use std::iter::zip;

use buoyant::{
    environment::DefaultEnvironment,
    font::{CharacterFont, TerminalChar},
    layout::Layout as _,
    primitives::Size,
    render::Render as _,
    render_target::{FixedTextBuffer, RenderTarget as _},
    view::{HorizontalTextAlignment, Text},
};

#[derive(Debug)]
struct ArbitraryFont {
    line_height: u16,
    character_width: u16,
}

impl CharacterFont for ArbitraryFont {
    fn line_height(&self) -> u16 {
        self.line_height
    }
    fn character_width(&self, _character: char) -> u16 {
        self.character_width
    }
}

#[test]
fn test_single_character() {
    let font = ArbitraryFont {
        line_height: 10,
        character_width: 5,
    };
    let text = Text::char("A", &font);
    let offer = Size::new(100, 100);
    let env = DefaultEnvironment::new(' ');
    let layout = text.layout(offer, &env);
    assert_eq!(layout.resolved_size, Size::new(5, 10));
}

#[test]
fn test_single_character_constrained() {
    let font = ArbitraryFont {
        line_height: 10,
        character_width: 5,
    };
    let text = Text::char("A", &font);
    let offer = Size::new(4, 10);
    let env = DefaultEnvironment::new(' ');
    let layout = text.layout(offer, &env);
    assert_eq!(layout.resolved_size, Size::new(5, 10));
}

#[test]
fn test_text_layout() {
    let font = ArbitraryFont {
        line_height: 10,
        character_width: 5,
    };
    let text = Text::char("Hello, world!", &font);
    let offer = Size::new(100, 100);
    let env = DefaultEnvironment::new(' ');
    let layout = text.layout(offer, &env);
    assert_eq!(layout.resolved_size, Size::new(5 * 13, 10));
}

#[test]
fn test_text_layout_wraps() {
    let font = ArbitraryFont {
        line_height: 10,
        character_width: 5,
    };
    let text = Text::char("Hello, world!", &font);
    let offer = Size::new(50, 100);
    let env = DefaultEnvironment::new(' ');
    let layout = text.layout(offer, &env);
    assert_eq!(layout.resolved_size, Size::new(6 * 5, 20));
}

#[test]
fn test_wraps_partial_words() {
    let font = ArbitraryFont {
        line_height: 10,
        character_width: 5,
    };
    let text = Text::char("123412341234", &font);
    let offer = Size::new(20, 100);
    let env = DefaultEnvironment::new(' ');
    let layout = text.layout(offer, &env);
    assert_eq!(layout.resolved_size, Size::new(20, 30));
}

#[test]
fn test_newline() {
    let font = ArbitraryFont {
        line_height: 10,
        character_width: 5,
    };
    let text = Text::char("1234\n12\n\n123\n", &font);
    let offer = Size::new(25, 100);
    let env = DefaultEnvironment::new(' ');
    let layout = text.layout(offer, &env);
    assert_eq!(layout.resolved_size, Size::new(20, 40));
}

#[test]
fn test_render_wrapping_leading() {
    let env = DefaultEnvironment::new(' ');
    let font = TerminalChar {};
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let text = Text::char("This is a lengthy text here", &font);
    let layout = text.layout(buffer.size(), &env);
    text.render(&mut buffer, &layout, &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "This  ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "is a  ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "length");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "y text");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "here  ");
}

#[test]
fn test_render_wrapping_center_even() {
    let env = DefaultEnvironment::new(' ');
    let font = TerminalChar {};
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let text = Text::char("This is a lengthy text here", &font)
        .multiline_text_alignment(HorizontalTextAlignment::Center);
    let layout = text.layout(buffer.size(), &env);
    text.render(&mut buffer, &layout, &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), " This ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), " is a ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "length");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "y text");
    assert_eq!(buffer.text[4].iter().collect::<String>(), " here ");
}

#[test]
fn test_render_wrapping_center_odd() {
    let env = DefaultEnvironment::new(' ');
    let font = TerminalChar {};
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let text = Text::char("This is a lengthy text 12345", &font)
        .multiline_text_alignment(HorizontalTextAlignment::Center);
    let layout = text.layout(buffer.size(), &env);
    text.render(&mut buffer, &layout, &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), " This ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), " is a ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "length");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "y text");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "12345 ");
}

#[test]
fn test_render_wrapping_trailing() {
    let env = DefaultEnvironment::new(' ');
    let font = TerminalChar {};
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let text = Text::char("This is a lengthy text here", &font)
        .multiline_text_alignment(HorizontalTextAlignment::Trailing);
    let layout = text.layout(buffer.size(), &env);
    text.render(&mut buffer, &layout, &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "  This");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "  is a");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "length");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "y text");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "  here");
}

#[test]
fn test_clipped_text_is_centered_correctly() {
    let font = TerminalChar {};
    let text = Text::char(
        "Several lines\n of text\nshould be correctly spaced when cut off",
        &font,
    )
    .multiline_text_alignment(HorizontalTextAlignment::Center);

    let env = DefaultEnvironment::new(' ');
    let mut buffer = FixedTextBuffer::<40, 2>::default();

    let layout = text.layout(buffer.size(), &env);

    assert_eq!(layout.resolved_size, Size::new(13, 2));

    text.render(&mut buffer, &layout, &env);

    let lines = [
        "Several lines                           ",
        "   of text                              ",
    ];
    zip(lines.iter(), buffer.text.iter()).for_each(|(expected, actual)| {
        assert_eq!(actual.iter().collect::<String>(), *expected);
    });
}

#[test]
fn test_clipped_text_trails_correctly() {
    let font = TerminalChar {};
    let text = Text::char(
        "Several lines\n of text\nshould be correctly spaced when cut off",
        &font,
    )
    .multiline_text_alignment(HorizontalTextAlignment::Trailing);

    let env = DefaultEnvironment::new(' ');
    let mut buffer = FixedTextBuffer::<40, 2>::default();

    let layout = text.layout(buffer.size(), &env);

    assert_eq!(layout.resolved_size, Size::new(13, 2));

    text.render(&mut buffer, &layout, &env);

    let lines = [
        "Several lines                           ",
        "      of text                           ",
    ];
    zip(lines.iter(), buffer.text.iter()).for_each(|(expected, actual)| {
        assert_eq!(actual.iter().collect::<String>(), *expected);
    });
}
