use buoyant::environment::DefaultEnvironment;
use buoyant::font::CharacterBufferFont;
use buoyant::layout::Layout;
use buoyant::match_view;
use buoyant::primitives::{Point, Size};
use buoyant::render::CharacterRender;
use buoyant::render::{CharacterRenderTarget, Renderable};
use buoyant::render_target::FixedTextBuffer;
use buoyant::view::{RenderExtensions as _, Text};
use std::string::String;

#[test]
fn test_match_view_two_variants() {
    let font = CharacterBufferFont;
    let make_view = |state| {
        match_view!(state => {
            0 => Text::str("zero\n!!!", &font),
            _ => Text::str("other", &font).foreground_color(' '),
        })
    };
    let mut buffer = FixedTextBuffer::<5, 5>::default();
    let env = DefaultEnvironment::default();

    let view = make_view(0);
    let layout = view.layout(&buffer.size().into(), &env);
    assert_eq!(layout.resolved_size, Size::new(4, 2).into());
    let tree = view.render_tree(&layout, Point::zero(), &env);
    tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "zero ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "!!!  ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "     ");

    buffer.clear();

    let view = make_view(1);
    let layout = view.layout(&buffer.size().into(), &env);
    assert_eq!(layout.resolved_size, Size::new(5, 1).into());
    let tree = view.render_tree(&layout, Point::zero(), &env);
    tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "other");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "     ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "     ");
}

#[test]
fn test_match_view_three_variants() {
    #[derive(Clone)]
    enum State {
        A,
        B(&'static str),
        C,
    }

    let font = CharacterBufferFont;

    let make_view = |state| {
        match_view!(state => {
            State::A => Text::str("AAA", &font),
            State::B(msg) => Text::str(msg, &font),
            State::C => Text::str("CCC", &font),
        })
        .foreground_color(' ')
    };
    let mut buffer = FixedTextBuffer::<5, 5>::default();
    let env = DefaultEnvironment::default();

    let view = make_view(State::A);
    let layout = view.layout(&buffer.size().into(), &env);
    assert_eq!(layout.resolved_size, Size::new(3, 1).into());
    let tree = view.render_tree(&layout, Point::zero(), &env);
    tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "AAA  ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "     ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "     ");

    let view = make_view(State::B("BBB"));
    let layout = view.layout(&buffer.size().into(), &env);
    assert_eq!(layout.resolved_size, Size::new(3, 1).into());
    let tree = view.render_tree(&layout, Point::zero(), &env);
    tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "BBB  ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "     ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "     ");

    buffer.clear();

    let view = make_view(State::C);
    let layout = view.layout(&buffer.size().into(), &env);
    assert_eq!(layout.resolved_size, Size::new(3, 1).into());
    let tree = view.render_tree(&layout, Point::zero(), &env);
    tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "CCC  ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "     ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "     ");
}

#[test]
fn test_match_view_borrow() {
    #[allow(unused)]
    #[derive(Clone)]
    enum State {
        A,
        B(String),
        C,
    }

    fn borrow_view<'a>(
        s: &'a State,
        font: &'a CharacterBufferFont,
    ) -> impl Renderable<char, Renderables: CharacterRender<char>> + use<'a> {
        match_view!(s => {
            State::A => Text::str("AAA", font),
            State::B(msg) => Text::str(msg, font),
            State::C => Text::str("CCC", font),
        })
    }

    let font = CharacterBufferFont;

    let mut buffer = FixedTextBuffer::<5, 5>::default();
    let env = DefaultEnvironment::default();

    let state = State::B("BBB".to_string());
    let view = borrow_view(&state, &font);
    let layout = view.layout(&buffer.size().into(), &env);
    assert_eq!(layout.resolved_size, Size::new(3, 1).into());
    let tree = view.render_tree(&layout, Point::zero(), &env);
    tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "BBB  ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "     ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "     ");
}

/// This could possibly be avoided by making an attempt at producing a new layout
/// when we go to render the view
#[should_panic(expected = "An outdated layout was used")]
#[test]
fn test_match_view_two_variants_invalid_layout() {
    let font = CharacterBufferFont;
    let make_view = |state| {
        match_view!(state => {
            0 => Text::str("zero\n!!!", &font),
            _ => Text::str("other", &font).foreground_color(' '),
        })
    };
    let mut buffer = FixedTextBuffer::<5, 5>::default();
    let env = DefaultEnvironment::default();

    let view = make_view(0);
    let layout = view.layout(&buffer.size().into(), &env);

    let view = make_view(1);
    let tree = view.render_tree(&layout, Point::zero(), &env);
    tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "other");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "     ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "     ");
}
