use core::time::Duration;

use crate::{
    environment::LayoutEnvironment,
    layout::{Layout, ResolvedLayout},
    primitives::{Point, Size},
};

mod animate;
mod capsule;
mod circle;
pub mod collections;
mod conditional_tree;
mod empty;
mod offset;
mod one_of;
mod rect;
mod rounded_rect;
mod shade_subtree;
mod text;

pub use animate::Animate;
pub use capsule::Capsule;
pub use circle::Circle;
pub use conditional_tree::{ConditionalTree, Subtree};
pub use offset::Offset;
pub use one_of::{OneOf2, OneOf3};
pub use rect::Rect;
pub use rounded_rect::RoundedRect;
pub use shade_subtree::ShadeSubtree;
pub use text::Text;

pub trait Renderable<Color>: Layout {
    type Renderables;
    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl LayoutEnvironment,
    ) -> Self::Renderables;
}

/// A view that can be rendered to a ``buoyant::render::CharacterRenderTarget``
///
/// This trait primarily serves as a shorthand for the more verbose ``Renderable<C, Renderables:
/// CharacterRender<C>>`` bound
pub trait CharacterView<C>: Renderable<C, Renderables: CharacterRender<C>> {}
impl<C, T: Renderable<C, Renderables: CharacterRender<C>>> CharacterView<C> for T {}

/// A type that does not render, produces no side effects, and no children.
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
pub use embedded_graphics_rendering::{EmbeddedGraphicsRender, EmbeddedView};

#[cfg(feature = "embedded-graphics")]
mod embedded_graphics_rendering {
    use crate::primitives::Point;
    use embedded_graphics::prelude::PixelColor;
    use embedded_graphics_core::draw_target::DrawTarget;

    use super::{AnimationDomain, Renderable};

    /// A view that can be rendered to an `embedded_graphics` target
    ///
    /// This trait serves as a shorthand for the more verbose `Renderable<C, Renderables:
    /// EmbeddedGraphicsRender<C>>` bound
    pub trait EmbeddedGraphicsRender<Color: PixelColor>: Sized + Clone {
        /// Render the view to the screen
        fn render(
            &self,
            render_target: &mut impl DrawTarget<Color = Color>,
            style: &Color,
            offset: Point,
        );

        /// Render view and all subviews, animating from a source view to a target view
        fn render_animated(
            render_target: &mut impl DrawTarget<Color = Color>,
            source: &Self,
            target: &Self,
            style: &Color,
            offset: Point,
            domain: &AnimationDomain,
        ) {
            let intermediate = Self::join(source.clone(), target.clone(), domain);
            intermediate.render(render_target, style, offset);
        }

        /// Produces a new tree by consuming and interpolating between two partially animated trees
        fn join(source: Self, target: Self, domain: &AnimationDomain) -> Self;
    }

    /// A view that can be rendered to an ``embedded_graphics::DrawTarget``
    pub trait EmbeddedView<C: PixelColor>:
        Renderable<C, Renderables: EmbeddedGraphicsRender<C>>
    {
    }

    impl<C: PixelColor, T: Renderable<C, Renderables: EmbeddedGraphicsRender<C>>> EmbeddedView<C>
        for T
    {
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct AnimationDomain {
    /// The animation factor of this domain, ranging from 0 to 255
    pub factor: u8,
    /// The duration since the application started
    pub app_time: Duration,
}

impl AnimationDomain {
    #[must_use]
    pub const fn new(factor: u8, app_time: Duration) -> Self {
        Self { factor, app_time }
    }

    /// Use this to create a new top-level animation domain when rendering
    #[must_use]
    pub const fn top_level(app_time: Duration) -> Self {
        Self {
            factor: 255,
            app_time,
        }
    }

    /// Whether the animation defined by this domain is complete
    #[must_use]
    pub const fn is_complete(&self) -> bool {
        self.factor == 255
    }
}

/// A draw target for rendering characters
pub trait CharacterRenderTarget {
    type Color;
    fn draw_character(&mut self, point: Point, character: char, color: &Self::Color);
    fn draw_string(&mut self, point: Point, string: &str, color: &Self::Color) {
        string.chars().enumerate().for_each(|(i, c)| {
            self.draw_character(point + Point::new(i as i16, 0), c, color);
        });
    }
    fn draw_color(&mut self, point: Point, color: &Self::Color);

    /// The bounds of the target
    fn size(&self) -> Size;
}

/// A view that can be rendered to an `embedded_graphics` target
pub trait CharacterRender<Color>: Sized + Clone {
    /// Render the view to the screen
    fn render(
        &self,
        render_target: &mut impl CharacterRenderTarget<Color = Color>,
        style: &Color,
        offset: Point,
    );

    /// Render view and all subviews, animating from a source view to a target view
    fn render_animated(
        render_target: &mut impl CharacterRenderTarget<Color = Color>,
        source: &Self,
        target: &Self,
        style: &Color,
        offset: Point,
        domain: &AnimationDomain,
    ) {
        let intermediate = Self::join(source.clone(), target.clone(), domain);
        intermediate.render(render_target, style, offset);
    }

    /// Produces a new tree by consuming and interpolating between two partially animated trees
    fn join(source: Self, target: Self, domain: &AnimationDomain) -> Self;
}
