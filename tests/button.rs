use core::time::Duration;

mod common;
use crate::common::{touch_down, touch_move, touch_up};

use buoyant::{
    environment::DefaultEnvironment,
    event::EventContext,
    font::CharacterBufferFont,
    primitives::{Point, Size},
    render::Render,
    render_target::FixedTextBuffer,
    view::prelude::*,
};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct AppState {
    pub a: i32,
    pub b: i32,
}

fn counter_view(state: &AppState) -> impl View<char, AppState> + use<> {
    VStack::new((
        Lens::new(count_view(state.a), |captures: &mut AppState| {
            &mut captures.a
        }),
        Lens::new(count_view(state.b), |captures: &mut AppState| {
            &mut captures.b
        }),
    ))
    .with_alignment(HorizontalAlignment::Leading)
}

fn count_view(count: i32) -> impl View<char, i32> {
    VStack::new((
        Text::new_fmt::<16>(format_args!("count: {count}"), &CharacterBufferFont),
        Button::new(
            |count: &mut i32| {
                *count += 1;
            },
            |_| Text::new("Increment", &CharacterBufferFont),
        ),
        Button::new(
            |count: &mut i32| {
                *count -= 1;
            },
            |_| Text::new("Decrement", &CharacterBufferFont),
        ),
    ))
}

fn rebuild_tree<V, F>(
    app_state: &mut AppState,
    view_state: &mut V::State,
    view: F,
    size: Size,
) -> V::Renderables
where
    V: View<char, AppState>,
    F: Fn(&AppState) -> V,
{
    let env = DefaultEnvironment::default();
    let view = (view)(app_state);
    let layout = view.layout(&size.into(), &env, app_state, view_state);
    view.render_tree(&layout, Point::zero(), &env, app_state, view_state)
}

#[test]
fn increment_single_frame() {
    let mut app_state = AppState::default();
    let input = buoyant::event::input::Input::new();
    let view = counter_view(&app_state);
    let mut view_state = view.build_state(&mut app_state);

    let mut buffer = FixedTextBuffer::<10, 6>::default();
    let mut tree = rebuild_tree(&mut app_state, &mut view_state, counter_view, buffer.size());
    tree.render(&mut buffer, &' ');
    assert_eq!(buffer.text[0].iter().collect::<String>(), "count: 0  ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "Increment ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "Decrement ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "count: 0  ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "Increment ");
    assert_eq!(buffer.text[5].iter().collect::<String>(), "Decrement ");
    assert_eq!(app_state, AppState { a: 0, b: 0 });

    view.handle_event(
        &touch_down(1, 1),
        &EventContext::new(Duration::ZERO, &input),
        &mut tree,
        &mut app_state,
        &mut view_state,
    );
    assert_eq!(app_state, AppState { a: 0, b: 0 });
    view.handle_event(
        &touch_up(1, 1),
        &EventContext::new(Duration::ZERO, &input),
        &mut tree,
        &mut app_state,
        &mut view_state,
    );
    assert_eq!(app_state, AppState { a: 1, b: 0 });

    let mut buffer = FixedTextBuffer::<10, 6>::default();
    let tree = rebuild_tree(&mut app_state, &mut view_state, counter_view, buffer.size());
    tree.render(&mut buffer, &' ');
    assert_eq!(buffer.text[0].iter().collect::<String>(), "count: 1  ");
    assert_eq!(buffer.text[1].iter().collect::<String>(), "Increment ");
    assert_eq!(buffer.text[2].iter().collect::<String>(), "Decrement ");
    assert_eq!(buffer.text[3].iter().collect::<String>(), "count: 0  ");
    assert_eq!(buffer.text[4].iter().collect::<String>(), "Increment ");
    assert_eq!(buffer.text[5].iter().collect::<String>(), "Decrement ");
}

#[test]
fn drag_cancel() {
    let mut app_state = AppState::default();
    let input = buoyant::event::input::Input::new();

    let view = counter_view(&app_state);
    let mut view_state = view.build_state(&mut app_state);

    let mut tree = rebuild_tree(
        &mut app_state,
        &mut view_state,
        counter_view,
        Size::new(10, 6),
    );

    view.handle_event(
        &touch_down(1, 1),
        &EventContext::new(Duration::ZERO, &input),
        &mut tree,
        &mut app_state,
        &mut view_state,
    );
    assert_eq!(app_state, AppState { a: 0, b: 0 });
    view.handle_event(
        &touch_move(1, 2),
        &EventContext::new(Duration::ZERO,&input),
        &mut tree,
        &mut app_state,
        &mut view_state,
    );
    assert_eq!(app_state, AppState { a: 0, b: 0 });
    view.handle_event(
        &touch_up(1, 2),
        &EventContext::new(Duration::ZERO,&input),
        &mut tree,
        &mut app_state,
        &mut view_state,
    );
    assert_eq!(app_state, AppState { a: 0, b: 0 });
}

#[test]
fn drag_cancel_uncancel() {
    let mut app_state = AppState::default();
    let input = buoyant::event::input::Input::new();
    let view = counter_view(&app_state);
    let mut view_state = view.build_state(&mut app_state);

    let mut tree = rebuild_tree(
        &mut app_state,
        &mut view_state,
        counter_view,
        Size::new(10, 6),
    );

    view.handle_event(
        &touch_down(1, 1),
        &EventContext::new(Duration::ZERO, &input),
        &mut tree,
        &mut app_state,
        &mut view_state,
    );
    assert_eq!(app_state, AppState { a: 0, b: 0 });
    view.handle_event(
        &touch_move(1, 2),
        &EventContext::new(Duration::ZERO, &input),
        &mut tree,
        &mut app_state,
        &mut view_state,
    );
    assert_eq!(app_state, AppState { a: 0, b: 0 });
    view.handle_event(
        &touch_move(5, 1),
        &EventContext::new(Duration::ZERO, &input),
        &mut tree,
        &mut app_state,
        &mut view_state,
    );
    assert_eq!(app_state, AppState { a: 0, b: 0 });
    view.handle_event(
        &touch_up(5, 1),
        &EventContext::new(Duration::ZERO, &input),
        &mut tree,
        &mut app_state,
        &mut view_state,
    );
    assert_eq!(app_state, AppState { a: 1, b: 0 });
}
