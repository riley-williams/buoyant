use core::time::Duration;

use crate::{
    environment::LayoutEnvironment,
    layout::{Layout, ResolvedLayout},
    primitives::Point,
    render_target::RenderTarget,
};

mod animate;
mod capsule;
mod circle;
pub mod collections;
mod empty;
#[cfg(feature = "embedded-graphics")]
mod image;
mod offset;
mod one_of;
mod rect;
mod rounded_rect;
mod shade_subtree;
mod text;

pub use animate::Animate;
pub use capsule::Capsule;
pub use circle::Circle;
#[cfg(feature = "embedded-graphics")]
pub use image::Image;
pub use offset::Offset;
pub use one_of::{OneOf2, OneOf3, OneOf4};
pub use rect::Rect;
pub use rounded_rect::RoundedRect;
pub use shade_subtree::ShadeSubtree;
pub use text::Text;

/// A type that can produce a render tree.
///
/// The ``Renderables`` associated type specifies the subtree produced.
/// Views may have a matching renderable, like in the case of ``Rectangle``,
/// which has a concrete size and position. Some views like the frame modifier
/// do not produce a node at all, and instead insert their subview render tree.
pub trait Renderable: Layout {
    type Renderables;
    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl LayoutEnvironment,
    ) -> Self::Renderables;
}

/// A type that does not render, produces no side effects, and has no children.
pub trait NullRender {}

impl<T: NullRender + Layout> Renderable for T {
    type Renderables = ();

    fn render_tree(
        &self,
        _layout: &ResolvedLayout<Self::Sublayout>,
        _origin: Point,
        _env: &impl LayoutEnvironment,
    ) {
    }
}

pub trait AnimatedJoin {
    /// Produces a new tree by consuming and interpolating between two partially animated trees
    fn join(source: Self, target: Self, domain: &AnimationDomain) -> Self;
}

pub trait Render<Color>: AnimatedJoin + Sized {
    /// Render the view to the screen
    fn render(
        &self,
        render_target: &mut impl RenderTarget<ColorFormat = Color>,
        style: &Color,
        offset: Point,
    );

    /// Render view and all subviews, animating from a source view to a target view
    ///
    /// The implementation of this method should match the implementation of
    /// ``AnimatedJoin::join`` to get smooth continuous animations
    fn render_animated(
        render_target: &mut impl RenderTarget<ColorFormat = Color>,
        source: &Self,
        target: &Self,
        style: &Color,
        offset: Point,
        domain: &AnimationDomain,
    );
}

#[derive(Debug, Clone, PartialEq, Eq)]
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
