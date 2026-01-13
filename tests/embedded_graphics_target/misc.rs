use buoyant::{
    render_target::{EmbeddedGraphicsRenderTarget, RenderTarget, surface::AsDrawTarget},
    view::prelude::*,
};
use embedded_graphics::{
    geometry::{OriginDimensions, Point as EgPoint, Size as EgSize},
    mock_display::MockDisplay,
    pixelcolor::Rgb888,
    prelude::{Drawable, Primitive, RgbColor, WebColors},
    primitives::{PrimitiveStyleBuilder, Rectangle as EgRectangle},
};

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
        .as_drawable(display.size(), Rgb888::WHITE, &mut ())
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
