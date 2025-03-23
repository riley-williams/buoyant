use buoyant::{
    environment::DefaultEnvironment,
    font::CharacterBufferFont,
    layout::Layout as _,
    primitives::{Dimensions, Size},
    view::{shape::Rectangle, HStack, Text, VStack, ViewExt},
};

/// The greedy lower priority view with a non-zero min size results in a layout overflow
/// when paired with a greedy higher priority view.
#[test]
fn oversized_layout_vstack() {
    let view = VStack::new((
        Text::new("12345", &CharacterBufferFont),
        Rectangle.frame().with_height(2).priority(-1),
        Rectangle,
    ));
    let offer = Size::new(10, 10);
    let env = DefaultEnvironment::default();
    let layout = view.layout(&offer.into(), &env);
    assert_eq!(layout.resolved_size, Dimensions::new(10, 10));
    assert_eq!(layout.sublayouts.0.resolved_size, Dimensions::new(5, 1));
    assert_eq!(layout.sublayouts.1.resolved_size, Dimensions::new(10, 2));
    assert_eq!(layout.sublayouts.2.resolved_size, Dimensions::new(10, 9));
}

/// The greedy lower priority view with a non-zero min size results in a layout overflow
/// when paired with a greedy higher priority view.
#[test]
fn oversized_layout_hstack() {
    let view = HStack::new((
        Text::new("12345", &CharacterBufferFont),
        Rectangle.frame().with_width(2).priority(-1),
        Rectangle,
    ));
    let offer = Size::new(10, 10);
    let env = DefaultEnvironment::default();
    let layout = view.layout(&offer.into(), &env);
    assert_eq!(layout.resolved_size, Dimensions::new(10, 10));
    assert_eq!(layout.sublayouts.0.resolved_size, Dimensions::new(5, 1));
    assert_eq!(layout.sublayouts.1.resolved_size, Dimensions::new(2, 10));
    assert_eq!(layout.sublayouts.2.resolved_size, Dimensions::new(5, 10));
}
