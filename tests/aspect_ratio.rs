use buoyant::{
    primitives::Point, render::Render as _, render_target::FixedTextBuffer, view::prelude::*,
};
mod common;
use common::make_render_tree;

#[test]
fn fixed_aspect_ratio_rect_fills_space() {
    let view = Rectangle
        .foreground_color('x')
        .aspect_ratio(Ratio::Fixed(6, 2), ContentMode::Fit);

    let mut buffer = FixedTextBuffer::<12, 6>::default();

    let tree = make_render_tree(&view, buffer.size());

    tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "xxxxxxxxxxxx");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "xxxxxxxxxxxx");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "xxxxxxxxxxxx");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "xxxxxxxxxxxx");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "            ");
    assert_eq!(buffer.text[5].iter().collect::<String>(), "            ");
}

#[test]
fn ideal_ratio_takes_child_ratio() {
    let view = Rectangle
        .flex_frame()
        .with_ideal_size(1200, 400)
        .foreground_color('x')
        .aspect_ratio(Ratio::Ideal, ContentMode::Fit);

    let mut buffer = FixedTextBuffer::<12, 6>::default();

    let tree = make_render_tree(&view, buffer.size());

    tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "xxxxxxxxxxxx");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "xxxxxxxxxxxx");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "xxxxxxxxxxxx");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "xxxxxxxxxxxx");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "            ");
    assert_eq!(buffer.text[5].iter().collect::<String>(), "            ");
}

#[test]
fn stacked_aspect_ratios_fill_space() {
    let view = HStack::new((
        Rectangle
            .flex_frame()
            .with_ideal_size(4, 1)
            .foreground_color('x')
            .aspect_ratio(Ratio::Ideal, ContentMode::Fit),
        Rectangle
            .foreground_color('y')
            .aspect_ratio(Ratio::Fixed(2, 3), ContentMode::Fit),
    ));

    let mut buffer = FixedTextBuffer::<12, 6>::default();

    let tree = make_render_tree(&view, buffer.size());

    tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "        yyyy");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "        yyyy");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "xxxxxxxxyyyy");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "xxxxxxxxyyyy");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "        yyyy");
    assert_eq!(buffer.text[5].iter().collect::<String>(), "        yyyy");
}
