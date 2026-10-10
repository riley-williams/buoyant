use buoyant::{
    app::Harness as _, font::CharacterBufferFont, primitives::Point, render::Render as _,
    render_target::FixedTextBuffer, view::prelude::*,
};
mod common;
use common::{app, make_render_tree};

#[test]
fn overlay_inherits_foreground_size() {
    let font = CharacterBufferFont {};
    let view = Text::new("This is\n!visible", &font)
        .multiline_text_alignment(HorizontalTextAlignment::Center)
        .padding(Edges::All, 1)
        .overlay(Alignment::default(), Rectangle.foreground_color('-'))
        .flex_frame()
        .with_infinite_max_width()
        .with_infinite_max_height();

    let mut buffer = FixedTextBuffer::<14, 7>::default();

    let tree = make_render_tree(&view, buffer.size(), &mut ());

    tree.render(&mut buffer, &' ');
    assert_eq!(buffer.text[0].iter().collect::<String>(), "              ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "  ----------  ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "  ----------  ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "  ----------  ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "  ----------  ");
    assert_eq!(buffer.text[5].iter().collect::<String>(), "              ");
    assert_eq!(buffer.text[6].iter().collect::<String>(), "              ");
}

#[test]
fn overlay_renders_on_top() {
    let font = CharacterBufferFont {};
    // Create a base view with text
    let view = Text::new("BASE", &font)
        .padding(Edges::All, 1)
        .foreground_color('B')
        .overlay(Alignment::default(), Rectangle.foreground_color('O'));

    let mut buffer = FixedTextBuffer::<6, 3>::default();

    let tree = make_render_tree(&view, buffer.size(), &mut ());

    tree.render(&mut buffer, &' ');

    // The overlay character 'O' should appear on top of the base content 'B'
    assert_eq!(buffer.text[0].iter().collect::<String>(), "OOOOOO");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "OOOOOO");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "OOOOOO");
}

#[test]
fn overlay_alignment_variations() {
    let view_fn = |alignment: Alignment| {
        EmptyView.frame_sized(3, 3).overlay(alignment, {
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

#[test]
fn multiple_overlay_layering() {
    let view = EmptyView
        .frame_sized(3, 3)
        .overlay(Alignment::TopLeading, {
            Rectangle.foreground_color('1').frame_sized(2, 2)
        })
        .overlay(Alignment::BottomTrailing, {
            Rectangle.foreground_color('2').frame_sized(2, 2)
        });

    let mut buffer = FixedTextBuffer::<3, 3>::default();

    let tree = make_render_tree(&view, buffer.size(), &mut ());
    tree.render(&mut buffer, &' ');

    // The second overlay should be on top where they overlap
    assert_eq!(buffer.text[0].iter().collect::<String>(), "11 ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "122");
    assert_eq!(buffer.text[2].iter().collect::<String>(), " 22");
}

#[test]
fn overlay_offset_order() {
    // Overlay is only relative. The second item in the HStack is drawn last and should
    // be on top of the overlaied view
    let view = HStack::new((
        Rectangle
            .foreground_color('1')
            .overlay(Alignment::Trailing, {
                Rectangle
                    .foreground_color('2')
                    .frame_sized(2, 1)
                    .offset(1, 0)
            }),
        Rectangle.foreground_color('3'),
    ));

    let mut buffer = FixedTextBuffer::<4, 3>::default();

    let tree = make_render_tree(&view, buffer.size(), &mut ());
    tree.render(&mut buffer, &' ');

    // The second overlay should be on top where they overlap
    assert_eq!(buffer.text[0].iter().collect::<String>(), "1133");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "1233");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "1133");
}

#[derive(Clone, Default, PartialEq, Eq, Debug)]
struct TouchState {
    foreground_taps: u32,
    overlay_taps: u32,
}

fn overlay_touch_view(_: &TouchState) -> impl View<char, TouchState> + use<> {
    Button::new(|s: &mut TouchState| s.foreground_taps += 1, |_| Rectangle).overlay(
        Alignment::default(),
        Button::new(|s: &mut TouchState| s.overlay_taps += 1, |_| Rectangle).frame_sized(4, 4),
    )
}

/// `overlay` consults the overlay (front) before the foreground (back). A touch in the
/// overlap fires only the overlay; a touch the overlay defers falls through to the
/// foreground.
#[test]
fn overlay_touch_consults_overlay_first() {
    let mut harness = app(TouchState::default(), overlay_touch_view);

    // The overlay button is a 4x4 frame centered in the 10x10 foreground at (3,3)-(7,7).
    // (5,5) is inside both, so the overlay (front) wins.
    harness.tap(Point::new(5, 5));
    assert_eq!(harness.state().overlay_taps, 1);
    assert_eq!(
        harness.state().foreground_taps,
        0,
        "overlay should claim the overlap"
    );

    // (1,1) is outside the overlay but inside the foreground rectangle, so the overlay
    // defers and the foreground handles it.
    harness.tap(Point::new(1, 1));
    assert_eq!(
        harness.state().overlay_taps,
        1,
        "overlay should not fire for the corner tap"
    );
    assert_eq!(harness.state().foreground_taps, 1);
}
