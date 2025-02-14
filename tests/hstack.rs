use std::iter::zip;

use buoyant::environment::DefaultEnvironment;
use buoyant::font::CharacterBufferFont;
use buoyant::layout::{Layout, VerticalAlignment};
use buoyant::primitives::{Dimensions, Point, ProposedDimension, ProposedDimensions, Size};
use buoyant::render::CharacterRenderTarget;
use buoyant::render::{CharacterRender, Renderable};
use buoyant::render_target::FixedTextBuffer;
use buoyant::view::{make_render_tree, RenderExtensions as _};
use buoyant::view::{shape::Rectangle, Divider, EmptyView, HStack, LayoutExtensions, Spacer, Text};

#[test]
fn test_greedy_layout_2() {
    let hstack = HStack::new((Spacer::default(), Spacer::default()));
    let offer = Size::new(100, 100);
    let env = DefaultEnvironment::non_animated();
    let layout = hstack.layout(&offer.into(), &env);
    assert_eq!(layout.resolved_size, Dimensions::new(100, 0));
}

#[test]
fn test_oversized_layout_2() {
    let vstack = HStack::new((Divider::default().padding(2), Spacer::default()));
    let offer = Size::new(10, 0);
    let env = DefaultEnvironment::non_animated();
    let layout = vstack.layout(&offer.into(), &env);
    assert_eq!(layout.resolved_size, Dimensions::new(10, 0));
}

#[test]
fn test_oversized_layout_3() {
    let vstack = HStack::new((
        Divider::default(),
        Divider::default().padding(2),
        Spacer::default(),
    ));
    let offer = Size::new(10, 0);
    let env = DefaultEnvironment::non_animated();
    let layout = vstack.layout(&offer.into(), &env);
    assert_eq!(layout.resolved_size, Dimensions::new(10, 0));
}

#[test]
fn test_undersized_layout_2() {
    let font = CharacterBufferFont {};
    let hstack = HStack::new((Text::str("123", &font), Text::str("4567", &font))).with_spacing(1);
    let offer = Size::new(50, 1);
    let env = DefaultEnvironment::non_animated();
    let layout = hstack.layout(&offer.into(), &env);
    assert_eq!(layout.resolved_size, Dimensions::new(8, 1));
}

#[test]
fn test_horizontal_render_2() {
    let font = CharacterBufferFont {};
    let hstack = HStack::new((Text::str("123", &font), Text::str("4567", &font)))
        .with_spacing(1)
        .foreground_color(' ');
    let mut buffer = FixedTextBuffer::<9, 1>::default();
    let tree = make_render_tree(&hstack, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(buffer.text[0].iter().collect::<String>(), "123 4567 ");
}

#[test]
fn test_undersized_layout_3_left_pad() {
    let font = CharacterBufferFont {};
    let hstack = HStack::new((
        Text::str("123", &font),
        Text::str("4567", &font),
        Spacer::default(),
    ))
    .foreground_color(' ');
    let offer = Size::new(10, 1);
    let env = DefaultEnvironment::non_animated();
    let layout = hstack.layout(&offer.into(), &env);
    assert_eq!(layout.resolved_size, Dimensions::new(10, 1));
    let mut buffer = FixedTextBuffer::<10, 1>::default();
    hstack
        .render_tree(&layout, Point::zero(), &env)
        .render(&mut buffer, &' ', Point::zero());

    assert_eq!(buffer.text[0].iter().collect::<String>(), "1234567   ");
}
#[test]
fn test_undersized_layout_3_right_pad_space() {
    let font = CharacterBufferFont {};
    let hstack = HStack::new((
        Spacer::default(),
        Text::str("234", &font),
        Text::str("5678", &font),
    ))
    .with_spacing(1)
    .foreground_color(' ');
    let offer = Size::new(10, 1);
    let env = DefaultEnvironment::non_animated();
    let layout = hstack.layout(&offer.into(), &env);
    assert_eq!(layout.resolved_size, Dimensions::new(10, 1));
    let mut buffer = FixedTextBuffer::<10, 1>::default();
    hstack
        .render_tree(&layout, Point::zero(), &env)
        .render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "  234 5678");
}

#[test]
fn test_oversized_layout_3_leading_pad_space() {
    // The second text view is too large to fit in the initial offer.
    let font = CharacterBufferFont {};
    let hstack = HStack::new((
        Spacer::default(),
        Text::str("234", &font),
        Text::str("56789", &font),
    ))
    .with_spacing(1)
    .foreground_color(' ');
    let offer = Size::new(10, 1);
    let env = DefaultEnvironment::non_animated();
    let layout = hstack.layout(&offer.into(), &env);
    assert_eq!(layout.resolved_size, Dimensions::new(10, 1));
    let mut buffer = FixedTextBuffer::<10, 1>::default();
    hstack
        .render_tree(&layout, Point::zero(), &env)
        .render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), " 234 56789");
}

#[test]
fn test_undersized_layout_3_middle_pad() {
    let font = CharacterBufferFont {};
    let hstack = HStack::new((
        Text::str("234", &font),
        Spacer::default(),
        Text::str("5678", &font),
    ))
    .foreground_color(' ');
    let offer = Size::new(10, 1);
    let env = DefaultEnvironment::non_animated();
    let layout = hstack.layout(&offer.into(), &env);
    assert_eq!(layout.resolved_size, Dimensions::new(10, 1));
    let mut buffer = FixedTextBuffer::<10, 1>::default();
    hstack
        .render_tree(&layout, Point::zero(), &env)
        .render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "234   5678");
}

#[test]
fn test_oversized_layout_3_middle_pad_space() {
    // The third text view is too large to fit in the initial offer.
    let font = CharacterBufferFont {};
    let hstack = HStack::new((
        Text::str("234", &font),
        Spacer::default(),
        Text::str("56789", &font),
    ))
    .with_spacing(1)
    .foreground_color(' ');
    let offer = Size::new(10, 1);
    let env = DefaultEnvironment::non_animated();
    let layout = hstack.layout(&offer.into(), &env);
    assert_eq!(layout.resolved_size, Dimensions::new(10, 1));
    let mut buffer = FixedTextBuffer::<10, 1>::default();
    hstack
        .render_tree(&layout, Point::zero(), &env)
        .render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "234  56789");
}

#[test]
fn test_oversized_layout_3_trailing_pad_space() {
    // The second text view is too large to fit in the initial offer.
    let font = CharacterBufferFont {};
    let hstack = HStack::new((
        Text::str("234", &font),
        Text::str("56789", &font),
        Spacer::default(),
    ))
    .with_spacing(1)
    .foreground_color(' ');
    let offer = Size::new(10, 1);
    let env = DefaultEnvironment::non_animated();
    let layout = hstack.layout(&offer.into(), &env);
    assert_eq!(layout.resolved_size, Dimensions::new(10, 1));
    let mut buffer = FixedTextBuffer::<10, 1>::default();
    hstack
        .render_tree(&layout, Point::zero(), &env)
        .render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "234 56789 ");
}

#[test]
fn test_layout_3_remainder_allocation() {
    // The HStack should attempt to lay out the views into the full width of the offer.
    let font = CharacterBufferFont {};
    let hstack = HStack::new((
        Text::str("aaa", &font),
        Text::str("bbb", &font),
        Text::str("ccc", &font),
    ))
    .foreground_color(' ');
    let env = DefaultEnvironment::non_animated();
    let mut buffer = FixedTextBuffer::<10, 1>::default();
    let offer = Size::new(7, 1);
    let layout = hstack.layout(&offer.into(), &env);
    hstack
        .render_tree(&layout, Point::zero(), &env)
        .render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "aaabbcc   ");

    let offer = Size::new(8, 1);
    let layout = hstack.layout(&offer.into(), &env);
    hstack
        .render_tree(&layout, Point::zero(), &env)
        .render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "aaabbbcc  ");

    let offer = Size::new(9, 1);
    let layout = hstack.layout(&offer.into(), &env);
    hstack
        .render_tree(&layout, Point::zero(), &env)
        .render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "aaabbbccc ");

    let offer = Size::new(10, 1);
    let layout = hstack.layout(&offer.into(), &env);
    hstack
        .render_tree(&layout, Point::zero(), &env)
        .render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "aaabbbccc ");
}

#[test]
fn test_layout_3_vertical_alignment_bottom() {
    // The HStack should attempt to lay out the views into the full width of the offer.
    let font = CharacterBufferFont {};
    let hstack = HStack::new((
        Text::str("aaa", &font),
        Divider::default().foreground_color('|'),
        Text::str("ccc", &font),
    ))
    .with_alignment(VerticalAlignment::Bottom)
    .with_spacing(1);
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let tree = make_render_tree(&hstack, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(buffer.text[0].iter().collect::<String>(), "   |  ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "   |  ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "   | c");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "aa | c");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "a  | c");
}

#[test]
fn test_layout_3_vertical_alignment_center() {
    // The HStack should attempt to lay out the views into the full width of the offer.
    let font = CharacterBufferFont {};
    let hstack = HStack::new((
        Text::str("aaa", &font),
        Divider::default().foreground_color('|'),
        Text::str("ccc", &font),
    ))
    .with_alignment(VerticalAlignment::Center)
    .with_spacing(1);
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let tree = make_render_tree(&hstack, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(buffer.text[0].iter().collect::<String>(), "   |  ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "aa | c");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "a  | c");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "   | c");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "   |  ");
}

#[test]
fn test_layout_3_vertical_alignment_top() {
    // The HStack should attempt to lay out the views into the full width of the offer.
    let font = CharacterBufferFont {};
    let hstack = HStack::new((
        Text::str("aaa", &font),
        Divider::default().foreground_color('|'),
        Text::str("ccc", &font),
    ))
    .with_alignment(VerticalAlignment::Top)
    .with_spacing(1);
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let tree = make_render_tree(&hstack, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(buffer.text[0].iter().collect::<String>(), "aa | c");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "a  | c");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "   | c");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "   |  ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "   |  ");
}

#[test]
fn test_minimal_offer_extra_space_1() {
    // The HStack should offer remaining space when the views do not consume the full width.
    let font = CharacterBufferFont {};
    let hstack = HStack::new((
        Text::str("a b c d e f", &font),
        Text::str("g h i", &font),
        Text::str("j", &font),
    ))
    .with_alignment(VerticalAlignment::Top)
    .with_spacing(1)
    .foreground_color(' ');

    let mut buffer = FixedTextBuffer::<19, 5>::default();

    let tree = make_render_tree(&hstack, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());

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

#[test]
fn test_layout_3_extra_space_allocation() {
    // The HStack should attempt to lay out the views into the full width of the offer.
    let font = CharacterBufferFont {};
    let hstack = HStack::new((
        Rectangle.foreground_color('x'),
        Text::str("T", &font),
        Rectangle.foreground_color('+'),
    ))
    .with_spacing(0);
    let mut buffer = FixedTextBuffer::<9, 3>::default();
    let tree = make_render_tree(&hstack, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(buffer.text[0].iter().collect::<String>(), "xxxx ++++");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "xxxxT++++");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "xxxx ++++");
}

fn view(
    max_width_1: u16,
    max_width_2: u16,
    max_width_3: u16,
) -> impl Renderable<char, Renderables: CharacterRender<char>> {
    HStack::new((
        Rectangle
            .foreground_color('x')
            .flex_frame()
            .with_min_width(3)
            .with_max_width(max_width_1),
        Rectangle
            .foreground_color('-')
            .flex_frame()
            .with_min_width(2)
            .with_max_width(max_width_2),
        Rectangle
            .foreground_color('+')
            .flex_frame()
            .with_min_width(4)
            .with_max_width(max_width_3),
    ))
}

#[ignore = "This test is correct, but putting off implementation fix for now"]
#[test]
fn stack_fits_subviews_regardless_of_flexibility_order() {
    let mut buffer = FixedTextBuffer::<9, 1>::default();
    for w1 in 1..12 {
        for w2 in 1..12 {
            for w3 in 1..12 {
                let view = view(w1, w2, w3);
                let tree = make_render_tree(&view, buffer.size());
                tree.render(&mut buffer, &' ', Point::zero());
                // This is the only arrangement that fits
                assert_eq!(buffer.text[0].iter().collect::<String>(), "xxx--++++");
            }
        }
    }
}

#[test]
fn empty_view_does_not_create_extra_spacing() {
    // The HStack should attempt to lay out the views into the full width of the offer.
    let font = CharacterBufferFont {};
    let hstack = HStack::new((Text::str("aaa", &font), EmptyView, Text::str("ccc", &font)))
        .with_alignment(VerticalAlignment::Top)
        .with_spacing(2)
        .foreground_color(' ');
    let mut buffer = FixedTextBuffer::<6, 5>::default();
    let tree = make_render_tree(&hstack, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());

    assert_eq!(buffer.text[0].iter().collect::<String>(), "aa  cc");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "a   c ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "      ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "      ");
}

#[test]
fn infinite_width_offer_results_in_sum_of_subview_widths() {
    let hstack = HStack::new((
        Rectangle.frame(Some(8), Some(3), None, None),
        Rectangle.frame(Some(40), Some(1), None, None),
        Rectangle.frame(Some(200), Some(8), None, None),
    ))
    .with_spacing(1);
    let offer = ProposedDimensions {
        width: ProposedDimension::Infinite,
        height: ProposedDimension::Exact(10),
    };
    let env = DefaultEnvironment::non_animated();
    let layout = hstack.layout(&offer, &env);
    assert_eq!(layout.resolved_size, Dimensions::new(248 + 2, 8));
}

#[test]
fn compact_width_offer_results_in_sum_of_subview_widths() {
    let hstack = HStack::new((
        Rectangle.frame(Some(8), Some(3), None, None),
        Rectangle.frame(Some(40), Some(1), None, None),
        Rectangle.frame(Some(200), Some(8), None, None),
    ))
    .with_spacing(1);
    let offer = ProposedDimensions {
        width: ProposedDimension::Compact,
        height: ProposedDimension::Exact(10),
    };
    let env = DefaultEnvironment::non_animated();
    let layout = hstack.layout(&offer, &env);
    assert_eq!(layout.resolved_size, Dimensions::new(248 + 2, 8));
}

#[test]
fn infinite_width_offer_results_in_sum_of_subview_widths_minus_empties() {
    let hstack = HStack::new((
        Rectangle.frame(Some(8), Some(3), None, None),
        EmptyView,
        Rectangle.frame(Some(200), Some(8), None, None),
    ))
    .with_spacing(1);
    let offer = ProposedDimensions {
        width: ProposedDimension::Infinite,
        height: ProposedDimension::Exact(10),
    };
    let env = DefaultEnvironment::non_animated();
    let layout = hstack.layout(&offer, &env);
    assert_eq!(layout.resolved_size, Dimensions::new(208 + 1, 8));
}

#[test]
fn compact_width_offer_results_in_sum_of_subview_widths_minus_empties() {
    let hstack = HStack::new((
        Rectangle.frame(Some(8), Some(3), None, None),
        EmptyView,
        Rectangle.frame(Some(200), Some(8), None, None),
    ))
    .with_spacing(1);
    let offer = ProposedDimensions {
        width: ProposedDimension::Compact,
        height: ProposedDimension::Exact(10),
    };
    let env = DefaultEnvironment::non_animated();
    let layout = hstack.layout(&offer, &env);
    assert_eq!(layout.resolved_size, Dimensions::new(208 + 1, 8));
}
