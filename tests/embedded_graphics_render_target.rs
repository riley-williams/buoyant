use buoyant::{
    environment::DefaultEnvironment,
    layout::Alignment,
    primitives::Point,
    render::Render,
    render_target::EmbeddedGraphicsRenderTarget,
    view::{
        padding::Edges,
        shape::{Capsule, Circle, Rectangle, RoundedRectangle},
        HStack, Image, Text, VStack, View, ViewExt,
    },
};
use embedded_graphics::{
    geometry::{OriginDimensions, Point as EgPoint, Size as EgSize},
    image::Image as EgImage,
    mono_font::{ascii::FONT_7X13, MonoTextStyle},
    prelude::{Drawable, Primitive, RgbColor, WebColors},
    primitives::{PrimitiveStyleBuilder, Rectangle as EgRectangle},
    text::Text as EgText,
};
use embedded_graphics::{mock_display::MockDisplay, pixelcolor::Rgb888};
use tinytga::Tga;

const MOCK_SIZE: u32 = 64;

fn render_to_mock(view: &impl View<Rgb888>) -> MockDisplay<Rgb888> {
    let mut display = MockDisplay::<Rgb888>::new();
    display.set_allow_overdraw(false);
    display.set_allow_out_of_bounds_drawing(false);
    let mut target = EmbeddedGraphicsRenderTarget::new(display);

    let env = DefaultEnvironment::default();
    let layout = view.layout(&target.display.size().into(), &env);
    let tree = view.render_tree(&layout, Point::zero(), &env);
    tree.render(&mut target, &Rgb888::WHITE, Point::zero());

    target.display
}

#[test]
fn sanity_shapes() {
    // This should write to every pixel at most once
    let view = HStack::new((
        VStack::new((
            Circle,
            Rectangle.geometry_group(),
            RoundedRectangle::new(5),
            Rectangle,
        )),
        VStack::new((Rectangle, Capsule)).geometry_group(),
    ))
    .frame_sized(MOCK_SIZE - 5, MOCK_SIZE - 6)
    .flex_frame()
    .with_infinite_max_width()
    .with_infinite_max_height()
    .with_alignment(Alignment::BottomTrailing);

    let display = render_to_mock(&view);

    assert_eq!(
        display.affected_area(),
        EgRectangle::new(EgPoint::new(5, 6), display.size() - EgSize::new(5, 6))
    );
}

#[test]
fn rectangle() {
    let view = Rectangle
        .padding(Edges::All, 4)
        .foreground_color(Rgb888::CSS_SPRING_GREEN);

    let display = render_to_mock(&view);

    let mut display_2 = MockDisplay::new();

    EgRectangle::new(EgPoint::new(4, 4), display.size() - EgSize::new(8, 8))
        .into_styled(
            PrimitiveStyleBuilder::new()
                .fill_color(Rgb888::CSS_SPRING_GREEN)
                .build(),
        )
        .draw(&mut display_2)
        .unwrap();
    display.assert_eq(&display_2);
}

#[test]
fn embedded_graphics_mono_font() {
    let view = Text::new("Test.\n12 3", &FONT_7X13).foreground_color(Rgb888::CSS_OLD_LACE);

    let display = render_to_mock(&view);

    let mut display_2 = MockDisplay::new();
    let style = MonoTextStyle::new(&FONT_7X13, Rgb888::CSS_OLD_LACE);
    EgText::new("Test.\n12 3", EgPoint::new(0, 10), style)
        .draw(&mut display_2)
        .unwrap();
    display.assert_eq(&display_2);
}

#[test]
fn embedded_graphics_image() {
    // Include an image from a local path as bytes
    let data = include_bytes!("./assets/rhombic-dodecahedron.tga");

    // Create a TGA instance from a byte slice.
    // The color type is set by defining the type of the `img` variable.
    let img: Tga<Rgb888> = Tga::from_slice(data).unwrap();

    let view = Image::new(&img);

    let display = render_to_mock(&view);

    let mut display_2 = MockDisplay::new();
    EgImage::new(&img, EgPoint::zero())
        .draw(&mut display_2)
        .unwrap();
    display.assert_eq(&display_2);
}
