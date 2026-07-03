use buoyant::{
    app::Harness as _, font::CharacterBufferFont, render::Render as _,
    render_target::FixedTextBuffer, view::prelude::*,
};
mod common;
use common::{app, make_render_tree};

#[test]
fn background_inherits_foreground_size() {
    let font = CharacterBufferFont {};
    let view = Text::new("This is on\ntop", &font)
        .multiline_text_alignment(HorizontalTextAlignment::Center)
        .padding(Edges::All, 1)
        .background(Alignment::default(), Rectangle)
        .flex_frame()
        .with_infinite_max_width()
        .with_infinite_max_height()
        .foreground_color('-');

    let mut buffer = FixedTextBuffer::<14, 7>::default();

    let tree = make_render_tree(&view, buffer.size(), &mut ());

    tree.render(&mut buffer, &' ');
    assert_eq!(buffer.text[0].iter().collect::<String>(), "              ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), " ------------ ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), " -This is on- ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), " ----top----- ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), " ------------ ");
    assert_eq!(buffer.text[5].iter().collect::<String>(), "              ");
}

#[test]
fn background_alignment_coverage() {
    let view_fn = |alignment: Alignment| {
        EmptyView.frame_sized(3, 3).background(alignment, {
            Rectangle.foreground_color('X').frame_sized(1, 1)
        })
    };

    let mut buffer = FixedTextBuffer::<3, 3>::default();

    let view = view_fn(Alignment::TopLeading);
    let tree = make_render_tree(&view, buffer.size(), &mut ());
    buffer.clear();
    tree.render(&mut buffer, &' ');

    assert_eq!(buffer.text[0].iter().collect::<String>(), "X  ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "   ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "   ");

    let view = view_fn(Alignment::Top);
    let tree = make_render_tree(&view, buffer.size(), &mut ());
    buffer.clear();
    tree.render(&mut buffer, &' ');

    assert_eq!(buffer.text[0].iter().collect::<String>(), " X ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "   ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "   ");

    let view = view_fn(Alignment::TopTrailing);
    let tree = make_render_tree(&view, buffer.size(), &mut ());
    buffer.clear();
    tree.render(&mut buffer, &' ');

    assert_eq!(buffer.text[0].iter().collect::<String>(), "  X");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "   ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "   ");

    let view = view_fn(Alignment::Leading);
    let tree = make_render_tree(&view, buffer.size(), &mut ());
    buffer.clear();
    tree.render(&mut buffer, &' ');

    assert_eq!(buffer.text[0].iter().collect::<String>(), "   ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "X  ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "   ");

    let view = view_fn(Alignment::default());
    let tree = make_render_tree(&view, buffer.size(), &mut ());
    buffer.clear();
    tree.render(&mut buffer, &' ');

    assert_eq!(buffer.text[0].iter().collect::<String>(), "   ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), " X ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "   ");

    let view = view_fn(Alignment::Trailing);
    let tree = make_render_tree(&view, buffer.size(), &mut ());
    buffer.clear();
    tree.render(&mut buffer, &' ');

    assert_eq!(buffer.text[0].iter().collect::<String>(), "   ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "  X");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "   ");

    let view = view_fn(Alignment::BottomLeading);
    let tree = make_render_tree(&view, buffer.size(), &mut ());
    buffer.clear();
    tree.render(&mut buffer, &' ');

    assert_eq!(buffer.text[0].iter().collect::<String>(), "   ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "   ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "X  ");

    let view = view_fn(Alignment::Bottom);
    let tree = make_render_tree(&view, buffer.size(), &mut ());
    buffer.clear();
    tree.render(&mut buffer, &' ');

    assert_eq!(buffer.text[0].iter().collect::<String>(), "   ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "   ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), " X ");

    let view = view_fn(Alignment::BottomTrailing);
    let tree = make_render_tree(&view, buffer.size(), &mut ());
    buffer.clear();
    tree.render(&mut buffer, &' ');

    assert_eq!(buffer.text[0].iter().collect::<String>(), "   ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "   ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "  X");
}

#[derive(Clone, Default, PartialEq, Eq, Debug)]
struct TouchState {
    foreground_taps: u32,
    background_taps: u32,
}

fn background_touch_view(_: &TouchState) -> impl View<char, TouchState> + use<> {
    Button::new(
        |s: &mut TouchState| s.foreground_taps += 1,
        |_| Circle.frame_sized(4, 4),
    )
    .frame_sized(10, 10)
    .background(Alignment::default(), {
        Button::new(|s: &mut TouchState| s.background_taps += 1, |_| Rectangle)
    })
}

/// `background` consults the foreground (front) before the background (back). A touch in
/// the overlap fires only the foreground; a touch the foreground defers falls through to
/// the background.
#[test]
fn background_touch_consults_foreground_first() {
    let mut harness = app(TouchState::default(), background_touch_view);

    // The foreground button's content is a 4x4 circle centered in the 10x10 frame at
    // (3,3)-(7,7). (5,5) is inside both the circle and the background rectangle, so the
    // foreground (front) wins.
    harness.tap(buoyant::primitives::Point::new(5, 5));
    assert_eq!(harness.state().foreground_taps, 1);
    assert_eq!(
        harness.state().background_taps,
        0,
        "foreground should claim the overlap"
    );

    // (1,1) is outside the foreground's circle but inside the background rectangle, so the
    // foreground defers and the background handles it.
    harness.tap(buoyant::primitives::Point::new(1, 1));
    assert_eq!(
        harness.state().foreground_taps,
        1,
        "foreground should not fire for the corner tap"
    );
    assert_eq!(harness.state().background_taps, 1);
}
