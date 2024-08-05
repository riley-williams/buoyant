use crate::{
    layout::{Layout, ResolvedLayout},
    primitives::{Point, Size},
    render::CharacterRender,
    render_target::CharacterRenderTarget,
};
use micromath::F32;

#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct RoundedRectangle {
    corner_radius: u16,
    anti_aliasing: bool,
}

impl RoundedRectangle {
    pub fn new(corner_radius: u16) -> Self {
        Self {
            corner_radius,
            anti_aliasing: false,
        }
    }
    pub fn with_anti_aliasing(mut self, anti_aliasing: bool) -> Self {
        self.anti_aliasing = anti_aliasing;
        self
    }
}

impl Layout for RoundedRectangle {
    type Sublayout = ();

    fn layout(
        &self,
        offer: Size,
        _: &impl crate::environment::LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        ResolvedLayout {
            sublayouts: (),
            resolved_size: offer,
        }
    }
}

impl<P: Copy> CharacterRender<P> for RoundedRectangle {
    fn render(
        &self,
        target: &mut impl CharacterRenderTarget<Color = P>,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl crate::environment::RenderEnvironment<Color = P>,
    ) {
        let width = layout.resolved_size.width;
        let height = layout.resolved_size.height;
        let color = env.foreground_color();

        for y in 0..height as i16 {
            for x in 0..width as i16 {
                target.draw(origin + Point::new(x, y), ' ', color);
            }
        }
    }
}

#[cfg(feature = "embedded-graphics")]
use embedded_graphics::draw_target::DrawTarget;

#[cfg(feature = "embedded-graphics")]
impl<P: embedded_graphics_core::pixelcolor::PixelColor> crate::render::PixelRender<P>
    for RoundedRectangle
{
    fn render(
        &self,
        target: &mut impl DrawTarget<Color = P>,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl crate::environment::RenderEnvironment<Color = P>,
    ) {
        let width = layout.resolved_size.width as u32;
        let height = layout.resolved_size.height as u32;
        let color = env.foreground_color();

        let corner_radius = self.corner_radius as u32;
        // attempt to draw inside as 3 rectangles to utilize accelerated rectangle drawing
        let mut center_origin = origin;
        center_origin.x += self.corner_radius as i16;
        let area = embedded_graphics::primitives::Rectangle::new(
            center_origin.into(),
            (width - 2 * corner_radius, height).into(),
        );
        _ = target.fill_solid(&area, color);

        let side_rect_height = height - 2 * corner_radius;
        let side_rect_y = origin.y as i32 + corner_radius as i32;

        let left_area = embedded_graphics::primitives::Rectangle::new(
            (origin.x as i32, side_rect_y).into(),
            (corner_radius, side_rect_height).into(),
        );
        _ = target.fill_solid(&left_area, color);

        let right_area = embedded_graphics::primitives::Rectangle::new(
            (
                origin.x as i32 + width as i32 - corner_radius as i32,
                side_rect_y,
            )
                .into(),
            (corner_radius, side_rect_height).into(),
        );
        _ = target.fill_solid(&right_area, color);

        // draw the corners
        let sq_radius = self.corner_radius as u32 * self.corner_radius as u32;
        for y in 0..self.corner_radius as u32 {
            let intercept: f32 = F32::from((sq_radius - y * y) as f32).sqrt().into();
            // TODO: anti-aliasing
            // let remainder = intercept.fract() * 255.0;
            let x = intercept as u32;
            // draw a line in each unfilled corner
            let tl_x = origin.x as i32 + corner_radius as i32 - x as i32;
            let tl_y = origin.y as i32 + y as i32;
            let rect = embedded_graphics::primitives::Rectangle::new(
                (tl_x, tl_y).into(),
                (origin.y as u32 + corner_radius, tl_y as u32).into(),
            );
            _ = target.fill_solid(&rect, color);

            let tr_x = origin.x as i32 + width as i32 - corner_radius as i32;
            let tr_y = origin.y as i32 + y as i32;
            let rect = embedded_graphics::primitives::Rectangle::new(
                (tr_x, tr_y).into(),
                (origin.y as u32 + corner_radius, tr_y as u32).into(),
            );
            _ = target.fill_solid(&rect, color);

            let bl_x = origin.x as i32 + corner_radius as i32 - x as i32;
            let bl_y = origin.y as i32 + height as i32 - y as i32;
            let rect = embedded_graphics::primitives::Rectangle::new(
                (bl_x, bl_y).into(),
                (origin.y as u32 + corner_radius, bl_y as u32).into(),
            );
            _ = target.fill_solid(&rect, color);

            let br_x = origin.x as i32 + width as i32 - corner_radius as i32;
            let br_y = origin.y as i32 + height as i32 - y as i32;
            let rect = embedded_graphics::primitives::Rectangle::new(
                (br_x, br_y).into(),
                (origin.y as u32 + corner_radius, br_y as u32).into(),
            );
            _ = target.fill_solid(&rect, color);
        }
    }
}
