# Transitions

Transitions occur when you have a conditional view like `if_view!` or `match_view!` that
changes branches. Because the branches contain different subtrees, there is no reasonable
way to animate between them.

![Transition](./images/transition.svg)

> The properties of views within unchanged branches are still animated as normal.

Because Buoyant will not animate between branches, you should avoid using conditional
views when all the branches are the same type:

```rust
# extern crate buoyant;
# extern crate embedded_graphics;
# use buoyant::{
#     if_view,
#     render::EmbeddedGraphicsView,
#     view::{shape::Rectangle, LayoutExtensions},
# };
# use embedded_graphics::pixelcolor::Rgb888;
# 
/// This will jump between two different rectangles
fn bar1(is_wide: bool) -> impl EmbeddedGraphicsView<Rgb888> {
    if_view!((is_wide) {
        Rectangle.frame_sized(100, 5)
    } else {
        Rectangle.frame_sized(20, 5)
    })
}

/// This will animate the frame of the Rectangle
fn bar2(is_wide: bool) -> impl EmbeddedGraphicsView<Rgb888> {
    Rectangle.frame_sized(if is_wide { 100 } else { 20 }, 5)
}
```

## Future Work

There is no animation when transitioning between branches today. However, this feature
is planned for a future release and will allow you to animate the motion of the frame
as the branch appears or disappears.
