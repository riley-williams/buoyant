//! Render primitives.
//!
//! If you are constructing views, this is not the module you want. Use types under [`crate::view`] instead.
//!
//! Render primitives represent the minimal information required to render a view. Nodes in the
//! render tree can be animated and joined with other trees of the same type to enable complex
//! animations.

use core::time::Duration;

use crate::render_target::RenderTarget;

mod animate;
pub mod collections;
mod container;
mod empty;
#[cfg(feature = "embedded-graphics")]
mod image;
mod offset;
mod one_of;
mod option;
mod scroll_metadata;
mod shade_subtree;
pub mod shape;
pub mod text;
mod transform;
mod transition_option;

pub use animate::Animate;
pub use container::Container;
#[cfg(feature = "embedded-graphics")]
pub use image::Image;
pub use offset::Offset;
pub use one_of::{OneOf2, OneOf3, OneOf4};
pub use scroll_metadata::ScrollMetadata;
pub use shade_subtree::ShadeSubtree;
pub use shape::Capsule;
pub use shape::Circle;
pub use shape::Rect;
pub use shape::RoundedRect;
pub use shape::StrokedShape;
pub use text::Text;
pub use transform::Transform;
pub use transition_option::TransitionOption;

pub trait AnimatedJoin {
    /// Modifies a target tree by joining it with the source tree
    fn join_from(&mut self, source: &Self, domain: &AnimationDomain);
}

/// A type that can be rendered to a target and animated
pub trait Render<Color>: AnimatedJoin + Sized {
    /// Render the view to the screen
    fn render(&self, render_target: &mut impl RenderTarget<ColorFormat = Color>, style: &Color);

    /// Render view and all subviews, animating from a source view to a target view
    ///
    /// The implementation of this method should match the implementation of
    /// [`AnimatedJoin::join_from`] to get smooth continuous animations
    fn render_animated(
        render_target: &mut impl RenderTarget<ColorFormat = Color>,
        source: &Self,
        target: &Self,
        style: &Color,
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
