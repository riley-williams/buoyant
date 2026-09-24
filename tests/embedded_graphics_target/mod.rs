mod clipping;
mod images;
mod misc;
mod shapes;
mod shapes_transform;
mod text;

use core::time::Duration;

use buoyant::{
    environment::DefaultEnvironment,
    primitives::Point,
    render::{AnimationDomain, Render},
    render_target::{EmbeddedGraphicsRenderTarget, RenderTarget as _},
    view::prelude::*,
};
use embedded_graphics::{mock_display::MockDisplay, pixelcolor::Rgb888, prelude::RgbColor};

use crate::common::make_render_tree;

pub fn render_to_mock(view: &impl View<Rgb888, ()>, allow_overdraw: bool) -> MockDisplay<Rgb888> {
    let mut display = MockDisplay::<Rgb888>::new();
    display.set_allow_overdraw(allow_overdraw);
    display.set_allow_out_of_bounds_drawing(false);
    let mut target = EmbeddedGraphicsRenderTarget::new_hinted(&mut display, Rgb888::BLACK);

    let env = DefaultEnvironment::default();
    let mut state = view.build_state(&mut ());
    let layout = view.layout(&target.size().into(), &env, &mut (), &mut state);
    let tree = view.render_tree(&layout.sublayouts, Point::zero(), &env, &mut (), &mut state);
    tree.render(&mut target, &Rgb888::WHITE);

    display
}

/// Renders the animation between `source` and `target` at `factor`.
pub fn render_animated_to_mock<V: View<Rgb888, ()>>(
    source: &V,
    target: &V,
    factor: u8,
) -> MockDisplay<Rgb888> {
    let mut display = MockDisplay::<Rgb888>::new();
    display.set_allow_out_of_bounds_drawing(false);
    let mut render_target = EmbeddedGraphicsRenderTarget::new_hinted(&mut display, Rgb888::BLACK);

    let size = render_target.size();
    let source_tree = make_render_tree(source, size, &mut ());
    let target_tree = make_render_tree(target, size, &mut ());

    Render::render_animated(
        &mut render_target,
        &source_tree,
        &target_tree,
        &Rgb888::WHITE,
        &AnimationDomain::new(factor, Duration::from_millis(100)),
    );

    display
}
