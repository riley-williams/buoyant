use buoyant::{
    app::{App, Harness as _},
    focus::Role,
    primitives::Size,
    render::ContentShape,
    view::prelude::*,
};

#[allow(clippy::trivially_copy_pass_by_ref)]
fn three_buttons_wrap(_: &()) -> impl View<(), ()> + use<> {
    VStack::new((
        Button::new(|(): &mut ()| {}, |_| Rectangle.content_shape(Circle)),
        Button::new(|(): &mut ()| {}, |_| Circle.content_shape(Rectangle)),
        Button::new(|(): &mut ()| {}, |_| Rectangle.content_shape(Capsule)),
        Button::new(
            |(): &mut ()| {},
            |_| Rectangle.content_shape(RoundedRectangle::new(5)),
        ),
    ))
}

#[test]
fn overrides_content_shapes() {
    let mut harness =
        App::new((), Size::new(100, 200), three_buttons_wrap).with_roles(Role::Button);

    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Circle(_))
    ));

    assert!(matches!(
        harness.next().shape(),
        Some(ContentShape::Rectangle(_))
    ));

    assert!(matches!(
        harness.next().shape(),
        Some(ContentShape::RoundedRectangle(_))
    ));

    assert!(matches!(
        harness.next().shape(),
        Some(ContentShape::RoundedRectangle(_))
    ));
}
