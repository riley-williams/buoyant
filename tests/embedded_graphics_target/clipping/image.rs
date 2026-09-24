use core::cell::Cell;

use buoyant::{
    primitives::{Interpolate as _, Point},
    view::prelude::*,
};
use embedded_graphics::{
    draw_target::DrawTarget,
    geometry::{OriginDimensions, Point as EgPoint, Size as EgSize},
    image::{Image as EgImage, ImageDrawable, ImageDrawableExt as _, SubImage},
    mock_display::MockDisplay,
    pixelcolor::Rgb888,
    prelude::{DrawTargetExt, Drawable},
    primitives::Rectangle as EgRectangle,
};
use tinytga::Tga;

use crate::embedded_graphics_target::{render_animated_to_mock, render_to_mock};

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

/// A small crop of the test image, so that it can be drawn at an offset
/// without running off the edge of the 64x64 mock display.
fn small_image<'a, 'b>(img: &'a Tga<'b, Rgb888>) -> SubImage<'a, Tga<'b, Rgb888>> {
    img.sub_image(&EgRectangle::new(EgPoint::new(4, 6), EgSize::new(24, 20)))
}

/// An image at `offset` inside a clip rect at (11, 3), sized to reach the
/// bottom trailing corner of the display.
fn image_in_offset_clip<T>(image: &T, offset: Point) -> impl View<Rgb888, ()> + '_
where
    T: ImageDrawable<Color = Rgb888> + OriginDimensions,
{
    Image::new(image)
        .offset(offset.x, offset.y)
        .frame_sized(53, 61)
        .with_alignment(Alignment::TopLeading)
        .clipped()
        .padding(Edges::Leading, 11)
        .padding(Edges::Top, 3)
}

/// The clip rect starts to the left of and above the image but still contains
/// it completely, so the whole image must be drawn.
#[test]
fn clip_rect_offset_from_fully_visible_image_does_not_crop_it() {
    let data = include_bytes!("../assets/rhombic-dodecahedron.tga");
    let img: Tga<Rgb888> = Tga::from_slice(data).unwrap();
    let image = small_image(&img);

    let view = image_in_offset_clip(&image, Point::new(9, 6));

    let display = render_to_mock(&view, false);

    let mut display_2 = MockDisplay::new();
    EgImage::new(&image, EgPoint::new(20, 9))
        .draw(&mut display_2)
        .unwrap();

    display.assert_eq(&display_2);
}

/// The clip rect crops the image on its leading edge only, so the trailing
/// columns of the image must survive.
#[test]
fn clip_rect_crossing_offset_image_keeps_trailing_columns() {
    let data = include_bytes!("../assets/rhombic-dodecahedron.tga");
    let img: Tga<Rgb888> = Tga::from_slice(data).unwrap();
    let image = small_image(&img);

    let view = image_in_offset_clip(&image, Point::new(-8, 6));

    let display = render_to_mock(&view, false);

    let mut display_2 = MockDisplay::new();
    let clip_area = EgRectangle::new(EgPoint::new(11, 3), EgSize::new(53, 61));
    EgImage::new(&image, EgPoint::new(3, 9))
        .draw(&mut display_2.clipped(&clip_area))
        .unwrap();

    display.assert_eq(&display_2);
}

/// An image animating into the clip rect is cropped at its interpolated
/// position, not at either end of the animation.
#[test]
fn animated_image_is_clipped_at_the_interpolated_position() {
    let data = include_bytes!("../assets/rhombic-dodecahedron.tga");
    let img: Tga<Rgb888> = Tga::from_slice(data).unwrap();
    let image = small_image(&img);

    let source_offset = Point::new(-20, 6);
    let target_offset = Point::new(12, 6);
    let factor = 128;

    let display = render_animated_to_mock(
        &image_in_offset_clip(&image, source_offset),
        &image_in_offset_clip(&image, target_offset),
        factor,
    );

    let mut display_2 = MockDisplay::new();
    let clip_origin = Point::new(11, 3);
    let clip_area = EgRectangle::new(clip_origin.into(), EgSize::new(53, 61));
    // The offsets are relative to the clip rect, the animation interpolates the
    // position of the image on the display.
    let position = Point::interpolate(
        clip_origin + source_offset,
        clip_origin + target_offset,
        factor,
    );
    EgImage::new(&image, position.into())
        .draw(&mut display_2.clipped(&clip_area))
        .unwrap();

    display.assert_eq(&display_2);
}

/// Counts of the drawing calls an [`ImageDrawable`] received.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct DrawCalls {
    whole: u32,
    sub: u32,
}

/// An image which records how it was asked to draw itself instead of drawing.
#[derive(Debug)]
struct ProbeImage<'a> {
    size: EgSize,
    calls: &'a Cell<DrawCalls>,
}

impl OriginDimensions for ProbeImage<'_> {
    fn size(&self) -> EgSize {
        self.size
    }
}

impl ImageDrawable for ProbeImage<'_> {
    type Color = Rgb888;

    fn draw<D: DrawTarget<Color = Rgb888>>(&self, _target: &mut D) -> Result<(), D::Error> {
        let mut calls = self.calls.get();
        calls.whole += 1;
        self.calls.set(calls);
        Ok(())
    }

    fn draw_sub_image<D: DrawTarget<Color = Rgb888>>(
        &self,
        _target: &mut D,
        _area: &EgRectangle,
    ) -> Result<(), D::Error> {
        let mut calls = self.calls.get();
        calls.sub += 1;
        self.calls.set(calls);
        Ok(())
    }
}

fn probe_draw_calls(offset: Point) -> DrawCalls {
    let calls = Cell::new(DrawCalls::default());
    let image = ProbeImage {
        size: EgSize::new(24, 20),
        calls: &calls,
    };

    _ = render_to_mock(&image_in_offset_clip(&image, offset), false);

    calls.get()
}

#[test]
fn fully_visible_image_is_drawn_in_one_piece() {
    assert_eq!(
        probe_draw_calls(Point::new(9, 6)),
        DrawCalls { whole: 1, sub: 0 }
    );
}

#[test]
fn partially_visible_image_is_drawn_as_a_sub_image() {
    assert_eq!(
        probe_draw_calls(Point::new(-8, 6)),
        DrawCalls { whole: 0, sub: 1 }
    );
}

#[test]
fn image_outside_the_clip_rect_is_not_drawn() {
    assert_eq!(
        probe_draw_calls(Point::new(-40, 6)),
        DrawCalls { whole: 0, sub: 0 }
    );
}
