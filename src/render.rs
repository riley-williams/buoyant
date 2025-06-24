//! Render primitives.
//!
//! If you are constructing views, this is probably not the module you want. Use ``ViewHandle`` instead.

use core::time::Duration;

use crate::{primitives::Point, render_target::RenderTarget};

mod animate;
pub mod collections;
mod container;
mod empty;
#[cfg(feature = "embedded-graphics")]
mod image;
mod offset;
mod one_of;
mod option;
mod shade_subtree;
pub mod shape;
mod text;

pub use animate::Animate;
pub use container::Container;
#[cfg(feature = "embedded-graphics")]
pub use image::Image;
pub use offset::Offset;
pub use one_of::{OneOf2, OneOf3, OneOf4};
pub use shade_subtree::ShadeSubtree;
pub use shape::Capsule;
pub use shape::Circle;
pub use shape::Rect;
pub use shape::RoundedRect;
pub use shape::StrokedShape;
pub use text::Text;

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
