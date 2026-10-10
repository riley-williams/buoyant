use core::cell::Cell;

use buoyant::{
    primitives::{Interpolate as _, Point},
    render::{AnimationDomain, Render},
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

use super::super::images::{BACKGROUND, FOREGROUND, GRADIENT_TGA, TEMPLATE_RAMP};
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
struct ProbeImage<'a, C> {
    size: EgSize,
    calls: &'a Cell<DrawCalls>,
    color: core::marker::PhantomData<C>,
}

impl<C> OriginDimensions for ProbeImage<'_, C> {
    fn size(&self) -> EgSize {
        self.size
    }
}

impl<C: embedded_graphics::pixelcolor::PixelColor> ImageDrawable for ProbeImage<'_, C> {
    type Color = C;

    fn draw<D: DrawTarget<Color = C>>(&self, _target: &mut D) -> Result<(), D::Error> {
        self.calls.update(|mut calls| {
            calls.whole += 1;
            calls
        });
        Ok(())
    }

    fn draw_sub_image<D: DrawTarget<Color = C>>(
        &self,
        _target: &mut D,
        _area: &EgRectangle,
    ) -> Result<(), D::Error> {
        self.calls.update(|mut calls| {
            calls.sub += 1;
            calls
        });
        Ok(())
    }
}

#[test]
fn image_visibility_selects_whole_sub_image_or_no_drawing() {
    use buoyant::{
        primitives::{Size, geometry::Rectangle},
        render::{
            self, Render as _,
            image::{Original, Template},
        },
        render_target::EmbeddedGraphicsRenderTarget,
    };
    use embedded_graphics::pixelcolor::Gray8;

    for (offset, expected) in [
        (Point::new(9, 6), DrawCalls { whole: 1, sub: 0 }),
        (Point::new(-8, 6), DrawCalls { whole: 0, sub: 1 }),
        (Point::new(-40, 6), DrawCalls { whole: 0, sub: 0 }),
    ] {
        for templated in [false, true] {
            let calls = Cell::new(DrawCalls::default());
            let mut display = MockDisplay::<Rgb888>::new();
            let mut target = EmbeddedGraphicsRenderTarget::new(&mut display)
                .with_offset(Point::new(11, 3))
                .with_clip(&Rectangle::new(Point::new(11, 3), Size::new(53, 61)));
            if templated {
                let image = ProbeImage::<Gray8> {
                    size: EgSize::new(24, 20),
                    calls: &calls,
                    color: core::marker::PhantomData,
                };
                render::Image::<_, Template>::new(offset, &image)
                    .render(&mut target, &Rgb888::new(255, 255, 255));
            } else {
                let image = ProbeImage::<Rgb888> {
                    size: EgSize::new(24, 20),
                    calls: &calls,
                    color: core::marker::PhantomData,
                };
                render::Image::<_, Original>::new(offset, &image)
                    .render(&mut target, &Rgb888::new(255, 255, 255));
            }
            assert_eq!(
                calls.get(),
                expected,
                "offset={offset:?}, templated={templated}"
            );
        }
    }
}

fn draw_static_or_animated<R: Render<Rgb888>>(
    target: &mut impl buoyant::render_target::RenderTarget<ColorFormat = Rgb888>,
    source: &R,
    image: &R,
    factor: Option<u8>,
) {
    if let Some(factor) = factor {
        R::render_animated(
            target,
            source,
            image,
            &FOREGROUND,
            &AnimationDomain::new(factor, core::time::Duration::from_millis(100)),
        );
    } else {
        image.render(target, &FOREGROUND);
    }
}

#[test]
fn gradient_clipping_tracks_image_layer_and_target_offsets() {
    use buoyant::{
        primitives::{Size, geometry::Rectangle},
        render::image::{Original, Template},
        render::{self},
        render_target::{EmbeddedGraphicsRenderTarget, RenderTarget as _},
    };
    use embedded_graphics::{Pixel, pixelcolor::Gray8};

    let original = Tga::<Rgb888>::from_slice(GRADIENT_TGA).unwrap();
    let template = Tga::<Gray8>::from_slice(GRADIENT_TGA).unwrap();
    let layer_offset = Point::new(3, -2);
    // Full, exact bounds, each pair of edges, an interior crop, touching but
    // outside, and an empty clip. Coordinates below are physical display pixels.
    for (position, clip_origin, clip_size) in [
        ((9, 7), (8, 6), (7, 5)),
        ((8, 6), (8, 6), (5, 3)),
        ((6, 5), (8, 6), (5, 3)),
        ((10, 7), (8, 6), (5, 3)),
        ((6, 7), (8, 6), (5, 3)),
        ((10, 5), (8, 6), (5, 3)),
        ((7, 5), (8, 6), (2, 1)),
        ((3, 6), (8, 6), (5, 3)),
        ((13, 6), (8, 6), (5, 3)),
        ((8, 6), (8, 6), (0, 0)),
        ((-2, -1), (0, 0), (64, 64)),
        ((62, 63), (0, 0), (64, 64)),
    ] {
        let clip = Rectangle::new(
            Point::new(clip_origin.0, clip_origin.1),
            Size::new(clip_size.0, clip_size.1),
        );
        for target_offset in [Point::zero(), Point::new(7, 5), Point::new(-3, -2)] {
            let origin = Point::new(position.0, position.1) - target_offset - layer_offset;
            let source_origin = origin - Point::new(4, 2);
            for factor in [None, Some(0), Some(128), Some(255)] {
                let rendered_origin = factor.map_or(origin, |factor| {
                    Point::interpolate(source_origin, origin, factor)
                }) + target_offset
                    + layer_offset;
                for templated in [false, true] {
                    let mut display = MockDisplay::new();
                    let mut target =
                        EmbeddedGraphicsRenderTarget::new_hinted(&mut display, BACKGROUND)
                            .with_offset(target_offset)
                            .with_clip(&clip);
                    target.with_layer(
                        |layer| layer.offset(layer_offset),
                        |target| {
                            if templated {
                                draw_static_or_animated(
                                    target,
                                    &render::Image::<_, Template>::new(source_origin, &template),
                                    &render::Image::<_, Template>::new(origin, &template),
                                    factor,
                                );
                            } else {
                                draw_static_or_animated(
                                    target,
                                    &render::Image::<_, Original>::new(source_origin, &original),
                                    &render::Image::<_, Original>::new(origin, &original),
                                    factor,
                                );
                            }
                        },
                    );

                    let mut expected = MockDisplay::new();
                    for (i, &luma) in GRADIENT_TGA[18..].iter().enumerate() {
                        let point = rendered_origin + Point::new((i % 5) as i32, (i / 5) as i32);
                        if clip.contains(&point) && !(templated && luma == 0) {
                            let color = if templated {
                                TEMPLATE_RAMP[usize::from(luma).div_ceil(64)]
                            } else {
                                Rgb888::new(luma, luma, luma)
                            };
                            expected.draw_iter([Pixel(point.into(), color)]).unwrap();
                        }
                    }
                    assert_eq!(
                        display, expected,
                        "position={position:?}, clip={clip:?}, target_offset={target_offset:?}, factor={factor:?}, templated={templated}"
                    );
                }
            }
        }
    }
}
