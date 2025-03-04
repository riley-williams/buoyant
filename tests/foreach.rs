use buoyant::primitives::Point;
use buoyant::render::CharacterRender;
use buoyant::render::CharacterRenderTarget;
use buoyant::view::RenderExtensions as _;
use buoyant::{
    font::CharacterBufferFont,
    layout::{HorizontalAlignment, VerticalAlignment},
    render_target::FixedTextBuffer,
    view::{make_render_tree, ForEach, HStack, Spacer, Text},
};

static FONT: CharacterBufferFont = CharacterBufferFont {};

#[derive(Debug)]
struct User {
    name: String,
    age: String,
}

#[test]
fn foreach_with_inner_wrapping_hstack() {
    let mut users = heapless::Vec::<User, 10>::new();

    users
        .push(User {
            name: "Alice".to_string(),
            age: "99".to_string(),
        })
        .unwrap();

    users
        .push(User {
            name: "Bob".to_string(),
            age: "2".to_string(),
        })
        .unwrap();

    users
        .push(User {
            name: "Person Name".to_string(),
            age: "77".to_string(),
        })
        .unwrap();

    let view = ForEach::<10, _, _, _>::new(&users, |user| {
        HStack::new((
            Text::new(&user.name, &FONT),
            Spacer::default(),
            Text::new(&user.age, &FONT),
        ))
        .with_alignment(VerticalAlignment::Bottom)
        .foreground_color(' ')
    });
    let mut buffer = FixedTextBuffer::<10, 5>::default();
    let tree = make_render_tree(&view, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "Alice   99");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "Bob      2");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "Person    ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "Name    77");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "          ");
}

#[test]
fn foreach_leading_aligned() {
    let mut users = heapless::Vec::<User, 10>::new();

    users
        .push(User {
            name: "Alice".to_string(),
            age: "99".to_string(),
        })
        .unwrap();

    users
        .push(User {
            name: "Bob".to_string(),
            age: "2".to_string(),
        })
        .unwrap();

    users
        .push(User {
            name: "Person Name".to_string(),
            age: "77".to_string(),
        })
        .unwrap();

    let view = ForEach::<10, _, _, _>::new(&users, |user| {
        HStack::new((Text::new(&user.name, &FONT), Text::new(&user.age, &FONT)))
            .with_alignment(VerticalAlignment::Bottom)
            .with_spacing(1)
    })
    .with_alignment(HorizontalAlignment::Leading)
    .foreground_color(' ');
    let mut buffer = FixedTextBuffer::<10, 5>::default();
    let tree = make_render_tree(&view, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "Alice 99  ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "Bob 2     ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "Person    ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "Name   77 ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "          ");
}

#[test]
fn foreach_trailing_aligned() {
    let mut users = heapless::Vec::<User, 10>::new();

    users
        .push(User {
            name: "Alice".to_string(),
            age: "99".to_string(),
        })
        .unwrap();

    users
        .push(User {
            name: "Bob".to_string(),
            age: "2".to_string(),
        })
        .unwrap();

    users
        .push(User {
            name: "Person Name".to_string(),
            age: "77".to_string(),
        })
        .unwrap();

    let view = ForEach::<10, _, _, _>::new(&users, |user| {
        HStack::new((Text::new(&user.name, &FONT), Text::new(&user.age, &FONT)))
            .with_alignment(VerticalAlignment::Bottom)
            .with_spacing(1)
    })
    .with_alignment(HorizontalAlignment::Trailing)
    .foreground_color(' ');
    let mut buffer = FixedTextBuffer::<10, 5>::default();
    let tree = make_render_tree(&view, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), " Alice 99 ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "    Bob 2 ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "Person    ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "Name   77 ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "          ");
}

#[test]
fn foreach_spacing() {
    let mut names = heapless::Vec::<String, 10>::new();
    names.push("Row 1".to_string()).unwrap();
    names.push("Row 2".to_string()).unwrap();
    names.push("Row 3".to_string()).unwrap();

    let view = ForEach::<10, _, _, _>::new(&names, |name| Text::new(*name, &FONT))
        .with_spacing(1)
        .foreground_color(' ');
    let mut buffer = FixedTextBuffer::<10, 5>::default();
    let tree = make_render_tree(&view, buffer.size());
    tree.render(&mut buffer, &' ', Point::zero());
    assert_eq!(buffer.text[0].iter().collect::<String>(), "Row 1     ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "          ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "Row 2     ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "          ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "Row 3     ");
}
