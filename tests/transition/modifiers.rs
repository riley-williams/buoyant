//! Tests that all the modifier views pass through the transition property.

use buoyant::{
    environment::DefaultEnvironment,
    primitives::Size,
    transition::{Direction, Slide, Transition},
    view::prelude::*,
};

fn all_the_modifiers() -> impl View<char, ()> {
    Circle
        .transition(Slide::top())
        .priority(1)
        .animated(
            Animation::linear(std::time::Duration::from_millis(100)),
            false,
        )
        .aspect_ratio(Ratio::Ideal, ContentMode::Fill)
        .background(Alignment::Bottom, Rectangle)
        .clipped()
        .erase_captures()
        .fixed_size(true, false)
        .flex_frame()
        .foreground_color('e')
        .frame_sized(125, 125)
        .geometry_group()
        .hint_background_color('b')
        .offset(10, 10)
        .overlay(Alignment::BottomTrailing, Rectangle)
        .padding(Edges::All, 10)
}

#[test]
fn modifiers_hand_off_transition() {
    let view = all_the_modifiers();
    let mut state = view.build_state(&mut ());
    let layout = view.layout(
        &Size::new(1000, 1000).into(),
        &DefaultEnvironment::default(),
        &mut (),
        &mut state,
    );
    let size = layout.resolved_size.into();
    // looks like a slide and quacks like a slide
    for factor in 0..255 {
        assert_eq!(
            ViewLayout::<()>::transition(&view).transform(Direction::In, factor, size),
            Slide::top().transform(Direction::In, factor, size),
        );
        assert_eq!(
            ViewLayout::<()>::transition(&view).transform(Direction::Out, factor, size),
            Slide::top().transform(Direction::Out, factor, size),
        );
        assert_eq!(
            ViewLayout::<()>::transition(&view).opacity(Direction::In, factor),
            Slide::top().opacity(Direction::In, factor),
        );
        assert_eq!(
            ViewLayout::<()>::transition(&view).opacity(Direction::Out, factor),
            Slide::top().opacity(Direction::Out, factor),
        );
    }
}

// doesn't really belong here...
// also in a module because I smartly have two methods named priority
mod passthrough {
    #[test]
    fn modifiers_pass_through_priority() {
        use buoyant::view::ViewLayout;
        let view = super::all_the_modifiers();
        assert_eq!(view.priority(), 1);
    }
}
