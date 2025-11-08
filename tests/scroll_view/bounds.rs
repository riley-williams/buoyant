use std::time::Duration;

use buoyant::{
    event::EventContext,
    render::Render,
    render_target::FixedTextBuffer,
    view::{prelude::*, scroll_view::ScrollDirection},
};

use crate::common::{helpers, touch_up};
use crate::{assert_str_grid_eq, common::touch_down};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
struct TestState {
    pub above: u8,
    pub inner_1: u8,
    pub inner_2: u8,
    pub inner_3: u8,
    pub below: u8,
}

fn scroll_view() -> impl View<char, TestState> {
    VStack::new((
        Lens::new(button('a'), |s: &mut TestState| &mut s.above),
        ScrollView::new(VStack::new((
            Lens::new(button('b'), |s: &mut TestState| &mut s.inner_1),
            Lens::new(button('c'), |s: &mut TestState| &mut s.inner_2),
            Lens::new(button('d'), |s: &mut TestState| &mut s.inner_3),
        )))
        .with_direction(ScrollDirection::Vertical)
        .with_bar_visibility(buoyant::view::scroll_view::ScrollBarVisibility::Never),
        Lens::new(button('e'), |s: &mut TestState| &mut s.below),
    ))
}

fn button(c: char) -> impl View<char, u8> + use<> {
    Button::new(
        |i: &mut u8| *i += 1,
        move |is_pressed| {
            Rectangle
                .frame()
                .with_height(2)
                .flex_infinite_width(HorizontalAlignment::Center)
                .foreground_color(if is_pressed { 'X' } else { c })
        },
    )
}

fn ctx(secs: u64) -> EventContext {
    EventContext::new(Duration::from_secs(secs))
}

#[test]
fn button_above_scroll() {
    assert_eq!(
        tap_button(2, 0, true),
        TestState {
            above: 1,
            inner_1: 0,
            inner_2: 0,
            inner_3: 0,
            below: 0,
        }
    );
}
#[test]
fn button_inner_1_scroll() {
    assert_eq!(
        tap_button(2, 2, true),
        TestState {
            above: 0,
            inner_1: 1,
            inner_2: 0,
            inner_3: 0,
            below: 0,
        }
    );
}

#[test]
fn button_inner_2_scroll() {
    assert_eq!(
        tap_button(2, 4, true),
        TestState {
            above: 0,
            inner_1: 0,
            inner_2: 1,
            inner_3: 0,
            below: 0,
        }
    );
}

#[test]
fn button_below_scroll() {
    assert_eq!(
        tap_button(2, 5, true),
        TestState {
            above: 0,
            inner_1: 0,
            inner_2: 0,
            inner_3: 0,
            below: 1,
        }
    );
}

#[test]
fn tap_outside_scroll() {
    assert_eq!(
        tap_button(2, 7, false),
        TestState {
            above: 0,
            inner_1: 0,
            inner_2: 0,
            inner_3: 0,
            below: 0,
        }
    );

    assert_eq!(
        tap_button(2, 28, false),
        TestState {
            above: 0,
            inner_1: 0,
            inner_2: 0,
            inner_3: 0,
            below: 0,
        }
    );

    assert_eq!(
        tap_button(-2, 5, false),
        TestState {
            above: 0,
            inner_1: 0,
            inner_2: 0,
            inner_3: 0,
            below: 0,
        }
    );
}

fn tap_button(x: i32, y: i32, should_recompute: bool) -> TestState {
    let mut buffer = FixedTextBuffer::<4, 7>::default();
    let size = buffer.size();

    let mut captures = TestState::default();
    let view = scroll_view();
    let mut state = view.build_state(&mut captures);

    let mut tree = helpers::tree(
        &view,
        &mut captures,
        &mut state,
        Duration::from_secs(1),
        size,
    );

    tree.render(&mut buffer, &' ');

    assert_str_grid_eq!(
        ["aaaa", "aaaa", "bbbb", "bbbb", "cccc", "eeee", "eeee",],
        &buffer.text
    );

    let result = view.handle_event(
        &touch_down(x, y),
        &ctx(2),
        &mut tree,
        &mut captures,
        &mut state,
    );
    assert_eq!(result.recompute_view, should_recompute);

    let result = view.handle_event(
        &touch_up(x, y),
        &ctx(3),
        &mut tree,
        &mut captures,
        &mut state,
    );
    assert_eq!(result.recompute_view, should_recompute);

    buffer.clear();

    tree = helpers::tree(
        &view,
        &mut captures,
        &mut state,
        Duration::from_secs(4),
        size,
    );

    tree.render(&mut buffer, &' ');

    assert_str_grid_eq!(
        ["aaaa", "aaaa", "bbbb", "bbbb", "cccc", "eeee", "eeee",],
        &buffer.text
    );

    captures
}
