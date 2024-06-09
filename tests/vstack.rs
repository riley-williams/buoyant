use buoyant::font::CharMonospace;
use buoyant::layout::{Environment, HorizontalAlignment, Layout, VerticalAlignment};
use buoyant::primitives::Size;
use buoyant::render::Render;
use buoyant::render_target::{FixedTextBuffer, RenderTarget as _};
use buoyant::view::{Divider, HStack, HorizontalTextAlignment, Padding, Spacer, Text, VStack};

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
    let font = CharMonospace {};
    let vstack = VStack::three(
        Text::char("123", &font),
        Text::char("4567", &font),
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
    let font = CharMonospace {};
    let vstack = VStack::three(
        Spacer::default(),
        Text::char("234", &font),
        Text::char("5678", &font),
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
fn test_oversized_layout_3_right_pad_space() {
    // The second text view is too large to fit in the initial offer.
    let font = CharMonospace {};
    let vstack = VStack::three(
        Spacer::default(),
        Text::char("234", &font),
        Text::char("56789", &font),
    )
    .spacing(1);
    let offer = Size::new(1, 10);
    let env = TestEnv {};
    let layout = vstack.layout(offer, &env);
    assert_eq!(layout.resolved_size, Size::new(1, 10));
    let mut buffer = FixedTextBuffer::<1, 10>::default();
    vstack.render(&mut buffer, &layout, &env);
    assert_eq!(collect_text(&buffer), " 234 56789");
}

#[test]
fn test_oversized_layout_3_middle_pad_space() {
    // The second text view is too large to fit in the initial offer.
    let font = CharMonospace {};
    let vstack = VStack::three(
        Text::char("234", &font),
        Spacer::default(),
        Text::char("56789", &font),
    )
    .spacing(1);
    let offer = Size::new(1, 10);
    let env = TestEnv {};
    let layout = vstack.layout(offer, &env);
    assert_eq!(layout.resolved_size, Size::new(1, 10));
    let mut buffer = FixedTextBuffer::<1, 10>::default();
    vstack.render(&mut buffer, &layout, &env);
    assert_eq!(collect_text(&buffer), "234  56789");
}

#[test]
fn test_oversized_layout_3_trailing_pad_space() {
    // The second text view is too large to fit in the initial offer.
    let font = CharMonospace {};
    let vstack = VStack::three(
        Text::char("234", &font),
        Text::char("56789", &font),
        Spacer::default(),
    )
    .spacing(1);
    let offer = Size::new(1, 10);
    let env = TestEnv {};
    let layout = vstack.layout(offer, &env);
    assert_eq!(layout.resolved_size, Size::new(1, 10));
    let mut buffer = FixedTextBuffer::<1, 10>::default();
    vstack.render(&mut buffer, &layout, &env);
    assert_eq!(collect_text(&buffer), "234 56789 ");
}

#[test]
fn test_undersized_layout_3_middle_pad() {
    let font = CharMonospace {};
    let vstack = VStack::three(
        Text::char("234", &font),
        Spacer::default(),
        Text::char("5678", &font),
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
    let font = CharMonospace {};
    let vstack = VStack::three(
        Text::char("aaa", &font),
        Text::char("bbb", &font),
        Text::char("ccc", &font),
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
    let font = CharMonospace {};
    let vstack = VStack::three(
        Text::char("aaa", &font),
        Divider::default(),
        Text::char("ccccccc", &font),
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
    let font = CharMonospace {};
    let vstack = VStack::three(
        Text::char("aaa", &font),
        Divider::default(),
        Text::char("cccc", &font),
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
    let font = CharMonospace {};
    let vstack = VStack::three(
        Text::char("aaa", &font),
        Divider::default(),
        Text::char("ccc", &font).multiline_text_alignment(HorizontalTextAlignment::Trailing),
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

#[test]
fn test_offer_remaining_space_for_undersized_views_expansion() {
    let font = CharMonospace {};
    let stack = VStack::three(
    HStack::three(
        Text::char(
            "This text is centered horizontally in the middle of its space\nThe stack however, has bottom alignment.",
            &font,
                )
                .multiline_text_alignment(HorizontalTextAlignment::Center),
        Spacer::default(),
        Text::char(
            "This text is aligned to the right, with trailing multi-line text alignment",
            &font,
                )
                .multiline_text_alignment(HorizontalTextAlignment::Trailing),
            )
            .spacing(1)
            .alignment(VerticalAlignment::Bottom),
    Divider::default(),
    VStack::three(
        Spacer::default(),
        Padding::new(2,
            Text::char(
                "This is several lines of text.\nEach line is centered in the available space.\n Spacers are used to fill all the remaining verical space and align the content within it.\n2 points of padding are around this text",
                &font,
                    )
                    .multiline_text_alignment(HorizontalTextAlignment::Center),
                ),
        Divider::default(),
        ),
    );

    let env = TestEnv {};
    // The spacers in this view should always cause the stack size to equal the offer size
    // 10k layouts...but should only take a few ms
    for width in 1..100 {
        for height in 1..100 {
            let size = Size::new(width, height);
            let layout = stack.layout(size, &env);
            assert_eq!(size, layout.resolved_size);
        }
    }
}
