use buoyant::{
    primitives::{Point, Size},
    view::prelude::*,
};
use embedded_graphics::{
    geometry::Point as EgPoint,
    image::{Image as EgImage, ImageDrawableExt},
    mock_display::MockDisplay,
    pixelcolor::Rgb888,
    prelude::{DrawTargetExt, Drawable},
};
use tinytga::Tga;

use super::render_to_mock;

#[test]
fn embedded_graphics_image() {
    // Include an image from a local path as bytes
    let data = include_bytes!("assets/rhombic-dodecahedron.tga");

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
    let data = include_bytes!("assets/rhombic-dodecahedron.tga");

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
