//! View types used for building interfaces

mod divider;
mod empty_view;
mod foreach;
mod hstack;
pub mod image;
pub mod match_view;
mod modifier;
pub mod shape;
mod spacer;
mod text;
mod vstack;
mod zstack;

pub use divider::Divider;
pub use empty_view::EmptyView;
pub use foreach::ForEach;
pub use hstack::HStack;
pub use image::Image;
pub use modifier::padding;
pub use spacer::Spacer;
pub(crate) use text::WhitespaceWrap;
pub use text::{HorizontalTextAlignment, Text};
pub use vstack::VStack;
pub use zstack::ZStack;

use modifier::{
    Animated, FixedFrame, FixedSize, FlexFrame, ForegroundStyle, GeometryGroup, Padding, Priority,
};

use crate::{
    environment::DefaultEnvironment,
    primitives::{Point, Size},
    render::{CharacterRender, Renderable},
    Animation,
};

pub trait LayoutExtensions: Sized {
    /// Applies padding to the specified edges
    fn padding(self, edges: padding::Edges, amount: u16) -> Padding<Self> {
        Padding::new(edges, amount, self)
    }

    fn frame(self) -> FixedFrame<Self> {
        FixedFrame::new(self)
    }

    fn flex_frame(self) -> FlexFrame<Self> {
        FlexFrame::new(self)
    }

    /// Proposes ``ProposedDimension::Compact``, resulting in the child view rendering at its ideal
    /// size along the specified axis.
    fn fixed_size(self, horizontal: bool, vertical: bool) -> FixedSize<Self> {
        FixedSize::new(horizontal, vertical, self)
    }

    /// Sets the priority of the view layout.
    ///
    /// Stacks lay out views in groups of priority, with higher priority views being laid out
    /// first. Each set of views in the stack with a given priority are laid out together, with the
    /// stack offering the remaining width divided by the number of views in the group.
    fn priority(self, priority: i8) -> Priority<Self> {
        Priority::new(priority, self)
    }

    /// Applies an animation to a view tree. All views in the tree will be animated according to
    /// the animation curve provided when the value changes. The elapsed duration will be reset
    /// if the value changes before the animation is complete.
    fn animated<T: PartialEq + Clone>(self, animation: Animation, value: T) -> Animated<Self, T> {
        Animated::new(self, animation, value)
    }

    /// Creates a new coordinate space under which views are positioned, allowing views within the
    /// coordinate space to animate relative to a shared origin.
    ///
    /// In the below implementation of a toggle button, the geometry group ensures the circle and
    /// capsule always animate together as one element. Without this, compound animations where the
    /// toggle frame moves as a result of a parent animation would result in the circle moving outside
    /// the capsule. Contrary to what intuition would suggest, simply moving the `.animated` modifier
    /// to encompass the entire toggle does not resolve the issue.
    ///
    /// Example:
    ///
    /// ```
    /// use core::time::Duration;
    /// use buoyant::view::{shape::{Circle, Capsule}, ZStack, padding, LayoutExtensions as _, RenderExtensions as _};
    /// use buoyant::Animation;
    /// use buoyant::layout::HorizontalAlignment;
    /// use buoyant::render::{EmbeddedGraphicsView, Renderable};
    /// use embedded_graphics::pixelcolor::Rgb565;
    /// use embedded_graphics::prelude::*;
    ///
    /// fn toggle_button(is_on: bool) -> impl EmbeddedGraphicsView<Rgb565> {
    ///     let (color, alignment) = if is_on {
    ///         (Rgb565::GREEN, HorizontalAlignment::Trailing)
    ///     } else {
    ///         (Rgb565::CSS_LIGHT_GRAY, HorizontalAlignment::Leading)
    ///     };
    ///
    ///     ZStack::new((
    ///         Capsule.foreground_color(color),
    ///         Circle
    ///             .foreground_color(Rgb565::WHITE)
    ///             .padding(padding::Edges::All, 2)
    ///             .animated(Animation::Linear(Duration::from_millis(100)), is_on),
    ///     ))
    ///     .with_horizontal_alignment(alignment)
    ///     .geometry_group()
    ///     .frame().with_width(50).with_height(25)
    /// }
    /// ```
    fn geometry_group(self) -> GeometryGroup<Self> {
        GeometryGroup::new(self)
    }
}

pub trait RenderExtensions<C>: Sized {
    /// Sets the foreground color
    fn foreground_color(self, color: C) -> ForegroundStyle<Self, C> {
        ForegroundStyle::new(color, self)
    }
}

impl<T: crate::layout::Layout> LayoutExtensions for T {}
impl<T: Renderable<C>, C> RenderExtensions<C> for T {}

// TODO: Remove this
pub fn make_render_tree<C, V>(view: &V, size: Size) -> V::Renderables
where
    V: Renderable<C>,
    V::Renderables: CharacterRender<C>,
{
    let env = DefaultEnvironment::default();
    let layout = view.layout(&size.into(), &env);
    view.render_tree(&layout, Point::default(), &env)
}
