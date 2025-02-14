use buoyant::environment::DefaultEnvironment;
use buoyant::font::CharacterBufferFont;
use buoyant::layout::{HorizontalAlignment, Layout, VerticalAlignment};
use buoyant::primitives::{Dimensions, Point, ProposedDimension, ProposedDimensions, Size};
use buoyant::render::CharacterRender;
use buoyant::render::CharacterRenderTarget;
use buoyant::render_target::FixedTextBuffer;
use buoyant::view::{make_render_tree, RenderExtensions as _};
use buoyant::view::{
    shape::Rectangle, Divider, EmptyView, HStack, HorizontalTextAlignment, LayoutExtensions,
    Spacer, Text, VStack,
};

mod common;
use common::collect_text;

#[test]
fn test_greedy_layout_2() {
    let vstack = VStack::new((Spacer::default(), Spacer::default()));
    let offer = Size::new(100, 100);
    let env = DefaultEnvironment::non_animated();
    let layout = vstack.layout(&offer.into(), &env);
    assert_eq!(layout.resolved_size, Dimensions::new(0, 100));
}

/// The Stack should never exceed the offer size.
#[test]
fn test_oversized_layout_2() {
    let vstack = VStack::new((Divider::default().padding(2), Spacer::default()));
    let offer = Size::new(0, 10);
    let env = DefaultEnvironment::non_animated();
    let layout = vstack.layout(&offer.into(), &env);
    assert_eq!(layout.resolved_size, Dimensions::new(0, 10));
}

#[test]
fn test_oversized_layout_3() {
    let vstack = VStack::new((
        Divider::default(),
        Divider::default().padding(2),
        Spacer::default(),
    ));
    let offer = Size::new(0, 10);
    let env = DefaultEnvironment::non_animated();
    let layout = vstack.layout(&offer.into(), &env);
    assert_eq!(layout.resolved_size, Dimensions::new(0, 10));
}

#[test]
fn infinite_height_offer_results_in_sum_of_subview_heights() {
    let vstack = VStack::new((
        Rectangle.frame(Some(3), Some(8), None, None),
        Rectangle.frame(Some(1), Some(40), None, None),
        Rectangle.frame(Some(8), Some(200), None, None),
    ))
    .with_spacing(1);
    let offer = ProposedDimensions {
        width: ProposedDimension::Exact(10),
        height: ProposedDimension::Infinite,
    };
    let env = DefaultEnvironment::non_animated();
    let layout = vstack.layout(&offer, &env);
    assert_eq!(layout.resolved_size, Dimensions::new(8, 248 + 2));
}

#[test]
fn compact_height_offer_results_in_sum_of_subview_heights() {
    let vstack = VStack::new((
        Rectangle.frame(Some(3), Some(8), None, None),
        Rectangle.frame(Some(1), Some(40), None, None),
        Rectangle.frame(Some(8), Some(200), None, None),
    ))
    .with_spacing(1);
    let offer = ProposedDimensions {
        width: ProposedDimension::Exact(10),
        height: ProposedDimension::Compact,
    };
    let env = DefaultEnvironment::non_animated();
    let layout = vstack.layout(&offer, &env);
    assert_eq!(layout.resolved_size, Dimensions::new(8, 248 + 2));
}

#[test]
fn infinite_height_offer_results_in_sum_of_subview_heights_minus_empties() {
    let vstack = VStack::new((
        Rectangle.frame(Some(3), Some(8), None, None),
        EmptyView,
        Rectangle.frame(Some(8), Some(200), None, None),
    ))
    .with_spacing(1);
    let offer = ProposedDimensions {
        width: ProposedDimension::Exact(10),
        height: ProposedDimension::Infinite,
    };
    let env = DefaultEnvironment::non_animated();
    let layout = vstack.layout(&offer, &env);
    assert_eq!(layout.resolved_size, Dimensions::new(8, 208 + 1));
}

#[test]
fn compact_height_offer_results_in_sum_of_subview_heights_minus_empties() {
    let vstack = VStack::new((
        Rectangle.frame(Some(3), Some(8), None, None),
        EmptyView,
        Rectangle.frame(Some(8), Some(200), None, None),
    ))
    .with_spacing(1);
    let offer = ProposedDimensions {
        width: ProposedDimension::Exact(10),
        height: ProposedDimension::Compact,
    };
    let env = DefaultEnvironment::non_animated();
    let layout = vstack.layout(&offer, &env);
    assert_eq!(layout.resolved_size, Dimensions::new(8, 208 + 1));
}

#[test]
fn test_undersized_layout_3_bottom_pad() {
    let font = CharacterBufferFont {};
    let vstack = VStack::new((
        Text::str("123", &font),
        Text::str("4567", &font),
        Spacer::default(),
    ))
    .foreground_color(' ');
    let offer = Size::new(1, 10);
    let env = DefaultEnvironment::non_animated();
    let layout = vstack.layout(&offer.into(), &env);
    assert_eq!(layout.resolved_size, Dimensions::new(1, 10));
    let mut buffer = FixedTextBuffer::<1, 10>::default();
    let tree = make_render_tree(&vstack, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());
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
    let font = CharacterBufferFont {};
    let vstack = VStack::new((
        Spacer::default(),
        Text::str("234", &font),
        Text::str("5678", &font),
    ))
    .with_spacing(1)
    .foreground_color(' ');
    let offer = Size::new(1, 10);
    let env = DefaultEnvironment::non_animated();
    let layout = vstack.layout(&offer.into(), &env);
    assert_eq!(layout.resolved_size, Dimensions::new(1, 10));
    let mut buffer = FixedTextBuffer::<1, 10>::default();
    let tree = make_render_tree(&vstack, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(collect_text(&buffer), "  234 5678");
}

#[test]
fn test_oversized_layout_3_right_pad_space() {
    // The second text view is too large to fit in the initial offer.
    let font = CharacterBufferFont {};
    let vstack = VStack::new((
        Spacer::default(),
        Text::str("234", &font),
        Text::str("56789", &font),
    ))
    .with_spacing(1)
    .foreground_color(' ');
    let offer = Size::new(1, 10);
    let env = DefaultEnvironment::non_animated();
    let layout = vstack.layout(&offer.into(), &env);
    assert_eq!(layout.resolved_size, Dimensions::new(1, 10));
    let mut buffer = FixedTextBuffer::<1, 10>::default();
    let tree = make_render_tree(&vstack, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(collect_text(&buffer), " 234 56789");
}

#[test]
fn test_oversized_layout_3_middle_pad_space() {
    // The second text view is too large to fit in the initial offer.
    let font = CharacterBufferFont {};
    let vstack = VStack::new((
        Text::str("234", &font),
        Spacer::default(),
        Text::str("56789", &font),
    ))
    .with_spacing(1)
    .foreground_color(' ');
    let offer = Size::new(1, 10);
    let env = DefaultEnvironment::non_animated();
    let layout = vstack.layout(&offer.into(), &env);
    assert_eq!(layout.resolved_size, Dimensions::new(1, 10));
    let mut buffer = FixedTextBuffer::<1, 10>::default();
    let tree = make_render_tree(&vstack, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(collect_text(&buffer), "234  56789");
}

#[test]
fn test_oversized_layout_3_trailing_pad_space() {
    // The second text view is too large to fit in the initial offer.
    let font = CharacterBufferFont {};
    let vstack = VStack::new((
        Text::str("234", &font),
        Text::str("56789", &font),
        Spacer::default(),
    ))
    .with_spacing(1)
    .foreground_color(' ');
    let offer = Size::new(1, 10);
    let env = DefaultEnvironment::non_animated();
    let layout = vstack.layout(&offer.into(), &env);
    assert_eq!(layout.resolved_size, Dimensions::new(1, 10));
    let mut buffer = FixedTextBuffer::<1, 10>::default();
    let tree = make_render_tree(&vstack, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(collect_text(&buffer), "234 56789 ");
}

#[test]
fn test_undersized_layout_3_middle_pad() {
    let font = CharacterBufferFont {};
    let vstack = VStack::new((
        Text::str("234", &font),
        Spacer::default(),
        Text::str("5678", &font),
    ))
    .foreground_color(' ');
    let offer = Size::new(1, 10);
    let env = DefaultEnvironment::non_animated();
    let layout = vstack.layout(&offer.into(), &env);
    assert_eq!(layout.resolved_size, Dimensions::new(1, 10));
    let mut buffer = FixedTextBuffer::<1, 10>::default();
    let tree = make_render_tree(&vstack, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(collect_text(&buffer), "234   5678");
}

#[ignore = "Not sure if I care about exactly which view gets the extra space first, just that it is all allocated"]
#[test]
fn test_layout_3_remainder_allocation() {
    // The VStack should attempt to lay out the views into the full width of the offer.
    let font = CharacterBufferFont {};
    let vstack = VStack::new((
        Text::str("aaa", &font),
        Text::str("bbb", &font),
        Text::str("ccc", &font),
    ))
    .foreground_color(' ');
    let mut buffer = FixedTextBuffer::<1, 10>::default();
    let offer = Size::new(1, 7);
    let tree = make_render_tree(&vstack, offer);
    tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(collect_text(&buffer), "aaabbcc   ");

    let offer = Size::new(1, 8);
    let tree = make_render_tree(&vstack, offer);
    tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(collect_text(&buffer), "aaabbbcc  ");

    let offer = Size::new(1, 9);
    let tree = make_render_tree(&vstack, offer);
    tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(collect_text(&buffer), "aaabbbccc ");

    let offer = Size::new(1, 10);
    let tree = make_render_tree(&vstack, offer);
    tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(collect_text(&buffer), "aaabbbccc ");
}

#[test]
fn test_layout_3_remainder_allocation_sizing_only() {
    // The VStack should attempt to lay out the views into the full width of the offer.
    let font = CharacterBufferFont {};
    let vstack = VStack::new((
        Text::str("aaa", &font),
        Text::str("bbb", &font),
        Text::str("ccc", &font),
    ));
    let env = DefaultEnvironment::non_animated();
    for height in 1..9 {
        let offer = Size::new(1, height);
        let layout = vstack.layout(&offer.into(), &env);
        assert_eq!(layout.resolved_size, Dimensions::new(1, height));
    }
}

#[test]
fn test_layout_3_horizontal_alignment_trailing() {
    // The VStack should attempt to lay out the views into the full width of the offer.
    let font = CharacterBufferFont {};
    let vstack = VStack::new((
        Text::str("aaa", &font),
        Divider::default().foreground_color('-'),
        Text::str("ccccccc", &font),
    ))
    .with_alignment(HorizontalAlignment::Trailing)
    .with_spacing(1);
    let mut buffer = FixedTextBuffer::<6, 7>::default();
    let tree = make_render_tree(&vstack, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());
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
    let font = CharacterBufferFont {};
    let vstack = VStack::new((
        Text::str("aaa", &font),
        Divider::default().foreground_color('-'),
        Text::str("cccc", &font),
    ))
    .with_alignment(HorizontalAlignment::Center);
    let mut buffer = FixedTextBuffer::<7, 5>::default();
    let tree = make_render_tree(&vstack, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "  aaa  ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "-------");
    assert_eq!(buffer.text[2].iter().collect::<String>(), " cccc  ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "       ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "       ");
}

#[test]
fn test_layout_3_alignment_leading() {
    // The VStack should attempt to lay out the views into the full width of the offer.
    let font = CharacterBufferFont {};
    let vstack = VStack::new((
        Text::str("aaa", &font),
        Divider::default().foreground_color('-'),
        Text::str("ccc", &font).multiline_text_alignment(HorizontalTextAlignment::Trailing),
    ))
    .with_alignment(HorizontalAlignment::Leading)
    .with_spacing(1);
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let tree = make_render_tree(&vstack, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());
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
    let vstack = VStack::new((
        Divider::default().foreground_color('-'),
        HStack::new((Divider::default().foreground_color('|'), Spacer::default())),
        Divider::default().foreground_color('-'),
    ));
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let tree = make_render_tree(&vstack, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "------");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "|     ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "|     ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "|     ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "------");
}

#[test]
fn test_layout_direction_is_set_inner_vstack() {
    let hstack = HStack::new((
        Divider::default().foreground_color('|'),
        VStack::new((Divider::default().foreground_color('-'), Spacer::default())),
        Divider::default().foreground_color('|'),
    ));
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let tree = make_render_tree(&hstack, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "|----|");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "|    |");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "|    |");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "|    |");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "|    |");
}

#[test]
fn test_flexible_layout_fills_frame_10k() {
    let font = CharacterBufferFont {};
    let stack = VStack::new((
    HStack::new((
        Text::str(
            "This text is centered horizontally in the middle of its space\nThe stack however, has bottom alignment.",
            &font,
                )
                .multiline_text_alignment(HorizontalTextAlignment::Center),
        Spacer::default(),
        Text::str(
            "This text is aligned to the right, with trailing multi-line text alignment",
            &font,
                )
                .multiline_text_alignment(HorizontalTextAlignment::Trailing),
        ))
        .with_spacing(1)
        .with_alignment(VerticalAlignment::Bottom),
    Divider::default(),
    VStack::new((
        Spacer::default(),
        Text::str(
            "This is several lines of text.\nEach line is centered in the available space.\n Spacers are used to fill all the remaining verical space and align the content within it.\n2 points of padding are around this text",
            &font,
        )
            .multiline_text_alignment(HorizontalTextAlignment::Center)
            .padding(2),
        Divider::default(),
        )),
    ));

    let env = DefaultEnvironment::non_animated();
    // The spacers in this view should always cause the stack size to equal the offer size
    for width in 1..100 {
        for height in 1..100 {
            let size = Size::new(width, height);
            let layout = stack.layout(&size.into(), &env);
            assert_eq!(size, layout.resolved_size.into());
        }
    }
}

#[test]
fn test_layout_3_extra_space_allocation() {
    // The VStack should attempt to lay out the views into the full width of the offer.
    let font = CharacterBufferFont {};
    let vstack = VStack::new((
        Rectangle.foreground_color('x'),
        Text::str("Text text", &font).multiline_text_alignment(HorizontalTextAlignment::Center),
        Rectangle.foreground_color('+'),
    ))
    .with_spacing(0);
    let mut buffer = FixedTextBuffer::<6, 10>::default();
    let tree = make_render_tree(&vstack, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "xxxxxx");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "xxxxxx");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "xxxxxx");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "xxxxxx");
    assert_eq!(buffer.text[4].iter().collect::<String>(), " Text ");
    assert_eq!(buffer.text[5].iter().collect::<String>(), " text ");
    assert_eq!(buffer.text[6].iter().collect::<String>(), "++++++");
    assert_eq!(buffer.text[7].iter().collect::<String>(), "++++++");
    assert_eq!(buffer.text[8].iter().collect::<String>(), "++++++");
    assert_eq!(buffer.text[9].iter().collect::<String>(), "++++++");
    // multiline text alignment applies within the frame of the text
    // the leading c is correct
}

#[test]
fn empty_view_does_not_recieve_spacing() {
    // The VStack should attempt to lay out the views into the full width of the offer.
    let font = CharacterBufferFont {};
    let vstack = VStack::new((Text::str("a", &font), EmptyView, Text::str("c", &font)))
        .with_spacing(1)
        .foreground_color(' ');
    let mut buffer = FixedTextBuffer::<7, 5>::default();
    let tree = make_render_tree(&vstack, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "a      ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "       ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "c      ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "       ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "       ");
}
