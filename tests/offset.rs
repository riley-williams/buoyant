use buoyant::{primitives::Size, view::prelude::*};

mod common;
use crate::common::tap;

#[test]
fn event_is_offset() {
    let view = Button::new(|x: &mut u32| *x += 1, |_| Rectangle).offset(3, 3);
    let mut x = 0;
    let mut state = view.build_state(&mut x);
    let size = Size::new(3, 3);

    tap(&view, &mut x, &mut state, size, 0, 0);
    assert_eq!(x, 0);

    tap(&view, &mut x, &mut state, size, 1, 1);
    assert_eq!(x, 0);

    tap(&view, &mut x, &mut state, size, 3, 3);
    assert_eq!(x, 1);

    tap(&view, &mut x, &mut state, size, 6, 3);
    assert_eq!(x, 1);
}
