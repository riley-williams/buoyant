mod clipping;
mod images;
mod misc;
mod shapes;
mod shapes_transform;
mod text;

use buoyant::{
    environment::DefaultEnvironment,
    primitives::Point,
    render::Render,
    render_target::{EmbeddedGraphicsRenderTarget, RenderTarget as _},
    view::prelude::*,
};
use embedded_graphics::{mock_display::MockDisplay, pixelcolor::Rgb888, prelude::RgbColor};

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
