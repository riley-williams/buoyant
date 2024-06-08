use buoyant::font::TextBufferFont;
use buoyant::layout::{Environment, HorizontalAlignment, Layout};
use buoyant::primitives::Size;
use buoyant::render::Render;
use buoyant::render_target::{FixedTextBuffer, RenderTarget as _};
use buoyant::view::{Divider, HStack, HorizontalTextAlignment, Spacer, Text, VStack};

struct TestEnv {}
impl Environment for TestEnv {}

fn collect_text<const W: usize, const H: usize>(buffer: &FixedTextBuffer<W, H>) -> String {
    buffer
        .text
        .iter()
        .map(|chars| chars.iter().collect::<String>())
        .collect::<String>()
}

#[test]
fn test_greedy_layout_2() {
    let vstack = VStack::two(Spacer::default(), Spacer::default());
    let offer = Size::new(100, 100);
    let env = TestEnv {};
    let layout = vstack.layout(offer, &env);
    assert_eq!(layout.resolved_size, Size::new(0, 100));
}

#[test]
fn test_undersized_layout_3_bottom_pad() {
    let vstack = VStack::three(
        Text::new("123", TextBufferFont {}),
        Text::new("4567", TextBufferFont {}),
        Spacer::default(),
    );
    let offer = Size::new(1, 10);
    let env = TestEnv {};
    let layout = vstack.layout(offer, &env);
    assert_eq!(layout.resolved_size, Size::new(1, 10));
    let mut buffer = FixedTextBuffer::<1, 10>::default();
    vstack.render(&mut buffer, &layout, &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "1");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "2");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "3");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "4");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "5");
    assert_eq!(buffer.text[5].iter().collect::<String>(), "6");
    assert_eq!(buffer.text[6].iter().collect::<String>(), "7");
    assert_eq!(buffer.text[7].iter().collect::<String>(), " ");
    assert_eq!(buffer.text[8].iter().collect::<String>(), " ");
    assert_eq!(buffer.text[9].iter().collect::<String>(), " ");
}

#[test]
fn test_undersized_layout_3_right_pad_space() {
    let vstack = VStack::three(
        Spacer::default(),
        Text::new("234", TextBufferFont {}),
        Text::new("5678", TextBufferFont {}),
    )
    .spacing(1);
    let offer = Size::new(1, 10);
    let env = TestEnv {};
    let layout = vstack.layout(offer, &env);
    assert_eq!(layout.resolved_size, Size::new(1, 10));
    let mut buffer = FixedTextBuffer::<1, 10>::default();
    vstack.render(&mut buffer, &layout, &env);
    assert_eq!(collect_text(&buffer), "  234 5678");
}

#[test]
fn test_oversized_layout_3_right_pad_space_overflows() {
    // The second text view is too large to fit in the offer.
    // Despite the view having enough space for all the text, we are avoiding extra layout
    // calls and only offer the layout group width / N to the views.
    // This constrains the number of layout passes to at most 2 per group.
    let vstack = VStack::three(
        Spacer::default(),
        Text::new("234", TextBufferFont {}),
        Text::new("56789", TextBufferFont {}),
    )
    .spacing(1);
    let offer = Size::new(1, 10);
    let env = TestEnv {};
    let layout = vstack.layout(offer, &env);
    assert_eq!(layout.resolved_size, Size::new(1, 10));
    let mut buffer = FixedTextBuffer::<1, 10>::default();
    vstack.render(&mut buffer, &layout, &env);
    assert_eq!(collect_text(&buffer), "  234 5678");
}

#[test]
fn test_undersized_layout_3_middle_pad() {
    let vstack = VStack::three(
        Text::new("234", TextBufferFont {}),
        Spacer::default(),
        Text::new("5678", TextBufferFont {}),
    );
    let offer = Size::new(1, 10);
    let env = TestEnv {};
    let layout = vstack.layout(offer, &env);
    assert_eq!(layout.resolved_size, Size::new(1, 10));
    let mut buffer = FixedTextBuffer::<1, 10>::default();
    vstack.render(&mut buffer, &layout, &env);
    assert_eq!(collect_text(&buffer), "234   5678");
}

#[test]
fn test_layout_3_remainder_allocation() {
    // The VStack should attempt to lay out the views into the full width of the offer.
    let vstack = VStack::three(
        Text::new("aaa", TextBufferFont {}),
        Text::new("bbb", TextBufferFont {}),
        Text::new("ccc", TextBufferFont {}),
    );
    let env = TestEnv {};
    let mut buffer = FixedTextBuffer::<1, 10>::default();
    let offer = Size::new(1, 7);
    let layout = vstack.layout(offer, &env);
    vstack.render(&mut buffer, &layout, &env);
    assert_eq!(collect_text(&buffer), "aaabbcc   ");

    let offer = Size::new(1, 8);
    let layout = vstack.layout(offer, &env);
    vstack.render(&mut buffer, &layout, &env);
    assert_eq!(collect_text(&buffer), "aaabbbcc  ");

    let offer = Size::new(1, 9);
    let layout = vstack.layout(offer, &env);
    vstack.render(&mut buffer, &layout, &env);
    assert_eq!(collect_text(&buffer), "aaabbbccc ");

    let offer = Size::new(1, 10);
    let layout = vstack.layout(offer, &env);
    vstack.render(&mut buffer, &layout, &env);
    assert_eq!(collect_text(&buffer), "aaabbbccc ");
}

#[test]
fn test_layout_3_horizontal_alignment_trailing() {
    // The VStack should attempt to lay out the views into the full width of the offer.
    let vstack = VStack::three(
        Text::new("aaa", TextBufferFont {}),
        Divider::default(),
        Text::new("ccccccc", TextBufferFont {}),
    )
    .alignment(HorizontalAlignment::Trailing)
    .spacing(1);
    let env = TestEnv {};
    let mut buffer = FixedTextBuffer::<6, 7>::default();
    let layout = vstack.layout(buffer.size(), &env);
    vstack.render(&mut buffer, &layout, &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "   aaa");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "------");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "cccccc");
    assert_eq!(buffer.text[5].iter().collect::<String>(), "c     ");
    // multiline text alignment applies within the frame of the text
    // the leading c is correct
}

#[test]
fn test_layout_3_alignment_center() {
    // The VStack should attempt to lay out the views into the full width of the offer.
    let vstack = VStack::three(
        Text::new("aaa", TextBufferFont {}),
        Divider::default(),
        Text::new("cccc", TextBufferFont {}),
    )
    .alignment(HorizontalAlignment::Center);
    let env = TestEnv {};
    let mut buffer = FixedTextBuffer::<7, 5>::default();
    let layout = vstack.layout(buffer.size(), &env);
    vstack.render(&mut buffer, &layout, &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "  aaa  ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "-------");
    assert_eq!(buffer.text[2].iter().collect::<String>(), " cccc  ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "       ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "       ");
}

#[test]
fn test_layout_3_alignment_leading() {
    // The VStack should attempt to lay out the views into the full width of the offer.
    let vstack = VStack::three(
        Text::new("aaa", TextBufferFont {}),
        Divider::default(),
        Text::new("ccc", TextBufferFont {})
            .multiline_text_alignment(HorizontalTextAlignment::Trailing),
    )
    .alignment(HorizontalAlignment::Leading)
    .spacing(1);
    let env = TestEnv {};
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let layout = vstack.layout(buffer.size(), &env);
    vstack.render(&mut buffer, &layout, &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "aaa   ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "------");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "ccc   ");
    // Despite the text being right aligned, the text is left aligned in the buffer.
    // Multiline text alignment only applies within the frame of the text
}

#[test]
fn test_layout_direction_is_set_inner_hstack() {
    let vstack = VStack::three(
        Divider::default(),
        HStack::two(Divider::default(), Spacer::default()),
        Divider::default(),
    );
    let env = TestEnv {};
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let layout = vstack.layout(buffer.size(), &env);
    vstack.render(&mut buffer, &layout, &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "------");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "|     ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "|     ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "|     ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "------");
}

#[test]
fn test_layout_direction_is_set_inner_vstack() {
    let hstack = HStack::three(
        Divider::default(),
        VStack::two(Divider::default(), Spacer::default()),
        Divider::default(),
    );
    let env = TestEnv {};
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let layout = hstack.layout(buffer.size(), &env);
    hstack.render(&mut buffer, &layout, &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "|----|");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "|    |");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "|    |");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "|    |");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "|    |");
}
