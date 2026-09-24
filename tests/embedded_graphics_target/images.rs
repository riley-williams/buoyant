use buoyant::{
    environment::DefaultEnvironment,
    event::{EventContext, TouchResult},
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

#[allow(clippy::let_unit_value)]
#[test]
fn image_handles_touch_only_when_started_inside() {
    let data = include_bytes!("assets/rhombic-dodecahedron.tga");
    let img: Tga<Rgb888> = Tga::from_slice(data).unwrap();
    let view = Image::new(&img);
    // The TGA is 64x64; offer a larger frame so touches outside the image are off-image.
    let mut state = view.build_state(&mut ());
    let layout = view.layout(
        &Size::new(100, 100).into(),
        &DefaultEnvironment::default(),
        &mut (),
        &mut state,
    );
    let mut tree = view.render_tree(
        &layout.sublayouts,
        Point::zero(),
        &DefaultEnvironment::default(),
        &mut (),
        &mut state,
    );
    let ctx = EventContext::new(core::time::Duration::ZERO);

    // A touch starting inside the image bounds (0,0)-(64,64) is handled.
    let result = view.handle_touch(
        &embedded_touch::Touch::new(
            0,
            Point::new(1, 1).into(),
            embedded_touch::Phase::Started,
            embedded_touch::Tool::Finger,
        ),
        &ctx,
        &mut tree,
        &mut (),
        &mut state,
    );
    assert_eq!(
        result,
        TouchResult::Handled,
        "touch starting inside image should be handled"
    );

    // A touch starting outside the image bounds is not handled.
    let result = view.handle_touch(
        &embedded_touch::Touch::new(
            0,
            Point::new(80, 80).into(),
            embedded_touch::Phase::Started,
            embedded_touch::Tool::Finger,
        ),
        &ctx,
        &mut tree,
        &mut (),
        &mut state,
    );
    assert_eq!(
        result,
        TouchResult::Deferred,
        "touch starting outside image should defer"
    );

    // A non-Started touch inside the image is not handled.
    let result = view.handle_touch(
        &embedded_touch::Touch::new(
            0,
            Point::new(1, 1).into(),
            embedded_touch::Phase::Moved,
            embedded_touch::Tool::Finger,
        ),
        &ctx,
        &mut tree,
        &mut (),
        &mut state,
    );
    assert_eq!(
        result,
        TouchResult::Deferred,
        "moved touch inside image should defer"
    );
}
