use buoyant::render::Render;

use buoyant::{font::CharacterBufferFont, render_target::FixedTextBuffer, view::prelude::*};
mod common;
use common::make_render_tree;

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

    let view = ForEach::<10>::new_vertical(&users, |user: &User| {
        HStack::new((
            Text::new(&user.name, &FONT),
            Spacer::default(),
            Text::new(&user.age, &FONT),
        ))
        .with_alignment(VerticalAlignment::Bottom)
        .foreground_color(' ')
    });
    let mut buffer = FixedTextBuffer::<10, 5>::default();
    let tree = make_render_tree(&view, buffer.size(), &mut ());
    tree.render(&mut buffer, &' ');
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

    let view = ForEach::<10>::new_vertical(&users, |user| {
        HStack::new((Text::new(&user.name, &FONT), Text::new(&user.age, &FONT)))
            .with_alignment(VerticalAlignment::Bottom)
            .with_spacing(1)
    })
    .with_alignment(HorizontalAlignment::Leading)
    .foreground_color(' ');
    let mut buffer = FixedTextBuffer::<10, 5>::default();
    let tree = make_render_tree(&view, buffer.size(), &mut ());
    tree.render(&mut buffer, &' ');
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

    let view = ForEach::<10>::new_vertical(&users, |user| {
        HStack::new((Text::new(&user.name, &FONT), Text::new(&user.age, &FONT)))
            .with_alignment(VerticalAlignment::Bottom)
            .with_spacing(1)
    })
    .with_alignment(HorizontalAlignment::Trailing)
    .foreground_color(' ');
    let mut buffer = FixedTextBuffer::<10, 5>::default();
    let tree = make_render_tree(&view, buffer.size(), &mut ());
    tree.render(&mut buffer, &' ');
    assert_eq!(buffer.text[0].iter().collect::<String>(), " Alice 99 ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "    Bob 2 ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "Person    ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "Name   77 ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "          ");
}

#[test]
fn foreach_spacing() {
    let mut rows = heapless::Vec::<String, 10>::new();
    rows.push("Row 1".to_string()).unwrap();
    rows.push("Row 2".to_string()).unwrap();
    rows.push("Row 3".to_string()).unwrap();

    let view = ForEach::<10>::new_vertical(&rows, |name| Text::new(name, &FONT))
        .with_spacing(1)
        .foreground_color(' ');
    let mut buffer = FixedTextBuffer::<10, 5>::default();
    let tree = make_render_tree(&view, buffer.size(), &mut ());
    tree.render(&mut buffer, &' ');
    assert_eq!(buffer.text[0].iter().collect::<String>(), "Row 1     ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "          ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "Row 2     ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "          ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "Row 3     ");
}

#[test]
fn foreach_undersized() {
    let items = vec!["A", "B", "C", "D"];

    let view =
        ForEach::<2>::new_vertical(&items, |name| Text::new(name, &FONT)).foreground_color(' ');
    let mut buffer = FixedTextBuffer::<10, 5>::default();
    let tree = make_render_tree(&view, buffer.size(), &mut ());
    tree.render(&mut buffer, &' ');
    assert_eq!(buffer.text[0].iter().collect::<String>(), "A         ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "B         ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "          ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "          ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "          ");
}
