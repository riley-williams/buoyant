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
    Animated, BackgroundView, FixedFrame, FixedSize, FlexFrame, ForegroundStyle, GeometryGroup,
    Padding, Priority,
};

use crate::{
    animation::Animation,
    environment::DefaultEnvironment,
    layout::{HorizontalAlignment, VerticalAlignment},
    primitives::{Point, Size},
    render::{CharacterRender, Renderable},
};

/// Modifiers that extend the functionality of views.
pub trait LayoutExtensions: Sized {
    /// Applies padding to the specified edges
    fn padding(self, edges: padding::Edges, amount: u16) -> Padding<Self> {
        Padding::new(edges, amount, self)
    }

    /// A virtual frame that can be configured with fixed size dimensions.
    fn frame(self) -> FixedFrame<Self> {
        FixedFrame::new(self)
    }

    /// A fixed size frame with the specified width and height.
    ///
    /// This is a shortcut for:
    ///
    /// ```
    /// # use buoyant::view::LayoutExtensions as _;
    /// # let my_view = buoyant::view::shape::Rectangle;
    /// # let width = 100;
    /// # let height = 100;
    /// my_view
    ///     .frame()
    ///     .with_width(width)
    ///     .with_height(height)
    /// # ;
    /// ```
    fn frame_with_size(self, width: u16, height: u16) -> FixedFrame<Self> {
        FixedFrame::new(self).with_width(width).with_height(height)
    }

    /// A virtual frame that can be configured with flexible dimensions.
    fn flex_frame(self) -> FlexFrame<Self> {
        FlexFrame::new(self)
    }

    /// Creates a virtual frame that expands to fill as much horizontal space as possible.
    ///
    /// This is a shortcut for:
    ///
    /// ```
    /// # use buoyant::view::LayoutExtensions as _;
    /// # let my_view = buoyant::view::shape::Rectangle;
    /// # let alignment = buoyant::layout::HorizontalAlignment::Center;
    /// my_view
    ///     .flex_frame()
    ///     .with_infinite_max_width()
    ///     .with_horizontal_alignment(alignment)
    /// # ;
    /// ```
    fn flex_infinite_width(self, alignment: HorizontalAlignment) -> FlexFrame<Self> {
        FlexFrame::new(self)
            .with_infinite_max_width()
            .with_horizontal_alignment(alignment)
    }

    /// Creates a virtual frame that expands to fill as much vertical space as possible.
    ///
    /// This is a shortcut for:
    ///
    /// ```
    /// # use buoyant::view::LayoutExtensions as _;
    /// # use buoyant::layout::VerticalAlignment;
    /// # let my_view = buoyant::view::shape::Rectangle;
    /// # let alignment = VerticalAlignment::Center;
    /// my_view
    ///     .flex_frame()
    ///     .with_infinite_max_height()
    ///     .with_vertical_alignment(alignment)
    /// # ;
    /// ```
    fn flex_infinite_height(self, alignment: VerticalAlignment) -> FlexFrame<Self> {
        FlexFrame::new(self)
            .with_infinite_max_width()
            .with_vertical_alignment(alignment)
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
    /// use buoyant::animation::Animation;
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
    ///             .animated(Animation::ease_in_out(Duration::from_millis(120)), is_on),
    ///     ))
    ///     .with_horizontal_alignment(alignment)
    ///     .geometry_group()
    ///     .frame()
    ///     .with_size(50, 25)
    /// }
    /// ```
    fn geometry_group(self) -> GeometryGroup<Self> {
        GeometryGroup::new(self)
    }

    /// A view that uses the layout of the foreground view and renders the background
    /// behind it.
    ///
    /// Example:
    ///
    /// ```
    /// use buoyant::view::{padding::Edges, shape::RoundedRectangle, Text, LayoutExtensions as _, RenderExtensions as _};
    /// use buoyant::render::{EmbeddedGraphicsView};
    /// use embedded_graphics::{prelude::*, pixelcolor::Rgb565, mono_font::ascii::FONT_9X15_BOLD};
    ///
    /// fn bordered_button() -> impl EmbeddedGraphicsView<Rgb565> {
    ///     Text::new("Press me", &FONT_9X15_BOLD)
    ///         .foreground_color(Rgb565::WHITE)
    ///         .padding(Edges::All, 10)
    ///         .background(|| {
    ///             RoundedRectangle::new(10)
    ///                 .foreground_color(Rgb565::BLUE)
    ///         })
    /// }
    /// ```
    fn background<U>(self, background: impl FnOnce() -> U) -> BackgroundView<Self, U> {
        BackgroundView::new(self, background())
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
