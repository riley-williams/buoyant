use buoyant::layout::HorizontalAlignment;
use buoyant::primitives::Size;
use buoyant::render::Render;
use buoyant::render_target::{EmbeddedGraphicsRenderTarget, RenderTarget as _};
use buoyant::view::shape::Circle;
use buoyant::view::View;
use buoyant::view::{HStack, Spacer, ViewExt as _};
use buoyant::{
    environment::DefaultEnvironment,
    layout::Layout as _,
    primitives::Point,
    render::Renderable as _,
    view::{shape::Rectangle, Text, VStack},
};
use criterion::{criterion_group, criterion_main, Criterion};
use embedded_graphics::geometry::OriginDimensions;
use embedded_graphics::prelude::{RgbColor, WebColors as _};
use embedded_graphics::{mono_font::iso_8859_4::FONT_6X12, pixelcolor::Rgb888};

criterion_group!(benches, bench_spacings, pipeline, layout, make_tree, render);

criterion_main!(benches);

fn bench_spacings(c: &mut Criterion) {
    let align_hstack = align_hstack();
    let align_frame = align_frame();
    let env = DefaultEnvironment::default();

    let mut group = c.benchmark_group("spacings");
    let size = Size::new(100, 100).into();
    group.bench_function("hstack", |b| {
        b.iter(|| {
            std::hint::black_box(align_hstack.layout(&size, &env));
        });
    });
    group.bench_function("frame", |b| {
        b.iter(|| {
            std::hint::black_box(align_frame.layout(&size, &env));
        });
    });
}

fn align_hstack() -> impl View<Rgb888> {
    VStack::new((
        Circle.foreground_color(Rgb888::CSS_CORAL),
        HStack::new((
            Circle.foreground_color(Rgb888::CSS_DARK_ORCHID),
            Spacer::default(),
        )),
        Circle.foreground_color(Rgb888::CSS_GOLDENROD),
    ))
    .with_alignment(HorizontalAlignment::Trailing)
    .with_spacing(10)
}

fn align_frame() -> impl View<Rgb888> {
    VStack::new((
        Circle.foreground_color(Rgb888::CSS_CORAL),
        Circle
            .foreground_color(Rgb888::CSS_DARK_ORCHID)
            .flex_frame()
            .with_infinite_max_width()
            .with_horizontal_alignment(HorizontalAlignment::Leading),
        Circle.foreground_color(Rgb888::CSS_GOLDENROD),
    ))
    .with_alignment(HorizontalAlignment::Trailing)
    .with_spacing(10)
}

#[expect(clippy::unit_arg)]
pub fn pipeline(c: &mut Criterion) {
    c.bench_function("pipeline", |b| {
        b.iter(|| {
            std::hint::black_box(render_to_mock());
        });
    });
}

pub fn layout(c: &mut Criterion) {
    let display = embedded_graphics::mock_display::MockDisplay::<Rgb888>::new();
    let view = view();
    let env = DefaultEnvironment::default();
    c.bench_function("layout", |b| {
        b.iter(|| {
            std::hint::black_box(view.layout(&display.size().into(), &env));
        });
    });
}

pub fn make_tree(c: &mut Criterion) {
    let display = embedded_graphics::mock_display::MockDisplay::<Rgb888>::new();
    let view = view();
    let env = DefaultEnvironment::default();
    let layout = view.layout(&display.size().into(), &env);

    c.bench_function("tree", |b| {
        b.iter(|| {
            std::hint::black_box(view.render_tree(&layout, Point::zero(), &env));
        });
    });
}

#[expect(clippy::unit_arg)]
pub fn render(c: &mut Criterion) {
    let mut display = embedded_graphics::mock_display::MockDisplay::new();
    display.set_allow_overdraw(true);
    let mut target = EmbeddedGraphicsRenderTarget::new(display);
    let view = view();
    let env = DefaultEnvironment::default();
    let layout = view.layout(&target.size().into(), &env);
    let tree = view.render_tree(&layout, Point::zero(), &env);

    c.bench_function("render", |b| {
        b.iter(|| {
            std::hint::black_box(tree.render(&mut target, &Rgb888::BLACK, Point::zero()));
            target.clear(Rgb888::BLACK);
        });
    });
}

fn render_to_mock() {
    let display = embedded_graphics::mock_display::MockDisplay::new();
    let mut target = EmbeddedGraphicsRenderTarget::new(display);
    let view = view();
    let env = DefaultEnvironment::default();
    let layout = view.layout(&target.size().into(), &env);
    let tree = view.render_tree(&layout, Point::zero(), &env);
    tree.render(&mut target, &Rgb888::BLACK, Point::zero());
}

fn view() -> impl View<Rgb888> {
    VStack::new((Text::new("Hello", &FONT_6X12), Rectangle))
}
