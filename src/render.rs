use crate::{
    environment::LayoutEnvironment,
    layout::{Layout, ResolvedLayout},
    primitives::{Point, Size},
};

pub mod collections;
pub mod primitives;

pub trait Renderable<Color>: Layout {
    type Renderables;
    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl LayoutEnvironment,
    ) -> Self::Renderables;
}

pub trait NullRender {}

impl<C, T: NullRender + Layout> Renderable<C> for T {
    type Renderables = ();

    fn render_tree(
        &self,
        _layout: &ResolvedLayout<Self::Sublayout>,
        _origin: Point,
        _env: &impl LayoutEnvironment,
    ) {
    }
}

#[cfg(feature = "embedded-graphics")]
use embedded_graphics::prelude::PixelColor;
#[cfg(feature = "embedded-graphics")]
use embedded_graphics_core::draw_target::DrawTarget;

/// A view that can be rendered to an embedded_graphics target
#[cfg(feature = "embedded-graphics")]
pub trait EmbeddedGraphicsRender<Color: PixelColor>: Sized + Clone {
    /// Render the view to the screen
    fn render(&self, render_target: &mut impl DrawTarget<Color = Color>, style: &Color);

    /// Render view and all subviews, animating from a source view to a target view
    fn render_animated(
        render_target: &mut impl DrawTarget<Color = Color>,
        source: &Self,
        target: &Self,
        style: &Color,
        config: &AnimationDomain,
    ) {
        let intermediate = Self::join(source.clone(), target.clone(), config);
        // TODO: interpolate styles
        intermediate.render(render_target, style);
    }

    /// Produces a new tree by consuming and interpolating between two partially animated trees
    fn join(source: Self, target: Self, config: &AnimationDomain) -> Self;
}

#[cfg(feature = "embedded-graphics")]
impl<C: PixelColor> EmbeddedGraphicsRender<C> for () {
    /// Render the view to the screen
    fn render(&self, _render_target: &mut impl DrawTarget<Color = C>, _style: &C) {}

    /// Render view and all subviews, animating from a source view to a target view
    fn render_animated(
        _render_target: &mut impl DrawTarget<Color = C>,
        _source: &Self,
        _target: &Self,
        _style: &C,
        _config: &AnimationDomain,
    ) {
    }

    /// Produces a new tree by consuming and interpolating between two partially animated trees
    fn join(_source: Self, _target: Self, _config: &AnimationDomain) -> Self {}
}

impl<C> CharacterRender<C> for () {
    /// Render the view to the screen
    fn render(&self, _render_target: &mut impl CharacterRenderTarget<Color = C>, _style: &C) {}

    /// Render view and all subviews, animating from a source view to a target view
    fn render_animated(
        _render_target: &mut impl CharacterRenderTarget<Color = C>,
        _source: &Self,
        _target: &Self,
        _style: &C,
        _config: &AnimationDomain,
    ) {
    }

    /// Produces a new tree by consuming and interpolating between two partially animated trees
    fn join(_source: Self, _target: Self, _config: &AnimationDomain) -> Self {}
}

#[derive(Debug, Clone, PartialEq)]
pub struct AnimationDomain {
    /// The animation factor of this domain, ranging from 0 to 255
    pub factor: u8,
    /// The time elapsed in milliseconds from when the animation started
    /// This is primarily useful for creating a subdomain with a different speed
    pub offset_ms: u64,
}

pub trait CharacterRenderTarget {
    type Color;
    fn draw_character(&mut self, point: Point, character: char, color: &Self::Color);
    fn draw_string(&mut self, point: Point, string: &str, color: &Self::Color) {
        string.chars().enumerate().for_each(|(i, c)| {
            self.draw_character(point + Point::new(i as i16, 0), c, color);
        });
    }
    fn draw_color(&mut self, point: Point, color: &Self::Color);

    fn size(&self) -> Size;
}

/// A view that can be rendered to an embedded_graphics target
pub trait CharacterRender<Color>: Sized + Clone {
    /// Render the view to the screen
    fn render(&self, render_target: &mut impl CharacterRenderTarget<Color = Color>, style: &Color);

    /// Render view and all subviews, animating from a source view to a target view
    fn render_animated(
        render_target: &mut impl CharacterRenderTarget<Color = Color>,
        source: &Self,
        target: &Self,
        style: &Color,
        config: &AnimationDomain,
    ) {
        let intermediate = Self::join(source.clone(), target.clone(), config);
        // TODO: interpolate styles
        intermediate.render(render_target, style);
    }

    /// Produces a new tree by consuming and interpolating between two partially animated trees
    fn join(source: Self, target: Self, config: &AnimationDomain) -> Self;
}
