use buoyant::{
    primitives::{Point, Size, geometry::Rectangle},
    view::prelude::*,
};

use super::harness::{FocusTestHarness, focused_rect};

struct State {
    a: u32,
    b: u32,
    c: u32,
}

#[test]
fn navigate_forward_and_backward() {
    const B1_RECT: Rectangle = Rectangle::new(Point::new(0, 0), Size::new(36, 4));
    const B2_RECT: Rectangle = Rectangle::new(Point::new(0, 8), Size::new(36, 4));
    const B3_RECT: Rectangle = Rectangle::new(Point::new(0, 12), Size::new(36, 12));

    fn test_view() -> impl View<(), State> {
        VStack::new((
            VStack::new((
                Button::new(|s: &mut State| s.a += 1, |_| Rectangle),
                Rectangle,
                Button::new(|s: &mut State| s.b += 1, |_| Rectangle),
            )),
            Button::new(|s: &mut State| s.c += 1, |_| Rectangle),
            Rectangle,
        ))
    }

    let state = State { a: 0, b: 0, c: 0 };
    let mut harness = FocusTestHarness::new(test_view(), state, Size::new(36, 36));

    // Obtain initial focus
    assert_eq!(harness.focus_forward(), focused_rect(B1_RECT));

    // Select button 1
    assert_eq!(harness.select(), focused_rect(B1_RECT));
    assert_eq!(harness.state.a, 1);

    // Move focus to button 2
    assert_eq!(harness.next(), focused_rect(B2_RECT));
    assert_eq!(harness.state.a, 1);
    assert_eq!(harness.state.b, 0);

    // Select button 2
    assert_eq!(harness.select(), focused_rect(B2_RECT));
    assert_eq!(harness.state.a, 1);
    assert_eq!(harness.state.b, 1);

    // Move focus to button 3
    assert_eq!(harness.next(), focused_rect(B3_RECT));
    assert_eq!(harness.state.a, 1);
    assert_eq!(harness.state.b, 1);
    assert_eq!(harness.state.c, 0);

    // Select button 3
    assert_eq!(harness.select(), focused_rect(B3_RECT));
    assert_eq!(harness.state.a, 1);
    assert_eq!(harness.state.b, 1);
    assert_eq!(harness.state.c, 1);

    // Move focus back to button 2
    assert_eq!(harness.previous(), focused_rect(B2_RECT));
    assert_eq!(harness.state.a, 1);
    assert_eq!(harness.state.b, 1);
    assert_eq!(harness.state.c, 1);

    // Select button 2
    assert_eq!(harness.select(), focused_rect(B2_RECT));
    assert_eq!(harness.state.a, 1);
    assert_eq!(harness.state.b, 2);
    assert_eq!(harness.state.c, 1);
}

#[test]
fn focus_skips_unfocusable_first_element() {
    const B1_RECT: Rectangle = Rectangle::new(Point::new(0, 4), Size::new(36, 4));

    fn test_view() -> impl View<(), State> {
        VStack::new((
            VStack::new((
                Rectangle, // default first, but doesn't match button mask
                Button::new(|s: &mut State| s.a += 1, |_| Rectangle),
                Button::new(|s: &mut State| s.b += 1, |_| Rectangle),
            )),
            Button::new(|s: &mut State| s.c += 1, |_| Rectangle),
            Rectangle,
        ))
    }

    let state = State { a: 0, b: 0, c: 0 };
    let mut harness = FocusTestHarness::new(test_view(), state, Size::new(36, 36));

    // Obtain initial focus - should skip Rectangle and focus Button A
    assert_eq!(harness.focus_forward(), focused_rect(B1_RECT));
}

#[test]
fn previous_into_container_ending_with_unfocusable() {
    const B1_RECT: Rectangle = Rectangle::new(Point::new(0, 0), Size::new(36, 4));
    const B2_RECT: Rectangle = Rectangle::new(Point::new(0, 4), Size::new(36, 4));
    const B3_RECT: Rectangle = Rectangle::new(Point::new(0, 12), Size::new(36, 12));

    fn test_view() -> impl View<(), State> {
        VStack::new((
            VStack::new((
                Button::new(|s: &mut State| s.a += 1, |_| Rectangle),
                Button::new(|s: &mut State| s.b += 1, |_| Rectangle),
                Rectangle, // Last item is NOT a button
            )),
            Button::new(|s: &mut State| s.c += 1, |_| Rectangle),
            Rectangle,
        ))
    }

    let state = State { a: 0, b: 0, c: 0 };
    let mut harness = FocusTestHarness::new(test_view(), state, Size::new(36, 36));

    // Focus button A
    assert_eq!(harness.focus_forward(), focused_rect(B1_RECT));

    // Move to button B
    assert_eq!(harness.next(), focused_rect(B2_RECT));

    // Move to button C (exits inner VStack)
    assert_eq!(harness.next(), focused_rect(B3_RECT));

    // Back to B, traversing backwards
    assert_eq!(harness.previous(), focused_rect(B2_RECT));
}

#[test]
fn hstack_navigate_forward_and_backward() {
    const B1_RECT: Rectangle = Rectangle::new(Point::new(0, 0), Size::new(4, 36));
    const B2_RECT: Rectangle = Rectangle::new(Point::new(8, 0), Size::new(4, 36));
    const B3_RECT: Rectangle = Rectangle::new(Point::new(12, 0), Size::new(12, 36));

    fn test_view() -> impl View<(), State> {
        HStack::new((
            HStack::new((
                Button::new(|s: &mut State| s.a += 1, |_| Rectangle),
                Rectangle,
                Button::new(|s: &mut State| s.b += 1, |_| Rectangle),
            )),
            Button::new(|s: &mut State| s.c += 1, |_| Rectangle),
            Rectangle,
        ))
    }

    let state = State { a: 0, b: 0, c: 0 };
    let mut harness = FocusTestHarness::new(test_view(), state, Size::new(36, 36));

    // Obtain initial focus
    assert_eq!(harness.focus_forward(), focused_rect(B1_RECT));

    // Select button 1
    assert_eq!(harness.select(), focused_rect(B1_RECT));
    assert_eq!(harness.state.a, 1);

    // Move focus to button 2
    assert_eq!(harness.next(), focused_rect(B2_RECT));
    assert_eq!(harness.state.a, 1);
    assert_eq!(harness.state.b, 0);

    // Select button 2
    assert_eq!(harness.select(), focused_rect(B2_RECT));
    assert_eq!(harness.state.a, 1);
    assert_eq!(harness.state.b, 1);

    // Move focus to button 3
    assert_eq!(harness.next(), focused_rect(B3_RECT));
    assert_eq!(harness.state.a, 1);
    assert_eq!(harness.state.b, 1);
    assert_eq!(harness.state.c, 0);

    // Select button 3
    assert_eq!(harness.select(), focused_rect(B3_RECT));
    assert_eq!(harness.state.a, 1);
    assert_eq!(harness.state.b, 1);
    assert_eq!(harness.state.c, 1);

    // Move focus back to button 2
    assert_eq!(harness.previous(), focused_rect(B2_RECT));
    assert_eq!(harness.state.a, 1);
    assert_eq!(harness.state.b, 1);
    assert_eq!(harness.state.c, 1);

    // Select button 2
    assert_eq!(harness.select(), focused_rect(B2_RECT));
    assert_eq!(harness.state.a, 1);
    assert_eq!(harness.state.b, 2);
    assert_eq!(harness.state.c, 1);
}

#[test]
fn hstack_focus_skips_unfocusable_first_element() {
    const B1_RECT: Rectangle = Rectangle::new(Point::new(4, 0), Size::new(4, 36));

    fn test_view() -> impl View<(), State> {
        HStack::new((
            HStack::new((
                Rectangle, // default first, but doesn't match button mask
                Button::new(|s: &mut State| s.a += 1, |_| Rectangle),
                Button::new(|s: &mut State| s.b += 1, |_| Rectangle),
            )),
            Button::new(|s: &mut State| s.c += 1, |_| Rectangle),
            Rectangle,
        ))
    }

    let state = State { a: 0, b: 0, c: 0 };
    let mut harness = FocusTestHarness::new(test_view(), state, Size::new(36, 36));

    // Obtain initial focus - should skip Rectangle and focus Button A
    assert_eq!(harness.focus_forward(), focused_rect(B1_RECT));
}

#[test]
fn hstack_previous_into_container_ending_with_unfocusable() {
    const B1_RECT: Rectangle = Rectangle::new(Point::new(0, 0), Size::new(4, 36));
    const B2_RECT: Rectangle = Rectangle::new(Point::new(4, 0), Size::new(4, 36));
    const B3_RECT: Rectangle = Rectangle::new(Point::new(12, 0), Size::new(12, 36));

    fn test_view() -> impl View<(), State> {
        HStack::new((
            HStack::new((
                Button::new(|s: &mut State| s.a += 1, |_| Rectangle),
                Button::new(|s: &mut State| s.b += 1, |_| Rectangle),
                Rectangle, // Last item is NOT a button
            )),
            Button::new(|s: &mut State| s.c += 1, |_| Rectangle),
            Rectangle,
        ))
    }

    let state = State { a: 0, b: 0, c: 0 };
    let mut harness = FocusTestHarness::new(test_view(), state, Size::new(36, 36));

    // Focus button A
    assert_eq!(harness.focus_forward(), focused_rect(B1_RECT));

    // Move to button B
    assert_eq!(harness.next(), focused_rect(B2_RECT));

    // Move to button C (exits inner HStack)
    assert_eq!(harness.next(), focused_rect(B3_RECT));

    // Back to B, traversing backwards
    assert_eq!(harness.previous(), focused_rect(B2_RECT));
}
