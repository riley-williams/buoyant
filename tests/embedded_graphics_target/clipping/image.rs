use buoyant::view::prelude::*;
use embedded_graphics::{
    geometry::{Point as EgPoint, Size as EgSize},
    image::Image as EgImage,
    mock_display::MockDisplay,
    pixelcolor::Rgb888,
    prelude::{DrawTargetExt, Drawable},
    primitives::Rectangle as EgRectangle,
};
use tinytga::Tga;

use crate::embedded_graphics_target::render_to_mock;

#[test]
fn clipped_to_exact_bounds() {
    let data = include_bytes!("../assets/rhombic-dodecahedron.tga");
    let img: Tga<Rgb888> = Tga::from_slice(data).unwrap();

    let view = Image::new(&img).clipped();

    let display = render_to_mock(&view, false);

    let mut display_2 = MockDisplay::new();
    EgImage::new(&img, EgPoint::zero())
        .draw(&mut display_2)
        .unwrap();

    display.assert_eq(&display_2);
}

#[test]
fn clip_overlaps_partially_diagonal() {
    let data = include_bytes!("../assets/rhombic-dodecahedron.tga");
    let img: Tga<Rgb888> = Tga::from_slice(data).unwrap();

    let view = Image::new(&img)
        .offset(20, 20)
        .frame_sized(60, 60)
        .with_alignment(Alignment::TopLeading)
        .clipped();

    let display = render_to_mock(&view, false);

    let mut display_2 = MockDisplay::new();
    let clip_area = EgRectangle::new(EgPoint::new(0, 0), EgSize::new(60, 60));
    EgImage::new(&img, EgPoint::new(20, 20))
        .draw(&mut display_2.clipped(&clip_area))
        .unwrap();

    display.assert_eq(&display_2);
}

#[test]
fn clip_rect_inside_view_bounds() {
    let data = include_bytes!("../assets/rhombic-dodecahedron.tga");
    let img: Tga<Rgb888> = Tga::from_slice(data).unwrap();

    let view = Image::new(&img)
        .frame_sized(32, 32)
        .with_alignment(Alignment::TopLeading)
        .clipped();

    let display = render_to_mock(&view, false);

    let mut display_2 = MockDisplay::new();

    let clip_area = EgRectangle::new(EgPoint::new(0, 0), EgSize::new(32, 32));
    EgImage::new(&img, EgPoint::zero())
        .draw(&mut display_2.clipped(&clip_area))
        .unwrap();

    display.assert_eq(&display_2);
}

#[test]
fn view_outside_clip_area_not_drawn() {
    let data = include_bytes!("../assets/rhombic-dodecahedron.tga");
    let img: Tga<Rgb888> = Tga::from_slice(data).unwrap();

    let view = Image::new(&img)
        .offset(0, -64) // Offset completely above the clip region
        .frame_sized(64, 64)
        .clipped();

    let display = render_to_mock(&view, false);

    let display_2 = MockDisplay::<Rgb888>::new();

    display.assert_eq(&display_2);
}
