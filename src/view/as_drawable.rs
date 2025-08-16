use crate::{
    primitives::{Interpolate, ProposedDimensions},
    render::Render,
    view::View,
};

/// A view that can be converted into an embedded-graphics drawable.
pub trait AsDrawable<Color, Captures: ?Sized> {
    /// Converts a view into an object that can be drawn with the [embedded_graphics]
    /// crate.
    ///
    /// This trait provides a convenient way to draw views directly by returning
    /// an [`embedded_graphics::Drawable`] and internally performing the layout and
    /// render tree generation.
    ///
    /// # Examples
    ///
    /// ```
    /// use buoyant::view::prelude::*;
    /// use embedded_graphics::{mono_font::ascii::FONT_10X20, pixelcolor::Rgb888, prelude::*};
    /// use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, Window};
    ///
    /// let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(480, 320));
    ///
    /// let view = Text::new("Hello Buoyant!", &FONT_10X20)
    ///     .foreground_color(Rgb888::GREEN);
    ///
    /// view.as_drawable(display.size(), Rgb888::BLACK, &mut ())
    ///     .draw(&mut display)
    ///     .unwrap();
    /// ```
    ///
    /// [embedded_graphics]: https://docs.rs/embedded_graphics
    fn as_drawable(
        &self,
        size: impl Into<ProposedDimensions>,
        default_color: Color,
        captures: &mut Captures,
    ) -> impl embedded_graphics_core::Drawable<Color = Color, Output = ()>;
}

impl<Color, Captures: ?Sized, T> AsDrawable<Color, Captures> for T
where
    Color: embedded_graphics_core::pixelcolor::PixelColor + Interpolate,
    T: View<Color, Captures>,
{
    fn as_drawable(
        &self,
        size: impl Into<ProposedDimensions>,
        default_color: Color,
        captures: &mut Captures,
    ) -> impl embedded_graphics_core::Drawable<Color = Color, Output = ()> {
        use crate::{environment::DefaultEnvironment, primitives::Point};

        let env = DefaultEnvironment::non_animated();
        let mut state = self.build_state(captures);
        let layout = self.layout(&size.into(), &env, captures, &mut state);
        let render_tree = self.render_tree(&layout, Point::zero(), &env, captures, &mut state);
        DrawableView {
            render_tree,
            default_color,
        }
    }
}

struct DrawableView<T, C> {
    render_tree: T,
    default_color: C,
}

impl<T: Render<C>, C: embedded_graphics_core::pixelcolor::PixelColor + Interpolate>
    embedded_graphics_core::Drawable for DrawableView<T, C>
{
    type Color = C;
    type Output = ();

    fn draw<D>(&self, target: &mut D) -> Result<Self::Output, D::Error>
    where
        D: embedded_graphics_core::draw_target::DrawTarget<Color = Self::Color>,
    {
        // create a temporary embedded graphics render target
        let mut embedded_target = crate::render_target::EmbeddedGraphicsRenderTarget::new(target);
        self.render_tree
            .render(&mut embedded_target, &self.default_color);
        Ok(())
    }
}
