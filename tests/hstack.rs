use std::iter::zip;

use buoyant::environment::DefaultEnvironment;
use buoyant::font::TerminalChar;
use buoyant::layout::{Layout, VerticalAlignment};
use buoyant::primitives::Size;
use buoyant::render::Render;
use buoyant::render_target::{FixedTextBuffer, RenderTarget};
use buoyant::view::{Divider, HStack, Spacer, Text, ViewExtensions};

#[test]
fn test_greedy_layout_2() {
    let hstack = HStack::two(Spacer::default(), Spacer::default());
    let offer = Size::new(100, 100);
    let env = DefaultEnvironment;
    let layout = hstack.layout(offer, &env);
    assert_eq!(layout.resolved_size, Size::new(100, 0));
}

#[test]
fn test_oversized_layout_2() {
    let vstack = HStack::two(Divider::default().padding(2), Spacer::default());
    let offer = Size::new(10, 0);
    let env = DefaultEnvironment;
    let layout = vstack.layout(offer, &env);
    assert_eq!(layout.resolved_size, Size::new(10, 0));
}

#[test]
fn test_oversized_layout_3() {
    let vstack = HStack::three(
        Divider::default(),
        Divider::default().padding(2),
        Spacer::default(),
    );
    let offer = Size::new(10, 0);
    let env = DefaultEnvironment;
    let layout = vstack.layout(offer, &env);
    assert_eq!(layout.resolved_size, Size::new(10, 0));
}

#[test]
fn test_undersized_layout_2() {
    let font = TerminalChar {};
    let hstack = HStack::two(Text::char("123", &font), Text::char("4567", &font)).spacing(1);
    let offer = Size::new(50, 1);
    let env = DefaultEnvironment;
    let layout = hstack.layout(offer, &env);
    assert_eq!(layout.resolved_size, Size::new(8, 1));
}

#[test]
fn test_horizontal_render_2() {
    let font = TerminalChar {};
    let hstack = HStack::two(Text::char("123", &font), Text::char("4567", &font)).spacing(1);
    let mut buffer = FixedTextBuffer::<9, 1>::default();
    let env = DefaultEnvironment;
    let layout = hstack.layout(buffer.size(), &env);
    hstack.render(&mut buffer, &layout, &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "123 4567 ");
}

#[test]
fn test_undersized_layout_3_left_pad() {
    let font = TerminalChar {};
    let hstack = HStack::three(
        Text::char("123", &font),
        Text::char("4567", &font),
        Spacer::default(),
    );
    let offer = Size::new(10, 1);
    let env = DefaultEnvironment;
    let layout = hstack.layout(offer, &env);
    assert_eq!(layout.resolved_size, Size::new(10, 1));
    let mut buffer = FixedTextBuffer::<10, 1>::default();
    hstack.render(&mut buffer, &layout, &env);

    assert_eq!(buffer.text[0].iter().collect::<String>(), "1234567   ");
}
#[test]
fn test_undersized_layout_3_right_pad_space() {
    let font = TerminalChar {};
    let hstack = HStack::three(
        Spacer::default(),
        Text::char("234", &font),
        Text::char("5678", &font),
    )
    .spacing(1);
    let offer = Size::new(10, 1);
    let env = DefaultEnvironment;
    let layout = hstack.layout(offer, &env);
    assert_eq!(layout.resolved_size, Size::new(10, 1));
    let mut buffer = FixedTextBuffer::<10, 1>::default();
    hstack.render(&mut buffer, &layout, &env);

    assert_eq!(buffer.text[0].iter().collect::<String>(), "  234 5678");
}

#[test]
fn test_oversized_layout_3_leading_pad_space() {
    // The second text view is too large to fit in the initial offer.
    let font = TerminalChar {};
    let hstack = HStack::three(
        Spacer::default(),
        Text::char("234", &font),
        Text::char("56789", &font),
    )
    .spacing(1);
    let offer = Size::new(10, 1);
    let env = DefaultEnvironment;
    let layout = hstack.layout(offer, &env);
    assert_eq!(layout.resolved_size, Size::new(10, 1));
    let mut buffer = FixedTextBuffer::<10, 1>::default();
    hstack.render(&mut buffer, &layout, &env);

    assert_eq!(buffer.text[0].iter().collect::<String>(), " 234 56789");
}

#[test]
fn test_undersized_layout_3_middle_pad() {
    let font = TerminalChar {};
    let hstack = HStack::three(
        Text::char("234", &font),
        Spacer::default(),
        Text::char("5678", &font),
    );
    let offer = Size::new(10, 1);
    let env = DefaultEnvironment;
    let layout = hstack.layout(offer, &env);
    assert_eq!(layout.resolved_size, Size::new(10, 1));
    let mut buffer = FixedTextBuffer::<10, 1>::default();
    hstack.render(&mut buffer, &layout, &env);

    assert_eq!(buffer.text[0].iter().collect::<String>(), "234   5678");
}

#[test]
fn test_oversized_layout_3_middle_pad_space() {
    // The third text view is too large to fit in the initial offer.
    let font = TerminalChar {};
    let hstack = HStack::three(
        Text::char("234", &font),
        Spacer::default(),
        Text::char("56789", &font),
    )
    .spacing(1);
    let offer = Size::new(10, 1);
    let env = DefaultEnvironment;
    let layout = hstack.layout(offer, &env);
    assert_eq!(layout.resolved_size, Size::new(10, 1));
    let mut buffer = FixedTextBuffer::<10, 1>::default();
    hstack.render(&mut buffer, &layout, &env);

    assert_eq!(buffer.text[0].iter().collect::<String>(), "234  56789");
}

#[test]
fn test_oversized_layout_3_trailing_pad_space() {
    // The second text view is too large to fit in the initial offer.
    let font = TerminalChar {};
    let hstack = HStack::three(
        Text::char("234", &font),
        Text::char("56789", &font),
        Spacer::default(),
    )
    .spacing(1);
    let offer = Size::new(10, 1);
    let env = DefaultEnvironment;
    let layout = hstack.layout(offer, &env);
    assert_eq!(layout.resolved_size, Size::new(10, 1));
    let mut buffer = FixedTextBuffer::<10, 1>::default();
    hstack.render(&mut buffer, &layout, &env);

    assert_eq!(buffer.text[0].iter().collect::<String>(), "234 56789 ");
}

#[test]
fn test_layout_3_remainder_allocation() {
    // The HStack should attempt to lay out the views into the full width of the offer.
    let font = TerminalChar {};
    let hstack = HStack::three(
        Text::char("aaa", &font),
        Text::char("bbb", &font),
        Text::char("ccc", &font),
    );
    let env = DefaultEnvironment;
    let mut buffer = FixedTextBuffer::<10, 1>::default();
    let offer = Size::new(7, 1);
    let layout = hstack.layout(offer, &env);
    hstack.render(&mut buffer, &layout, &env);

    assert_eq!(buffer.text[0].iter().collect::<String>(), "aaabbcc   ");

    let offer = Size::new(8, 1);
    let layout = hstack.layout(offer, &env);
    hstack.render(&mut buffer, &layout, &env);

    assert_eq!(buffer.text[0].iter().collect::<String>(), "aaabbbcc  ");

    let offer = Size::new(9, 1);
    let layout = hstack.layout(offer, &env);
    hstack.render(&mut buffer, &layout, &env);

    assert_eq!(buffer.text[0].iter().collect::<String>(), "aaabbbccc ");

    let offer = Size::new(10, 1);
    let layout = hstack.layout(offer, &env);
    hstack.render(&mut buffer, &layout, &env);

    assert_eq!(buffer.text[0].iter().collect::<String>(), "aaabbbccc ");
}

#[test]
fn test_layout_3_vertical_alignment_bottom() {
    // The HStack should attempt to lay out the views into the full width of the offer.
    let font = TerminalChar {};
    let hstack = HStack::three(
        Text::char("aaa", &font),
        Divider::default(),
        Text::char("ccc", &font),
    )
    .alignment(VerticalAlignment::Bottom)
    .spacing(1);
    let env = DefaultEnvironment;
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let layout = hstack.layout(buffer.size(), &env);
    hstack.render(&mut buffer, &layout, &env);

    assert_eq!(buffer.text[0].iter().collect::<String>(), "   |  ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "   |  ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "   | c");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "aa | c");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "a  | c");
}

#[test]
fn test_layout_3_vertical_alignment_center() {
    // The HStack should attempt to lay out the views into the full width of the offer.
    let font = TerminalChar {};
    let hstack = HStack::three(
        Text::char("aaa", &font),
        Divider::default(),
        Text::char("ccc", &font),
    )
    .alignment(VerticalAlignment::Center)
    .spacing(1);
    let env = DefaultEnvironment;
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let layout = hstack.layout(buffer.size(), &env);
    hstack.render(&mut buffer, &layout, &env);

    assert_eq!(buffer.text[0].iter().collect::<String>(), "   |  ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "aa | c");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "a  | c");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "   | c");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "   |  ");
}

#[test]
fn test_layout_3_vertical_alignment_top() {
    // The HStack should attempt to lay out the views into the full width of the offer.
    let font = TerminalChar {};
    let hstack = HStack::three(
        Text::char("aaa", &font),
        Divider::default(),
        Text::char("ccc", &font),
    )
    .alignment(VerticalAlignment::Top)
    .spacing(1);
    let env = DefaultEnvironment;
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let layout = hstack.layout(buffer.size(), &env);
    hstack.render(&mut buffer, &layout, &env);

    assert_eq!(buffer.text[0].iter().collect::<String>(), "aa | c");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "a  | c");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "   | c");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "   |  ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "   |  ");
}

#[test]
fn test_minimal_offer_extra_space_1() {
    // The HStack should offer remaining space when the views do not consume the full width.
    let font = TerminalChar {};
    let hstack = HStack::three(
        Text::char("a b c d e f", &font),
        Text::char("g h i", &font),
        Text::char("j", &font),
    )
    .alignment(VerticalAlignment::Top)
    .spacing(1);

    let env = DefaultEnvironment;
    let mut buffer = FixedTextBuffer::<19, 5>::default();

    let layout = hstack.layout(buffer.size(), &env);

    hstack.render(&mut buffer, &layout, &env);

    let lines = [
        "a b c d e f g h i j",
        "                   ",
        "                   ",
        "                   ",
        "                   ",
    ];
    zip(lines.iter(), buffer.text.iter()).for_each(|(expected, actual)| {
        assert_eq!(actual.iter().collect::<String>(), *expected);
    });
}
