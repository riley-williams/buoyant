use buoyant::{
    environment::DefaultEnvironment,
    layout::Alignment,
    primitives::{Point, Size},
    render::Render,
    render_target::{EmbeddedGraphicsRenderTarget, RenderTarget as _},
    surface::AsDrawTarget,
    view::{
        padding::Edges,
        shape::{Capsule, Circle, Rectangle, RoundedRectangle},
        AsDrawable as _, HStack, Image, Text, VStack, View, ViewExt,
    },
};
use embedded_graphics::{
    geometry::{OriginDimensions, Point as EgPoint, Size as EgSize},
    image::{Image as EgImage, ImageDrawableExt},
    mock_display::MockDisplay,
    mono_font::{ascii::FONT_7X13, MonoTextStyle},
    pixelcolor::Rgb888,
    prelude::{DrawTargetExt, Drawable, Primitive, RgbColor, WebColors},
    primitives::{PrimitiveStyleBuilder, Rectangle as EgRectangle},
    text::Text as EgText,
};
use tinytga::Tga;
use u8g2_fonts::{fonts, types::FontColor, FontRenderer};

const MOCK_SIZE: u32 = 64;

fn render_to_mock(view: &impl View<Rgb888>, allow_overdraw: bool) -> MockDisplay<Rgb888> {
    let mut display = MockDisplay::<Rgb888>::new();
    display.set_allow_overdraw(allow_overdraw);
    display.set_allow_out_of_bounds_drawing(false);
    let mut target = EmbeddedGraphicsRenderTarget::new(&mut display);

    let env = DefaultEnvironment::default();
    let layout = view.layout(&target.size().into(), &env);
    let tree = view.render_tree(&layout, Point::zero(), &env);
    tree.render(&mut target, &Rgb888::WHITE, Point::zero());

    display
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

    let display = render_to_mock(&view, false);

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

    let display = render_to_mock(&view, false);

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
fn raw_surface_draw_target() {
    let mut display = MockDisplay::<Rgb888>::new();
    display.set_allow_overdraw(false);
    display.set_allow_out_of_bounds_drawing(false);

    let rectangle = EgRectangle::new(EgPoint::new(4, 4), display.size() - EgSize::new(8, 8))
        .into_styled(
            PrimitiveStyleBuilder::new()
                .fill_color(Rgb888::CSS_SPRING_GREEN)
                .build(),
        );

    let mut target = EmbeddedGraphicsRenderTarget::new(&mut display);
    // target as surface -> surface as target
    rectangle
        .draw(&mut target.raw_surface().draw_target())
        .unwrap();

    let mut display_2 = MockDisplay::new();
    rectangle.draw(&mut display_2).unwrap();

    target.display().assert_eq(&display_2);
}

#[test]
fn as_drawable() {
    let mut display = MockDisplay::new();

    Rectangle
        .padding(Edges::All, 4)
        .foreground_color(Rgb888::CSS_SPRING_GREEN)
        .as_drawable(display.size(), Rgb888::WHITE)
        .draw(&mut display)
        .unwrap();

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

    let display = render_to_mock(&view, false);

    let mut display_2 = MockDisplay::new();
    let style = MonoTextStyle::new(&FONT_7X13, Rgb888::CSS_OLD_LACE);
    EgText::new("Test.\n12 3", EgPoint::new(0, 10), style)
        .draw(&mut display_2)
        .unwrap();
    display.assert_eq(&display_2);
}

#[test]
fn u8g2_font() {
    let text = "Test.\n12 3";
    let font = FontRenderer::new::<fonts::u8g2_font_haxrcorp4089_t_cyrillic>();
    let view = Text::new(text, &font)
        .foreground_color(Rgb888::CSS_SPRING_GREEN)
        .padding(Edges::All, 1);

    let display = render_to_mock(&view, true);

    let mut display_2 = MockDisplay::new();
    display_2.set_allow_overdraw(true);
    font.render(
        text,
        Point::new(1, 1).into(),
        u8g2_fonts::types::VerticalPosition::Top,
        FontColor::Transparent(Rgb888::CSS_SPRING_GREEN),
        &mut display_2,
    )
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

    let display = render_to_mock(&view, false);

    let mut display_2 = MockDisplay::new();
    EgImage::new(&img, EgPoint::zero())
        .draw(&mut display_2)
        .unwrap();
    display.assert_eq(&display_2);
}

#[test]
fn embedded_graphics_offset_image_slice() {
    // Include an image from a local path as bytes
    let data = include_bytes!("./assets/rhombic-dodecahedron.tga");

    // Create a TGA instance from a byte slice.
    // The color type is set by defining the type of the `img` variable.
    let binding = Tga::from_slice(data).unwrap();
    let img = binding.sub_image(
        &buoyant::primitives::geometry::Rectangle::new(Point::new(5, 5), Size::new(25, 25)).into(),
    );

    let view = Image::new(&img).padding(Edges::Leading, 1);

    let display = render_to_mock(&view, false);

    let mut display_2 = MockDisplay::new();
    EgImage::new(&img, EgPoint::zero())
        .draw(&mut display_2.translated(Point::new(1, 0).into()))
        .unwrap();
    display.assert_eq(&display_2);
}
