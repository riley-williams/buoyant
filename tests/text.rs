use buoyant::{
    font::{Font, TextBufferFont},
    layout::{Environment, Layout as _},
    primitives::Size,
    render::Render as _,
    render_target::{FixedTextBuffer, RenderTarget as _},
    view::{HorizontalTextAlignment, Text},
};

#[derive(Debug)]
struct MonospaceFont {
    line_height: u16,
    character_width: u16,
}

impl Font for MonospaceFont {
    fn line_height(&self) -> u16 {
        self.line_height
    }
    fn character_width(&self, _character: char) -> u16 {
        self.character_width
    }
}

struct TestEnv;
impl Environment for TestEnv {}

#[test]
fn test_single_character() {
    let font = MonospaceFont {
        line_height: 10,
        character_width: 5,
    };
    let text = Text::new("A", font);
    let offer = Size::new(100, 100);
    let layout = text.layout(offer, &TestEnv);
    assert_eq!(layout.resolved_size, Size::new(5, 10));
}

#[test]
fn test_single_character_constrained() {
    let font = MonospaceFont {
        line_height: 10,
        character_width: 5,
    };
    let text = Text::new("A", font);
    let offer = Size::new(4, 10);
    let layout = text.layout(offer, &TestEnv);
    assert_eq!(layout.resolved_size, Size::new(5, 10));
}

#[test]
fn test_text_layout() {
    let font = MonospaceFont {
        line_height: 10,
        character_width: 5,
    };
    let text = Text::new("Hello, world!", font);
    let offer = Size::new(100, 100);
    let layout = text.layout(offer, &TestEnv);
    assert_eq!(layout.resolved_size, Size::new(5 * 13, 10));
}

#[test]
fn test_text_layout_wraps() {
    let font = MonospaceFont {
        line_height: 10,
        character_width: 5,
    };
    let text = Text::new("Hello, world!", font);
    let offer = Size::new(50, 100);
    let layout = text.layout(offer, &TestEnv);
    assert_eq!(layout.resolved_size, Size::new(6 * 5, 20));
}

#[test]
fn test_wraps_partial_words() {
    let font = MonospaceFont {
        line_height: 10,
        character_width: 5,
    };
    let text = Text::new("123412341234", font);
    let offer = Size::new(20, 100);
    let layout = text.layout(offer, &TestEnv);
    assert_eq!(layout.resolved_size, Size::new(20, 30));
}

#[test]
fn test_newline() {
    let font = MonospaceFont {
        line_height: 10,
        character_width: 5,
    };
    let text = Text::new("1234\n12\n\n123\n", font);
    let offer = Size::new(25, 100);
    let layout = text.layout(offer, &TestEnv);
    assert_eq!(layout.resolved_size, Size::new(20, 40));
}

#[test]
fn test_size_is_correct_after_exceeding_cache() {
    let font = MonospaceFont {
        line_height: 10,
        character_width: 5,
    };
    let text = Text::new("1234\n12\n\n123\n", font).max_lines::<1>();
    let offer = Size::new(25, 100);
    let layout = text.layout(offer, &TestEnv);
    assert_eq!(layout.resolved_size, Size::new(20, 40));
}

#[test]
fn test_render_wrapping_leading() {
    let env = TestEnv {};
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let text = Text::new("This is a lengthy text here", TextBufferFont {}).max_lines::<8>();
    let layout = text.layout(buffer.size(), &env);
    text.render(
        &mut buffer,
        &layout.layout_cache,
        layout.resolved_size,
        &env,
    );
    assert_eq!(buffer.text[0].iter().collect::<String>(), "This  ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "is a  ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "length");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "y text");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "here  ");
}

#[test]
fn test_render_wrapping_center_even() {
    let env = TestEnv {};
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let text = Text::new("This is a lengthy text here", TextBufferFont {})
        .max_lines::<8>()
        .multiline_text_alignment(HorizontalTextAlignment::Center);
    let layout = text.layout(buffer.size(), &env);
    text.render(
        &mut buffer,
        &layout.layout_cache,
        layout.resolved_size,
        &env,
    );
    assert_eq!(buffer.text[0].iter().collect::<String>(), " This ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), " is a ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "length");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "y text");
    assert_eq!(buffer.text[4].iter().collect::<String>(), " here ");
}

#[test]
fn test_render_wrapping_center_odd() {
    let env = TestEnv {};
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let text = Text::new("This is a lengthy text 12345", TextBufferFont {})
        .max_lines::<8>()
        .multiline_text_alignment(HorizontalTextAlignment::Center);
    let layout = text.layout(buffer.size(), &env);
    text.render(
        &mut buffer,
        &layout.layout_cache,
        layout.resolved_size,
        &env,
    );
    assert_eq!(buffer.text[0].iter().collect::<String>(), " This ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), " is a ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "length");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "y text");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "12345 ");
}

#[test]
fn test_render_wrapping_trailing() {
    let env = TestEnv {};
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let text = Text::new("This is a lengthy text here", TextBufferFont {})
        .max_lines::<8>()
        .multiline_text_alignment(HorizontalTextAlignment::Trailing);
    let layout = text.layout(buffer.size(), &env);
    text.render(
        &mut buffer,
        &layout.layout_cache,
        layout.resolved_size,
        &env,
    );
    assert_eq!(buffer.text[0].iter().collect::<String>(), "  This");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "  is a");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "length");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "y text");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "  here");
}

#[test]
fn test_renders_beyond_cache() {
    let env = TestEnv {};
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let text = Text::new("This is a lengthy text here", TextBufferFont {}).max_lines::<2>();
    let layout = text.layout(buffer.size(), &env);
    text.render(
        &mut buffer,
        &layout.layout_cache,
        layout.resolved_size,
        &env,
    );
    assert_eq!(buffer.text[0].iter().collect::<String>(), "This  ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "is a  ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "length");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "y text");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "here  ");
}

#[test]
fn test_renders_beyond_cache_do_render_extra_spaces() {
    let env = TestEnv {};
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let text = Text::new("This is a length text here", TextBufferFont {}).max_lines::<3>();
    let layout = text.layout(buffer.size(), &env);
    text.render(
        &mut buffer,
        &layout.layout_cache,
        layout.resolved_size,
        &env,
    );
    assert_eq!(buffer.text[0].iter().collect::<String>(), "This  ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "is a  ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "length");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "text  "); // no space at the start
    assert_eq!(buffer.text[4].iter().collect::<String>(), "here  ");
}
#[test]
fn test_render_beyond_cache_wrapping_center_even() {
    let env = TestEnv {};
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let text = Text::new("This is a lengthy text here", TextBufferFont {})
        .max_lines::<1>()
        .multiline_text_alignment(HorizontalTextAlignment::Center);
    let layout = text.layout(buffer.size(), &env);
    text.render(
        &mut buffer,
        &layout.layout_cache,
        layout.resolved_size,
        &env,
    );
    assert_eq!(buffer.text[0].iter().collect::<String>(), " This ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), " is a ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "length");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "y text");
    assert_eq!(buffer.text[4].iter().collect::<String>(), " here ");
}

#[test]
fn test_render_beyond_cache_wrapping_center_odd() {
    let env = TestEnv {};
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let text = Text::new("This is a lengthy text 12345", TextBufferFont {})
        .max_lines::<0>()
        .multiline_text_alignment(HorizontalTextAlignment::Center);
    let layout = text.layout(buffer.size(), &env);
    text.render(
        &mut buffer,
        &layout.layout_cache,
        layout.resolved_size,
        &env,
    );
    assert_eq!(buffer.text[0].iter().collect::<String>(), " This ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), " is a ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "length");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "y text");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "12345 ");
}

#[test]
fn test_render_beyond_cache_wrapping_trailing() {
    let env = TestEnv {};
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let text = Text::new("This is a lengthy text here", TextBufferFont {})
        .max_lines::<1>()
        .multiline_text_alignment(HorizontalTextAlignment::Trailing);
    let layout = text.layout(buffer.size(), &env);
    text.render(
        &mut buffer,
        &layout.layout_cache,
        layout.resolved_size,
        &env,
    );
    assert_eq!(buffer.text[0].iter().collect::<String>(), "  This");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "  is a");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "length");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "y text");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "  here");
}
