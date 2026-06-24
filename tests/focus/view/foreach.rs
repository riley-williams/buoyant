use buoyant::{
    app::{App, Harness as _},
    event::{Event, EventResult, Key},
    focus::{FocusAction, Role},
    primitives::Size,
    render::ContentShape,
    view::{map_event::Mapping, prelude::*},
};

#[derive(Default)]
struct State {
    selected: Option<usize>,
}

static THREE_ITEMS: [u32; 3] = [0, 1, 2];
static TWO_ITEMS: [u32; 2] = [0, 1];
static ONE_ITEM: [u32; 1] = [0];
static EMPTY_ITEMS: [u32; 0] = [];

fn foreach_three_items(_: &State) -> impl View<(), State> + use<> {
    ForEach::<3>::new_vertical(&THREE_ITEMS, |&item| {
        let index = item as usize;
        Button::new(move |s: &mut State| s.selected = Some(index), |_| Circle)
    })
}

fn foreach_one_item(_: &State) -> impl View<(), State> + use<> {
    ForEach::<1>::new_vertical(&ONE_ITEM, |&item| {
        let index = item as usize;
        Button::new(move |s: &mut State| s.selected = Some(index), |_| Circle)
    })
}

fn foreach_empty(_: &State) -> impl View<(), State> + use<> {
    ForEach::<0>::new_vertical(&EMPTY_ITEMS, |&_item| {
        Button::new(
            |_: &mut State| panic!("This should never be called"),
            |_| Circle,
        )
    })
}

fn stack_with_foreach(_: &State) -> impl View<(), State> + use<> {
    VStack::new((
        Button::new(|_: &mut State| {}, |_| Rectangle),
        ForEach::<2>::new_vertical(&TWO_ITEMS, |&item| {
            let index = item as usize;
            Button::new(move |s: &mut State| s.selected = Some(index), |_| Circle)
        }),
        Button::new(|_: &mut State| {}, |_| RoundedRectangle::new(10)),
    ))
}

#[test]
fn first_item_is_focusable() {
    let mut harness = App::new(State::default(), Size::new(100, 300), foreach_three_items)
        .with_roles(Role::Button);

    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Circle(_))
    ));
}

#[test]
fn navigate_forward_through_list() {
    let mut harness = App::new(State::default(), Size::new(100, 300), foreach_three_items)
        .with_roles(Role::Button);

    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Circle(_))
    ));

    assert!(matches!(
        harness.next().shape(),
        Some(ContentShape::Circle(_))
    ));

    assert!(matches!(
        harness.next().shape(),
        Some(ContentShape::Circle(_))
    ));

    assert!(matches!(harness.next(), EventResult::Deferred));
}

#[test]
fn navigate_backward_through_list() {
    let mut harness = App::new(State::default(), Size::new(100, 300), foreach_three_items)
        .with_roles(Role::Button);

    harness.focus_forward();
    harness.next();
    harness.next();

    assert!(matches!(
        harness.previous().shape(),
        Some(ContentShape::Circle(_))
    ));

    assert!(matches!(
        harness.previous().shape(),
        Some(ContentShape::Circle(_))
    ));

    assert!(matches!(harness.previous(), EventResult::Deferred));
}

#[test]
fn select_triggers_correct_action() {
    let mut harness = App::new(State::default(), Size::new(100, 300), foreach_three_items)
        .with_roles(Role::Button);

    harness.focus_forward();
    harness.next();
    harness.select();

    assert_eq!(
        harness.state().selected,
        Some(1),
        "Should select item at index 1"
    );
}

#[test]
fn empty_list_returns_deferred() {
    let mut harness =
        App::new(State::default(), Size::new(100, 100), foreach_empty).with_roles(Role::Button);

    assert!(matches!(harness.focus_forward(), EventResult::Deferred));
}

#[test]
fn focus_backward_from_end() {
    let mut harness = App::new(State::default(), Size::new(100, 300), foreach_three_items)
        .with_roles(Role::Button)
        .with_focus_at_end();

    assert!(matches!(
        harness.focus_backward().shape(),
        Some(ContentShape::Circle(_))
    ));
}

#[test]
fn focus_backward_then_previous() {
    let mut harness = App::new(State::default(), Size::new(100, 300), foreach_three_items)
        .with_roles(Role::Button)
        .with_focus_at_end();

    harness.focus_backward();

    assert!(matches!(
        harness.previous().shape(),
        Some(ContentShape::Circle(_))
    ));
}

#[test]
fn single_item_list_navigation() {
    let mut harness =
        App::new(State::default(), Size::new(100, 100), foreach_one_item).with_roles(Role::Button);

    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Circle(_))
    ));

    assert!(matches!(harness.next(), EventResult::Deferred));

    assert!(matches!(harness.previous(), EventResult::Deferred));
}

#[test]
fn stack_navigation_through_foreach() {
    let mut harness = App::new(State::default(), Size::new(100, 300), stack_with_foreach)
        .with_roles(Role::Button);

    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Rectangle(_))
    ));

    assert!(matches!(
        harness.next().shape(),
        Some(ContentShape::Circle(_))
    ));

    assert!(matches!(
        harness.next().shape(),
        Some(ContentShape::Circle(_))
    ));

    assert!(matches!(
        harness.next().shape(),
        Some(ContentShape::RoundedRectangle(_))
    ));
}

#[test]
fn empty_list_end_focus_returns_deferred() {
    let mut harness = App::new(State::default(), Size::new(100, 100), foreach_empty)
        .with_roles(Role::Button)
        .with_focus_at_end();

    assert!(matches!(harness.focus_backward(), EventResult::Deferred));
    // closure shouldn't somehow magically be called, idk
    assert!(matches!(harness.select(), EventResult::Deferred));
}

fn key_aware_foreach(_: &State) -> impl View<(), State> + use<> {
    ForEach::<3>::new_vertical(&THREE_ITEMS, |&item| {
        let index = item as usize;
        Button::new(move |s: &mut State| s.selected = Some(index), |_| Circle).map_event(
            move |event, _: &mut State| match event {
                Event::KeyDown(Key::Character('\n')) => {
                    Mapping::Replace(Event::from(FocusAction::Select))
                }
                Event::KeyUp(_) => Mapping::Defer,
                _ => Mapping::Passthrough,
            },
        )
    })
}

#[test]
fn key_down_routes_only_to_focused_child() {
    let mut harness =
        App::new(State::default(), Size::new(100, 300), key_aware_foreach).with_roles(Role::Button);

    // Focus the first item.
    assert!(matches!(
        harness.focus_forward().shape(),
        Some(ContentShape::Circle(_))
    ));
    harness.key_down(Key::Character('\n'));
    assert_eq!(harness.state().selected, Some(0));

    // Move to the second item; the key should activate it, not the first.
    harness.next();
    harness.key_down(Key::Character('\n'));
    assert_eq!(harness.state().selected, Some(1));
}
