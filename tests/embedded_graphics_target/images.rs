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

// Hand-authored, uncompressed 5x3 grayscale TGA, with its origin at the top left.
// The first row is a 0%, ~25%, ~50%, ~75%, 100% ramp. Reversing and shuffling
// the next rows makes row/column clipping and orientation errors easy to spot.
pub(super) const GRADIENT_TGA: &[u8] = &[
    0, 0, 3, // no image ID or color map; grayscale image
    0, 0, 0, 0, 0, // unused color map specification
    0, 0, 0, 0, // origin
    5, 0, 3, 0, 8, 0x20, // width, height, 8 bits per pixel, top-left origin
    0, 64, 128, 192, 255, // ascending ramp
    255, 192, 128, 64, 0, // descending ramp
    64, 192, 255, 0, 128, // shuffled ramp
];

pub(super) const BACKGROUND: Rgb888 = Rgb888::new(20, 40, 60);
pub(super) const FOREGROUND: Rgb888 = Rgb888::new(220, 140, 100);
// The renderer interpolates in 1/256 steps, with an exact endpoint at 255.
pub(super) const TEMPLATE_RAMP: [Rgb888; 5] = [
    BACKGROUND,
    Rgb888::new(70, 65, 70),
    Rgb888::new(120, 90, 80),
    Rgb888::new(170, 115, 90),
    FOREGROUND,
];

fn assert_template_gradient(display: &MockDisplay<Rgb888>, ramp: [Rgb888; 5]) {
    use embedded_graphics::{Pixel, prelude::DrawTarget as _};

    let mut expected = MockDisplay::new();
    for (i, &luma) in GRADIENT_TGA[18..].iter().enumerate() {
        let point = EgPoint::new((i % 5) as i32, (i / 5) as i32);
        // Black pixels must remain untouched even when a background hint exists.
        if luma != 0 {
            expected
                .draw_iter([Pixel(point, ramp[usize::from(luma).div_ceil(64)])])
                .unwrap();
        }
        assert_eq!(
            display.get_pixel(point),
            expected.get_pixel(point),
            "{point:?}"
        );
    }
    display.assert_eq(&expected);
}

#[test]
fn template_gradient_uses_the_active_background_hint() {
    use crate::common::make_render_tree;
    use buoyant::{
        render::Render as _,
        render_target::{EmbeddedGraphicsRenderTarget, RenderTarget as _},
    };
    use embedded_graphics::pixelcolor::Gray8;

    let image = Tga::<Gray8>::from_slice(GRADIENT_TGA).unwrap();
    let tree = make_render_tree::<Rgb888, _, _>(
        &Image::new(&image).as_template(),
        Size::new(5, 3),
        &mut (),
    );

    // A scoped hint overrides the target's hint and is restored afterwards.
    // No hint falls back to the foreground; opacity also uses the active hint.
    let half_opacity_ramp = [
        BACKGROUND,
        Rgb888::new(45, 52, 65),
        Rgb888::new(70, 65, 70),
        Rgb888::new(95, 77, 75),
        Rgb888::new(120, 90, 80),
    ];
    for (hint, opacity, ramp) in [
        (Some(BACKGROUND), 255, TEMPLATE_RAMP),
        (None, 255, [FOREGROUND; 5]),
        (Some(BACKGROUND), 128, half_opacity_ramp),
    ] {
        let mut display = MockDisplay::new();
        let mut target = EmbeddedGraphicsRenderTarget::new(&mut display);
        assert_eq!(target.background_hint(), None);
        target.with_layer(
            |layer| layer.hint_background(Rgb888::new(1, 2, 3)),
            |target| {
                assert_eq!(target.background_hint(), Some(Rgb888::new(1, 2, 3)));
                if let Some(hint) = hint {
                    target.with_layer(
                        |layer| layer.hint_background(hint).opacity(opacity),
                        |target| {
                            tree.render(target, &FOREGROUND);
                        },
                    );
                    assert_eq!(target.background_hint(), Some(Rgb888::new(1, 2, 3)));
                }
            },
        );
        assert_eq!(target.background_hint(), None);
        if hint.is_none() {
            tree.render(&mut target, &FOREGROUND);
        }

        assert_template_gradient(&display, ramp);
    }
}

#[test]
fn template_grayscale_formats_span_the_full_foreground_range() {
    use buoyant::{render::Render as _, render_target::EmbeddedGraphicsRenderTarget};
    use embedded_graphics::{
        image::ImageDrawable,
        pixelcolor::{Gray2, Gray4, Gray8},
    };

    fn check(
        image: &impl ImageDrawable<Color: embedded_graphics::pixelcolor::GrayColor>,
        ramp: [Rgb888; 5],
    ) {
        let mut display = MockDisplay::new();
        buoyant::render::Image::<_, buoyant::render::image::Template>::new(Point::zero(), image)
            .render(
                &mut EmbeddedGraphicsRenderTarget::new_hinted(&mut display, BACKGROUND),
                &FOREGROUND,
            );
        assert_template_gradient(&display, ramp);
    }

    // TGA's 8-bit samples become 0,1,2,2,3 in Gray2 and 0,4,8,11,15 in Gray4.
    check(
        &Tga::<Gray2>::from_slice(GRADIENT_TGA).unwrap(),
        [
            BACKGROUND,
            Rgb888::new(86, 73, 73),
            Rgb888::new(152, 106, 86),
            Rgb888::new(152, 106, 86),
            FOREGROUND,
        ],
    );
    check(
        &Tga::<Gray4>::from_slice(GRADIENT_TGA).unwrap(),
        [
            BACKGROUND,
            Rgb888::new(73, 66, 70),
            Rgb888::new(126, 93, 81),
            Rgb888::new(166, 113, 89),
            FOREGROUND,
        ],
    );
    check(
        &Tga::<Gray8>::from_slice(GRADIENT_TGA).unwrap(),
        TEMPLATE_RAMP,
    );
}
