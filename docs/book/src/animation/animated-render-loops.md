# Animated Render Loops

## Animating Between Render Trees

To animate rendering between two render trees, use the `render_animated()` method:

```rust
# extern crate buoyant;
# extern crate embedded_graphics;
# use std::time::Duration;
#
# use buoyant::{
#     environment::DefaultEnvironment,
#     primitives::{Point, Size},
#     render::{
#         AnimatedJoin, AnimationDomain, Render,
#     },
#     render_target::EmbeddedGraphicsRenderTarget,
#     view::prelude::*,
# };
# use embedded_graphics::{pixelcolor::Rgb888, prelude::RgbColor};
#
# let mut display = embedded_graphics::mock_display::MockDisplay::new();
# let mut target = EmbeddedGraphicsRenderTarget::new(&mut display);
# let app_time = Duration::from_secs(0);
#
# let mut captures = ();
# let environment = DefaultEnvironment::new(app_time);
# let source_view = view();
# let mut view_state = source_view.build_state(&mut ());
# let source_layout = source_view.layout(&Size::new(200, 100).into(), &environment, &mut captures, &mut view_state);
let source_render_tree = source_view.render_tree(&source_layout, Point::zero(), &environment, &mut captures, &mut view_state);

# let environment = DefaultEnvironment::new(app_time);
# let target_view = view();
# let target_layout = target_view.layout(&Size::new(200, 100).into(), &environment, &mut captures, &mut view_state);
let target_render_tree = target_view.render_tree(&target_layout, Point::zero(), &environment, &mut captures, &mut view_state);

Render::render_animated(
    &mut target,
    &source_render_tree,
    &target_render_tree,
    &Rgb888::BLACK,
    &AnimationDomain::top_level(app_time),
);
#
# /// This is just a tribute to the greatest view in the world.
# fn view() -> impl View<Rgb888, ()> {
#     EmptyView // Couldn't remember
# }
```

## Joining Trees

Generally, all animations in Buoyant are interruptible. In the same way you can animate
rendering between two trees, two trees can be joined at a moment in time to form a new one.
This allows continuously merging and generating new trees to drive smooth compound
animations.

Render tree types conform to `AnimatedJoin`, which allows you to create a joined tree
at a specific point in time. With some exceptions, the result of rendering the joined tree
is the same as rendering the two trees with `render_animated()`.

```rust
# extern crate buoyant;
# extern crate embedded_graphics;
# use std::time::Duration;
#
# use buoyant::{
#     environment::DefaultEnvironment,
#     primitives::{Point, Size},
#     render::{
#         AnimatedJoin, AnimationDomain, Render,
#     },
#     render_target::EmbeddedGraphicsRenderTarget,
#     view::prelude::*,
# };
# use embedded_graphics::{pixelcolor::Rgb888, prelude::RgbColor};
#
# let mut display = embedded_graphics::mock_display::MockDisplay::new();
# let mut target = EmbeddedGraphicsRenderTarget::new(&mut display);
# let app_time = Duration::from_secs(0);
#
# let mut captures = ();
# let environment = DefaultEnvironment::new(app_time);
# let source_view = view();
# let mut view_state = source_view.build_state(&mut ());
# let source_layout = source_view.layout(&Size::new(200, 100).into(), &environment, &mut captures, &mut view_state);
let source_render_tree = source_view.render_tree(&source_layout, Point::zero(), &environment, &mut captures, &mut view_state);

# let environment = DefaultEnvironment::new(app_time);
# let target_view = view();
# let target_layout = target_view.layout(&Size::new(200, 100).into(), &environment, &mut captures, &mut view_state);
let mut target_render_tree = target_view.render_tree(&target_layout, Point::zero(), &environment, &mut captures, &mut view_state);

// Join two trees into the target
target_render_tree.join_from(
    &source_render_tree,
    &AnimationDomain::top_level(app_time),
);

// Calling render on the joined tree produces the same result as
// the render_animated call above
target_render_tree.render(&mut target, &Rgb888::BLACK);
#
# fn view() -> impl View<Rgb888, ()> {
#     EmptyView
# }
```

Joining trees encodes information about the partially completed animation, which allows
multiple staggered animations to occur in a render loop.

Continue to [Event Loops](../interactivity/event-loops.md) for a complete  animated render
loop example with event handling.
