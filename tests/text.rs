use std::iter::zip;

use buoyant::{
    environment::DefaultEnvironment,
    font::{BufferCharacterFont, CharacterFont, FontLayout},
    layout::Layout as _,
    primitives::{Point, Size},
    render::CharacterRender,
    render_target::{CharacterRenderTarget, FixedTextBuffer},
    view::{HorizontalTextAlignment, LayoutExtensions as _, Text},
};

#[derive(Debug)]
struct ArbitraryFont {
    line_height: u16,
    character_width: u16,
}

impl FontLayout for ArbitraryFont {
    fn line_height(&self) -> u16 {
        self.line_height
    }
    fn character_width(&self, _character: char) -> u16 {
        self.character_width
    }
}

impl CharacterFont<char> for ArbitraryFont {
    fn render_iter<T, I>(&self, _target: &mut T, _origin: Point, _characters: I)
    where
        T: buoyant::render_target::CharacterRenderTarget<Color = char>,
        I: IntoIterator<Item = (char, char)>,
    {
        panic!("Not renderable");
    }
}

#[test]
fn test_single_character() {
    let font = ArbitraryFont {
        line_height: 10,
        character_width: 5,
    };
    let text = Text::str("A", &font);
    let offer = Size::new(100, 100);
    let env = DefaultEnvironment::new(());
    let layout = text.layout(offer, &env);
    assert_eq!(layout.resolved_size, Size::new(5, 10));
}

#[test]
fn test_single_character_constrained() {
    let font = ArbitraryFont {
        line_height: 10,
        character_width: 5,
    };
    let text = Text::str("A", &font);
    let offer = Size::new(4, 10);
    let env = DefaultEnvironment::new(());
    let layout = text.layout(offer, &env);
    assert_eq!(layout.resolved_size, Size::new(5, 10));
}

#[test]
fn test_text_layout() {
    let font = ArbitraryFont {
        line_height: 10,
        character_width: 5,
    };
    let text = Text::str("Hello, world!", &font);
    let offer = Size::new(100, 100);
    let env = DefaultEnvironment::new(());
    let layout = text.layout(offer, &env);
    assert_eq!(layout.resolved_size, Size::new(5 * 13, 10));
}

#[test]
fn test_text_layout_wraps() {
    let font = ArbitraryFont {
        line_height: 10,
        character_width: 5,
    };
    let text = Text::str("Hello, world!", &font);
    let offer = Size::new(50, 100);
    let env = DefaultEnvironment::new(());
    let layout = text.layout(offer, &env);
    assert_eq!(layout.resolved_size, Size::new(6 * 5, 20));
}

#[test]
fn test_wraps_partial_words() {
    let font = ArbitraryFont {
        line_height: 10,
        character_width: 5,
    };
    let text = Text::str("123412341234", &font);
    let offer = Size::new(20, 100);
    let env = DefaultEnvironment::new(());
    let layout = text.layout(offer, &env);
    assert_eq!(layout.resolved_size, Size::new(20, 30));
}

#[test]
fn test_newline() {
    let font = ArbitraryFont {
        line_height: 10,
        character_width: 5,
    };
    let text = Text::str("1234\n12\n\n123\n", &font);
    let offer = Size::new(25, 100);
    let env = DefaultEnvironment::new(());
    let layout = text.layout(offer, &env);
    assert_eq!(layout.resolved_size, Size::new(20, 40));
}

#[test]
fn test_render_wrapping_leading() {
    let env = DefaultEnvironment::new(());
    let font = BufferCharacterFont {};
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let text = Text::str("This is a lengthy text here", &font);
    let layout = text.layout(buffer.size(), &env);
    text.render(&mut buffer, &layout, Point::zero(), &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "This  ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "is a  ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "length");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "y text");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "here  ");
}

#[test]
fn test_render_wrapping_center_even() {
    let env = DefaultEnvironment::new(());
    let font = BufferCharacterFont {};
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let text = Text::str("This is a lengthy text here", &font)
        .multiline_text_alignment(HorizontalTextAlignment::Center);
    let layout = text.layout(buffer.size(), &env);
    text.render(&mut buffer, &layout, Point::zero(), &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), " This ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), " is a ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "length");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "y text");
    assert_eq!(buffer.text[4].iter().collect::<String>(), " here ");
}

#[test]
fn test_render_wrapping_center_odd() {
    let env = DefaultEnvironment::new(());
    let font = BufferCharacterFont {};
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let text = Text::str("This is a lengthy text 12345", &font)
        .multiline_text_alignment(HorizontalTextAlignment::Center);
    let layout = text.layout(buffer.size(), &env);
    text.render(&mut buffer, &layout, Point::zero(), &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), " This ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), " is a ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "length");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "y text");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "12345 ");
}

#[test]
fn test_render_wrapping_trailing() {
    let env = DefaultEnvironment::new(());
    let font = BufferCharacterFont {};
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let text = Text::str("This is a lengthy text here", &font)
        .multiline_text_alignment(HorizontalTextAlignment::Trailing);
    let layout = text.layout(buffer.size(), &env);
    text.render(&mut buffer, &layout, Point::zero(), &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "  This");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "  is a");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "length");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "y text");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "  here");
}

#[test]
fn test_clipped_text_is_centered_correctly() {
    let font = BufferCharacterFont {};
    let text = Text::str(
        "Several lines\n of text\nshould be correctly spaced when cut off",
        &font,
    )
    .multiline_text_alignment(HorizontalTextAlignment::Center);

    let env = DefaultEnvironment::new(());
    let mut buffer = FixedTextBuffer::<40, 2>::default();

    let layout = text.layout(buffer.size(), &env);

    assert_eq!(layout.resolved_size, Size::new(13, 2));

    text.render(&mut buffer, &layout, Point::zero(), &env);

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
    let font = BufferCharacterFont {};
    let text = Text::str(
        "Several lines\n of text\nshould be correctly spaced when cut off",
        &font,
    )
    .multiline_text_alignment(HorizontalTextAlignment::Trailing)
    .frame(None, Some(2), None, None); // constrain to 2 pts tall

    let env = DefaultEnvironment::new(());
    let mut buffer = FixedTextBuffer::<40, 3>::default();

    let layout = text.layout(buffer.size(), &env);

    assert_eq!(layout.resolved_size, Size::new(13, 2));

    text.render(&mut buffer, &layout, Point::zero(), &env);

    let lines = [
        "Several lines                           ",
        "      of text                           ",
        "                                        ",
    ];
    zip(lines.iter(), buffer.text.iter()).for_each(|(expected, actual)| {
        assert_eq!(actual.iter().collect::<String>(), *expected);
    });
}
