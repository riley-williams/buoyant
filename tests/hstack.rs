use buoyant::font::TextBufferFont;
use buoyant::layout::{Environment, Layout, VerticalAlignment};
use buoyant::primitives::Size;
use buoyant::render::{FixedTextBuffer, Render, RenderTarget};
use buoyant::view::{Divider, HStack, Spacer, Text};

struct TestEnv {}
impl Environment for TestEnv {}

#[test]
fn test_greedy_layout_2() {
    let hstack = HStack::two(Spacer::default(), Spacer::default());
    let offer = Size::new(100, 100);
    let env = TestEnv {};
    let layout = hstack.layout(offer, &env);
    assert_eq!(layout.resolved_size, Size::new(100, 0));
}
#[test]
fn test_undersized_layout_2() {
    let hstack = HStack::two(
        Text::new("123", TextBufferFont {}),
        Text::new("4567", TextBufferFont {}),
    )
    .spacing(1);
    let offer = Size::new(50, 1);
    let env = TestEnv {};
    let layout = hstack.layout(offer, &env);
    assert_eq!(layout.resolved_size, Size::new(8, 1));
}

#[test]
fn test_horizontal_render_2() {
    let hstack = HStack::two(
        Text::new("123", TextBufferFont {}),
        Text::new("4567", TextBufferFont {}),
    )
    .spacing(1);
    let mut buffer = FixedTextBuffer::<9, 1>::default();
    let env = TestEnv {};
    let layout = hstack.layout(buffer.size(), &env);
    layout.render(&mut buffer, &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "123 4567 ");
}

#[test]
fn test_undersized_layout_3_left_pad() {
    let hstack = HStack::three(
        Text::new("123", TextBufferFont {}),
        Text::new("4567", TextBufferFont {}),
        Spacer::default(),
    );
    let offer = Size::new(10, 1);
    let env = TestEnv {};
    let layout = hstack.layout(offer, &env);
    assert_eq!(layout.resolved_size, Size::new(10, 1));
    let mut buffer = FixedTextBuffer::<10, 1>::default();
    layout.render(&mut buffer, &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "1234567   ");
}
#[test]
fn test_undersized_layout_3_right_pad_space() {
    let hstack = HStack::three(
        Spacer::default(),
        Text::new("234", TextBufferFont {}),
        Text::new("5678", TextBufferFont {}),
    )
    .spacing(1);
    let offer = Size::new(10, 1);
    let env = TestEnv {};
    let layout = hstack.layout(offer, &env);
    assert_eq!(layout.resolved_size, Size::new(10, 1));
    let mut buffer = FixedTextBuffer::<10, 1>::default();
    layout.render(&mut buffer, &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "  234 5678");
}
#[test]
fn test_oversized_layout_3_right_pad_space_overflows() {
    // The second text view is too large to fit in the offer.
    // Despite the view having enough space for all the text, we are avoiding extra layout
    // calls and only offer the layout group width / N to the views.
    // This constrains the number of layout passes to at most 2 per group.
    let hstack = HStack::three(
        Spacer::default(),
        Text::new("234", TextBufferFont {}),
        Text::new("56789", TextBufferFont {}),
    )
    .spacing(1);
    let offer = Size::new(10, 1);
    let env = TestEnv {};
    let layout = hstack.layout(offer, &env);
    assert_eq!(layout.resolved_size, Size::new(10, 1));
    let mut buffer = FixedTextBuffer::<10, 1>::default();
    layout.render(&mut buffer, &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "  234 5678");
}
#[test]
fn test_undersized_layout_3_middle_pad() {
    let hstack = HStack::three(
        Text::new("234", TextBufferFont {}),
        Spacer::default(),
        Text::new("5678", TextBufferFont {}),
    );
    let offer = Size::new(10, 1);
    let env = TestEnv {};
    let layout = hstack.layout(offer, &env);
    assert_eq!(layout.resolved_size, Size::new(10, 1));
    let mut buffer = FixedTextBuffer::<10, 1>::default();
    layout.render(&mut buffer, &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "234   5678");
}
#[test]
fn test_layout_3_remainder_allocation() {
    // The HStack should attempt to lay out the views into the full width of the offer.
    let hstack = HStack::three(
        Text::new("aaa", TextBufferFont {}),
        Text::new("bbb", TextBufferFont {}),
        Text::new("ccc", TextBufferFont {}),
    );
    let env = TestEnv {};
    let mut buffer = FixedTextBuffer::<10, 1>::default();
    let offer = Size::new(7, 1);
    let layout = hstack.layout(offer, &env);
    layout.render(&mut buffer, &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "aaabbcc   ");

    let offer = Size::new(8, 1);
    let layout = hstack.layout(offer, &env);
    layout.render(&mut buffer, &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "aaabbbcc  ");

    let offer = Size::new(9, 1);
    let layout = hstack.layout(offer, &env);
    layout.render(&mut buffer, &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "aaabbbccc ");

    let offer = Size::new(10, 1);
    let layout = hstack.layout(offer, &env);
    layout.render(&mut buffer, &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "aaabbbccc ");
}

#[test]
fn test_layout_3_vertical_alignment_bottom() {
    // The HStack should attempt to lay out the views into the full width of the offer.
    let hstack = HStack::three(
        Text::new("aaa", TextBufferFont {}),
        Divider::default(),
        Text::new("ccc", TextBufferFont {}),
    )
    .alignment(VerticalAlignment::Bottom)
    .spacing(1);
    let env = TestEnv {};
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let layout = hstack.layout(buffer.size(), &env);
    layout.render(&mut buffer, &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "   |  ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "   |  ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "   | c");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "aa | c");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "a  | c");
}

#[test]
fn test_layout_3_vertical_alignment_center() {
    // The HStack should attempt to lay out the views into the full width of the offer.
    let hstack = HStack::three(
        Text::new("aaa", TextBufferFont {}),
        Divider::default(),
        Text::new("ccc", TextBufferFont {}),
    )
    .alignment(VerticalAlignment::Center)
    .spacing(1);
    let env = TestEnv {};
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let layout = hstack.layout(buffer.size(), &env);
    layout.render(&mut buffer, &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "   |  ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "aa | c");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "a  | c");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "   | c");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "   |  ");
}

#[test]
fn test_layout_3_vertical_alignment_top() {
    // The HStack should attempt to lay out the views into the full width of the offer.
    let hstack = HStack::three(
        Text::new("aaa", TextBufferFont {}),
        Divider::default(),
        Text::new("ccc", TextBufferFont {}),
    )
    .alignment(VerticalAlignment::Top)
    .spacing(1);
    let env = TestEnv {};
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let layout = hstack.layout(buffer.size(), &env);
    layout.render(&mut buffer, &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "aa | c");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "a  | c");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "   | c");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "   |  ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "   |  ");
}
