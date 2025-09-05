# Interactivity

Take a look at this simple view with buttons to increment and decrement a counter:

```rust
# extern crate buoyant;
# extern crate embedded_graphics;
# use buoyant::view::prelude::*;
# use embedded_graphics::pixelcolor::{Rgb888, RgbColor};
# use embedded_graphics::mono_font::ascii::FONT_9X15;
#
fn counter_view(count: i32) -> impl View<Rgb888, i32> {
    VStack::new((
        Text::new_fmt::<24>(
            format_args!("count: {count}"),
            &FONT_9X15,
        ),
        Button::new(
            |count: &mut i32| { *count += 1; },
            |_| Text::new("Increment", &FONT_9X15),
        ),
        Button::new(
            |count: &mut i32| { *count -= 1; },
            |_| Text::new("Decrement", &FONT_9X15),
        ),
    ))
}
```

> What's going on? `count` is moved into `counter_view`, and then the buttons have closures
> which can somehow also mutate `count`? Where's the dark magic macro hiding?
>
> – definitely not me, a Rust beginner first seeing this pattern in Xilem

Don't worry, no unholy sacrifices to the borrow checker have been made. The variable
name `count` is just being reused in two separate contexts:

- **View Instantiation:** Data is passed in the function parameters to construct the view.
This data is (generally) going to be immutable. Nothing special here, just passing data
to functions.

- **Event Handling:** The second generic parameter of `View<Color, XYZ>` means event
handlers in this view are promised an `&mut XYZ`. This type can of course be
different from the view function parameters, and is not available when constructing
the view.

This can also be written in a way that's hopefully more clear:

```rust
# extern crate buoyant;
# extern crate embedded_graphics;
# use buoyant::view::prelude::*;
# use embedded_graphics::pixelcolor::{Rgb888, RgbColor};
# use embedded_graphics::mono_font::ascii::FONT_9X15;
#
fn counter_view(count_readonly: i32) -> impl View<Rgb888, i32> {
    VStack::new((
        Text::new_fmt::<24>(
            format_args!("count: {count_readonly}"),
            &FONT_9X15,
        ),
        Button::new(
            |count_mut: &mut i32| { *count_mut += 1; },
            |_| Text::new("Increment", &FONT_9X15),
        ),
        Button::new(
            |count_mut: &mut i32| { *count_mut -= 1; },
            |_| Text::new("Decrement", &FONT_9X15),
        ),
    ))
}
```

The read-only `count` is passed when the view is constructed, and the mutable
reference to count is passed when handling events.
