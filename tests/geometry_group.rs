use buoyant::{
    font::CharacterBufferFont, primitives::Size, render::Render as _,
    render_target::FixedTextBuffer, view::prelude::*,
};
mod common;
use common::{make_render_tree, tap};

#[test]
fn geometry_group_retains_text_offset() {
    let font = CharacterBufferFont {};
    let content = VStack::new((
        Text::new("aa aa", &font).foreground_color(' '),
        Text::new("bb", &font).geometry_group(),
        Text::new("ccc", &font),
    ))
    .geometry_group();
    let mut buffer = FixedTextBuffer::<6, 4>::default();
    let tree = make_render_tree(&content, buffer.size(), &mut ());
    tree.render(&mut buffer, &' ');
    assert_eq!(buffer.text[0].iter().collect::<String>(), "aa aa ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), " bb   ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), " ccc  ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "      ");
}

#[test]
fn geometry_group_retains_fill_offset() {
    let font = CharacterBufferFont {};
    let content = VStack::new((
        Text::new("aa aa", &font).foreground_color(' '),
        Rectangle
            .frame_sized(2, 1)
            .foreground_color('b')
            .geometry_group(),
        Text::new("ccc", &font),
    ))
    .geometry_group();
    let mut buffer = FixedTextBuffer::<6, 4>::default();
    let tree = make_render_tree(&content, buffer.size(), &mut ());
    tree.render(&mut buffer, &' ');
    assert_eq!(buffer.text[0].iter().collect::<String>(), "aa aa ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), " bb   ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), " ccc  ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "      ");
}

#[test]
fn event_offset_is_preserved() {
    let view = Button::new(|x: &mut u32| *x += 1, |_| Rectangle)
        .geometry_group()
        .offset(3, 3);
    let mut x = 0;
    let mut state = view.build_state(&mut x);
    let size = Size::new(3, 3);

    tap(&view, &mut x, &mut state, size, 0, 0);
    assert_eq!(x, 0);

    tap(&view, &mut x, &mut state, size, 1, 1);
    assert_eq!(x, 0);

    tap(&view, &mut x, &mut state, size, 3, 3);
    assert_eq!(x, 1);

    tap(&view, &mut x, &mut state, size, 6, 3);
    assert_eq!(x, 1);
}
