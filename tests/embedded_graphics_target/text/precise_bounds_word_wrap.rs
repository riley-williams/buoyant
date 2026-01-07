//! Tests for precise bounds calculation of text views using various fonts.
//!
//! Compares the expected bounds with the actual rendered bounds on an embedded-graphics target.

use buoyant::environment::DefaultEnvironment;
use buoyant::font::FontRender;
use buoyant::if_view;
use buoyant::primitives::ProposedDimensions;
use buoyant::render::Render as _;
use buoyant::render_target::EmbeddedGraphicsRenderTarget;
use buoyant::{
    font::Font,
    primitives::{Point, Size, geometry::Rectangle},
    view::prelude::*,
};

use embedded_graphics::prelude::RgbColor;
use embedded_graphics::{mock_display::MockDisplay, pixelcolor::Rgb888};
use u8g2_fonts::FontRenderer;

fn expected_bounds(text: &str, font: &impl Font, size: &ProposedDimensions) -> Option<Rectangle> {
    let text_view = Text::new(text, font).with_precise_bounds();
    let layout = text_view.layout(size, &DefaultEnvironment::default(), &mut (), &mut ());

    if layout.resolved_size.area() == 0 {
        None
    } else {
        Some(Rectangle::new(Point::zero(), layout.resolved_size.into()))
    }
}

/// Helper to render text and return the affected area
fn rendered_text_bounds(
    text: &str,
    font: &impl buoyant::font::FontRender<Rgb888>,
    size: Size,
    print: bool,
) -> Option<Rectangle> {
    let view = if_view!((print) {
        Text::new(text, font)
        .with_precise_bounds()
        .foreground_color(Rgb888::WHITE)
        .background(Alignment::Center, {
                    Rectangle
                        .stroked_offset(2, StrokeOffset::Inner)
                        .foreground_color(Rgb888::GREEN)
                })
    } else {
        Text::new(text, font)
        .with_precise_bounds()
        .foreground_color(Rgb888::WHITE)

    });

    let mut display = MockDisplay::new();
    display.set_allow_out_of_bounds_drawing(true);
    display.set_allow_overdraw(true);
    let mut target = EmbeddedGraphicsRenderTarget::new_hinted(&mut display, Rgb888::BLACK);
    let mut state = view.build_state(&mut ());
    let layout = view.layout(
        &size.into(),
        &DefaultEnvironment::default(),
        &mut (),
        &mut state,
    );
    let render_tree = view.render_tree(
        &layout,
        Point::zero(),
        &DefaultEnvironment::default(),
        &mut (),
        &mut state,
    );
    render_tree.render(&mut target, &Rgb888::WHITE);

    if print {
        println!("{display:?}");
    }

    let area = display.affected_area();
    if area.size.width == 0 || area.size.height == 0 {
        None
    } else {
        Some(area.into())
    }
}

const TEST_TEXTS: &[&str] = &[
    "Hello, World!",
    "The quick brown fox jumps over the lazy dog.",
    "1234567890",
    "!@#$%^&*()_+-=[]{}|;':\",.<>/?`~",
    "Line 1\nLine 2\nLine 3",
];

#[test]
fn precise_bounds_u8g2_helvetica() {
    exhaustively_check_precise_bounds(
        &FontRenderer::new::<u8g2_fonts::fonts::u8g2_font_helvB08_tf>(),
    );
}

#[test]
fn precise_bounds_u8g2_courier_18() {
    exhaustively_check_precise_bounds(
        &FontRenderer::new::<u8g2_fonts::fonts::u8g2_font_courB18_tf>(),
    );
}

#[test]
fn precise_bounds_u8g2_courier_14() {
    exhaustively_check_precise_bounds(
        &FontRenderer::new::<u8g2_fonts::fonts::u8g2_font_courR14_tf>(),
    );
}

#[test]
fn precise_bounds_u8g2_glasstown() {
    exhaustively_check_precise_bounds(&FontRenderer::new::<
        u8g2_fonts::fonts::u8g2_font_glasstown_nbp_tf,
    >());
}

#[ignore = "The results are extremely close, not sure if this is a quirk of the renderer or a bug"]
#[test]
fn precise_bounds_otf_sniglet() {
    exhaustively_check_precise_bounds(
        &rusttype::Font::try_from_bytes(
            include_bytes!("../assets/fonts/Sniglet Regular.otf") as &[u8]
        )
        .unwrap(),
    );
}

#[ignore = "Significant negative offsets cause weirdness"]
#[test]
fn precise_bounds_u8g2_mystery_quest() {
    exhaustively_check_precise_bounds(&FontRenderer::new::<
        u8g2_fonts::fonts::u8g2_font_mystery_quest_24_tf,
    >());
}

fn exhaustively_check_precise_bounds(font: &impl FontRender<Rgb888>) {
    // With precise bounds
    for text in TEST_TEXTS {
        for width in 20..60 {
            for height in 20..40 {
                let size = Size::new(width, height);
                let expected_bounds = expected_bounds(text, font, &size.into());
                let actual_bounds = rendered_text_bounds(text, font, size, false);
                if actual_bounds != expected_bounds {
                    rendered_text_bounds(text, font, size, true);
                }
                assert!(
                    size.width >= actual_bounds.clone().map_or(0, |r| r.size.width),
                    "Expected actual width of at most {} but got {:?}",
                    size.width,
                    actual_bounds
                );
                assert!(
                    size.height >= actual_bounds.clone().map_or(0, |r| r.size.height),
                    "Expected actual height of at most {} but got {:?}",
                    size.height,
                    actual_bounds
                );
                assert_eq!(
                    expected_bounds, actual_bounds,
                    "Failed for size: {size:?}, text: {text:?}"
                );
            }
        }
    }
}

/// The 'j' here has a negative x offset, which can cause issues
/// This specific size is known to be tricky
#[test]
fn check_j_offset() {
    let font = &FontRenderer::new::<u8g2_fonts::fonts::u8g2_font_mystery_quest_24_tf>();
    // With precise bounds
    let text = "The quick brown fox jumps over the lazy dog.";
    let width = 51;
    let height = 31;
    let size = Size::new(width, height);
    let expected_bounds = expected_bounds(text, font, &size.into());
    let actual_bounds = rendered_text_bounds(text, font, size, false);
    if actual_bounds != expected_bounds {
        rendered_text_bounds(text, font, size, true);
    }
    assert_eq!(
        expected_bounds, actual_bounds,
        "Failed for size: {size:?}, text: {text:?}"
    );
}
