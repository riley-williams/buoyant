# Animated Render Loops

## Animating Between Render Trees

To animate between two render trees, you can use the `render_animated()` method:

```rust
# extern crate buoyant;
# extern crate embedded_graphics;
# use std::time::Duration;
# 
# use buoyant::{
#     environment::DefaultEnvironment,
#     layout::Layout as _,
#     primitives::{Point, Size},
#     render::{
#         AnimatedJoin, AnimationDomain, Render,
#         Renderable as _,
#     },
#     render_target::EmbeddedGraphicsRenderTarget,
#     view::{EmptyView, View},
# };
# use embedded_graphics::{pixelcolor::Rgb888, prelude::RgbColor};
# 
# let mut display = embedded_graphics::mock_display::MockDisplay::new();
# let mut target = EmbeddedGraphicsRenderTarget::new(&mut display);
# let app_time = Duration::from_secs(0);
# 
# let environment = DefaultEnvironment::new(app_time);
# let source_view = view();
# let source_layout = source_view.layout(&Size::new(200, 100).into(), &environment);
let source_render_tree = source_view.render_tree(&source_layout, Point::zero(), &environment);

# let environment = DefaultEnvironment::new(app_time);
# let target_view = view();
# let target_layout = target_view.layout(&Size::new(200, 100).into(), &environment);
let target_render_tree = target_view.render_tree(&target_layout, Point::zero(), &environment);

Render::render_animated(
    &mut target,
    &source_render_tree,
    &target_render_tree,
    &Rgb888::BLACK,
    Point::zero(),
    &AnimationDomain::top_level(app_time),
);
# 
# fn view() -> impl View<Rgb888> {
#     EmptyView
# }
```

## Joining Trees

Generally, all animations in Buoyant are interruptible. In the same way you can animate
rendering between two trees, you can also join two trees to form a new one. This allows
you to continuously merge and generate new trees to create a smooth animated render loop.

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
#     layout::Layout as _,
#     primitives::{Point, Size},
#     render::{
#         AnimatedJoin, AnimationDomain, Render,
#         Renderable as _,
#     },
#     render_target::EmbeddedGraphicsRenderTarget,
#     view::{EmptyView, View},
# };
# use embedded_graphics::{pixelcolor::Rgb888, prelude::RgbColor};
# 
# let mut display = embedded_graphics::mock_display::MockDisplay::new();
# let mut target = EmbeddedGraphicsRenderTarget::new(&mut display);
# let app_time = Duration::from_secs(0);
# 
# let environment = DefaultEnvironment::new(app_time);
# let source_view = view();
# let source_layout = source_view.layout(&Size::new(200, 100).into(), &environment);
let source_render_tree = source_view.render_tree(&source_layout, Point::zero(), &environment);

# let environment = DefaultEnvironment::new(app_time);
# let target_view = view();
# let target_layout = target_view.layout(&Size::new(200, 100).into(), &environment);
let target_render_tree = target_view.render_tree(&target_layout, Point::zero(), &environment);

// Join two trees
let joined_tree = AnimatedJoin::join(
    source_render_tree,
    target_render_tree,
    &AnimationDomain::top_level(app_time),
);

// Calling render on the joined tree produces the same result as
// the render_animated call above
joined_tree.render(&mut target, &Rgb888::BLACK, Point::zero());
# 
# fn view() -> impl View<Rgb888> {
#     EmptyView
# }
```

Joining trees encodes information about the partially completed animation, which allows multiple
staggered animations to occur in a render loop.

## Creating a Render Loop

Buoyant on its own does not track whether state has changed, and you are responsible
for managing the view and render tree lifecycle in response to state changes.

Here's a rough outline of what a that might look like:

```rust,ignore
/// Produce a render tree for a given state, time, and size
fn make_tree(state: &State, time: Duration, size: &Size) -> impl Render<Rgb888> {
    let view = /* ... */;
    let layout = /* ... */;
    view.render_tree(/* ... */)
}

fn main() {
    let mut display = /* ... */;
    let app_start = Instant::now(); // track offset from app start

    let mut state = State::default();

    let mut source_tree = make_tree(&state, app_start.elapsed());
    let mut target_tree = make_tree(&state, app_start.elapsed());

    loop {
        display.clear(Rgb888::BLACK);

        // Render, animating between the source and target trees
        Render::render_animated(
            &mut display,
            &source_tree,
            &target_tree,
            &Rgb888::WHITE,
            Point::zero(),
            &AnimationDomain::top_level(app_start.elapsed()),
        );

        // Update state
        match event {
            Event::ButtonPressed(Button::A) => {
                state.a.toggle();
            }
            /* ... */
        }

        // If state changed, create a new source tree by joining the old source and target.
        // The joined tree is partially animated between the old source and target trees.
        if state.changed() {
            source_tree = AnimatedJoin::join(
                source_tree,
                target_tree,
                &AnimationDomain::top_level(app_start.elapsed()),
            );
            target_tree = make_tree(&state, app_start.elapsed());
        }
    }
}
```

This loop will animate between the source and target trees, creating a new target tree when
the state changes. The source tree is joined with the original target tree to create a new
source tree that continues the animation from where it left off.

## Tracking Changes in State

You may find it useful to leverage the borrow checker to track whether state has changed by
placing mutable state behind a method that sets a dirty flag.

```rust
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Settings {
    pub big_text: bool,
    pub increase_contrast: bool,
    pub reduce_animation: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct State {
    settings: Settings,
    is_dirty: bool,
}

impl State {
    /// Returns a mutable reference to the app's state, and marks the state as dirty.
    pub fn settings_mut(&mut self) -> &mut Settings {
        self.is_dirty = true;
        &mut self.settings
    }

    pub fn settings(&self) -> &Settings {
        &self.settings
    }

    /// Resets the dirty flag and returns its previous value.
    pub fn reset_dirty(&mut self) -> bool {
        let was_dirty = self.is_dirty;
        self.is_dirty = false;
        was_dirty
    }
}
```
