use buoyant::{
    environment::DefaultEnvironment,
    font::BufferCharacterFont,
    layout::{Layout as _, VerticalAlignment},
    primitives::Point,
    render::CharacterRender as _,
    render_target::{CharacterRenderTarget as _, FixedTextBuffer},
    view::{ForEach, HStack, Spacer, Text},
};

static FONT: BufferCharacterFont = BufferCharacterFont {};

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
        HStack::three(
            Text::char(&user.name, &FONT),
            Spacer::default(),
            Text::char(&user.age, &FONT),
        )
        .alignment(VerticalAlignment::Bottom)
    });
    let env = DefaultEnvironment::new(());
    let mut buffer = FixedTextBuffer::<10, 5>::default();
    let layout = view.layout(buffer.size(), &env);
    view.render(&mut buffer, &layout, Point::zero(), &env);
    assert_eq!(buffer.text[0].iter().collect::<String>(), "Alice   99");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "Bob      2");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "Person    ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "Name    77");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "          ");
}
