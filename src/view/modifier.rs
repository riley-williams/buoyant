//! View modifiers are used to alter the layout and rendering of views.
//!
//! These are not intended to be used directly, but rather via the methods in [`ViewModifier`]
//!
//! [`ViewModifier`]: crate::view::ViewModifier

mod animated;
#[allow(missing_docs)]
pub mod aspect_ratio;
mod background;
mod erase_captures;
mod fixed_frame;
mod fixed_size;
mod flex_frame;
mod foreground_color;
mod geometry_group;
mod hidden;
mod offset;
mod overlay;
#[allow(missing_docs)]
pub mod padding;
mod priority;

pub(crate) use animated::Animated;
pub(crate) use aspect_ratio::AspectRatio;
pub(crate) use background::BackgroundView;
pub(crate) use erase_captures::EraseCaptures;
pub(crate) use fixed_frame::FixedFrame;
pub(crate) use fixed_size::FixedSize;
pub(crate) use flex_frame::FlexFrame;
pub(crate) use foreground_color::ForegroundStyle;
pub(crate) use geometry_group::GeometryGroup;
pub(crate) use hidden::Hidden;
pub(crate) use offset::Offset;
pub(crate) use overlay::OverlayView;
pub(crate) use padding::Padding;
pub(crate) use priority::Priority;

use crate::{
    animation::Animation,
    layout::{Alignment, HorizontalAlignment, VerticalAlignment},
    primitives::Point,
    view::ViewMarker,
};

impl<T> ViewModifier for T where T: ViewMarker {}

/// Modifiers that extend the functionality of views.
pub trait ViewModifier: Sized {
    /// Applies an animation to a view tree. All views in the tree will be animated according to
    /// the animation curve provided when the value changes. The elapsed duration will be reset
    /// if the value changes before the animation is complete.
    ///
    /// # Examples
    ///
    /// A toggle button that animates the circle within a capsule, ensuring they stay together:
    ///
    /// ```
    /// use core::time::Duration;
    /// use buoyant::view::prelude::*;
    /// use embedded_graphics::pixelcolor::Rgb565;
    /// use embedded_graphics::prelude::*;
    ///
    /// fn toggle_button(is_on: bool) -> impl View<Rgb565, ()> {
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
    ///             .padding(Edges::All, 2)
    ///             .animated(Animation::ease_in_out(Duration::from_millis(120)), is_on),
    ///     ))
    ///     .with_horizontal_alignment(alignment)
    ///     .geometry_group()
    ///     .frame_sized(50, 25)
    /// }
    /// ```
    ///
    /// See [`ViewModifier::geometry_group`] for creating correct compound animations.
    fn animated<T: PartialEq + Clone>(self, animation: Animation, value: T) -> Animated<Self, T> {
        Animated::new(self, animation, value)
    }

    /// Constrains the dimensions to the specified aspect ratio.
    ///
    /// # Examples
    ///
    /// A [`Fixed`][`aspect_ratio::Ratio::Fixed`] 16:9 aspect ratio rectangle that
    /// will scale to fit the available space:
    ///
    /// ```
    /// use buoyant::view::prelude::*;
    /// use embedded_graphics::pixelcolor::Rgb565;
    /// use embedded_graphics::prelude::RgbColor;
    ///
    /// fn widescreen_rectangle() -> impl View<Rgb565, ()> {
    ///     Rectangle
    ///         .aspect_ratio(
    ///             Ratio::Fixed(16, 9),
    ///             ContentMode::Fit
    ///         )
    /// }
    /// ```
    ///
    /// Use [`Ratio::Ideal`][`aspect_ratio::Ratio::Ideal`] to maintain the child's
    /// ideal aspect ratio.
    ///
    /// An icon that maintains its aspect ratio while fitting within a 100x100 area:
    ///
    /// ```
    /// use buoyant::view::prelude::*;
    /// use embedded_graphics::pixelcolor::Rgb565;
    /// use embedded_graphics::prelude::RgbColor;
    ///
    /// fn profile_icon() -> impl View<Rgb565, ()> {
    ///     image_placeholder()
    ///         .aspect_ratio(Ratio::Ideal, ContentMode::Fit)
    ///         .flex_frame()
    ///         .with_max_size(100, 100)
    /// }
    ///
    /// /// (Equivalent to) a flexible 2:3 ratio image
    /// fn image_placeholder() -> impl View<Rgb565, ()> {
    ///     Rectangle
    ///         .flex_frame()
    ///         .with_ideal_size(200, 300)
    /// }
    /// ```
    fn aspect_ratio(
        self,
        aspect_ratio: aspect_ratio::Ratio,
        content_mode: aspect_ratio::ContentMode,
    ) -> AspectRatio<Self> {
        AspectRatio::new(self, aspect_ratio, content_mode)
    }

    /// Adds content behind the modified view, laid out within the modified view's bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use buoyant::view::prelude::*;
    /// use embedded_graphics::{prelude::*, pixelcolor::Rgb565, mono_font::ascii::FONT_9X15_BOLD};
    ///
    /// fn bordered_text() -> impl View<Rgb565, ()> {
    ///     Text::new("Foreground", &FONT_9X15_BOLD)
    ///         .padding(Edges::All, 6)
    ///         .background(Alignment::default(), RoundedRectangle::new(10).stroked(2))
    ///         .foreground_color(Rgb565::WHITE)
    /// }
    /// ```
    fn background<U>(self, alignment: Alignment, background: U) -> BackgroundView<Self, U> {
        BackgroundView::new(self, background, alignment)
    }

    /// Converts the captures of a parent view to [`()`]
    ///
    /// # Examples
    ///
    /// Erase a parent `u32` capture to insert a non-capturing component view:
    ///
    /// ```
    /// use buoyant::view::prelude::*;
    /// use embedded_graphics::pixelcolor::Rgb888;
    ///
    /// fn view() -> impl View<Rgb888, u32> {
    ///     component_view().erase_captures()
    /// }
    ///
    /// fn component_view() -> impl View<Rgb888, ()> {
    ///     Rectangle
    /// }
    /// ```
    ///
    /// When making generic views that do not rely on any particular captures,
    /// consider instead using a generic type parameter for the captures.
    ///
    /// The generic should generally remove the implicit [`Sized`] bound with `T: ?Sized`,
    /// as captures are passed by reference.
    ///
    /// ```
    /// use buoyant::view::prelude::*;
    /// use embedded_graphics::pixelcolor::Rgb888;
    ///
    /// fn component_view<T: ?Sized>() -> impl View<Rgb888, T> {
    ///     Rectangle
    /// }
    /// ```
    fn erase_captures(self) -> EraseCaptures<Self> {
        EraseCaptures::new(self)
    }

    /// Proposes [`ProposedDimension::Compact`][compact], resulting in the child
    /// view rendering at its ideal size along the specified axis.
    ///
    /// # Examples
    ///
    /// Especially with multi-line text, it is often desirable to have the text
    /// forced to its full potential height while keeping the width constraints
    /// unchanged.
    ///
    /// ```
    /// use buoyant::view::prelude::*;
    /// use embedded_graphics::{pixelcolor::Rgb565, mono_font::ascii::FONT_9X15_BOLD};
    ///
    /// fn ideal_height_text() -> impl View<Rgb565, ()> {
    ///     Text::new("Pretend there is a lot of content here", &FONT_9X15_BOLD)
    ///         .fixed_size(false, true)
    /// }
    /// ```
    ///
    /// Where possible, prefer increasing the [`priority`][`ViewModifier::priority`] of
    /// the problematic text view subtree in the stack as [`ViewModifier::fixed_size`]
    /// will never clip text even when it's too large to fit in the available space.
    ///
    /// [compact]: crate::primitives::ProposedDimension::Compact
    fn fixed_size(self, horizontal: bool, vertical: bool) -> FixedSize<Self> {
        FixedSize::new(horizontal, vertical, self)
    }

    /// A virtual frame that can be configured with flexible dimensions.
    ///
    /// # Examples
    ///
    /// A flexible frame that constraints the view to a minimum size of 25x25,
    /// maximum width of 50, and aligns the content to the top leading corner:
    ///
    /// ```
    /// use buoyant::view::prelude::*;
    ///
    /// # let my_view = Rectangle;
    /// my_view
    ///     .flex_frame()
    ///     .with_min_size(25, 25)
    ///     .with_max_width(50)
    ///     .with_alignment(Alignment::TopLeading)
    /// # ;
    /// ```
    fn flex_frame(self) -> FlexFrame<Self> {
        FlexFrame::new(self)
    }

    /// Creates a virtual frame that expands to fill as much vertical space as possible.
    ///
    /// This is a shortcut for:
    ///
    /// ```
    /// # use buoyant::view::prelude::*;
    /// # let my_view = Rectangle;
    /// # let alignment = VerticalAlignment::Center;
    /// my_view
    ///     .flex_frame()
    ///     .with_infinite_max_height()
    ///     .with_vertical_alignment(alignment)
    /// # ;
    /// ```
    fn flex_infinite_height(self, alignment: VerticalAlignment) -> FlexFrame<Self> {
        FlexFrame::new(self)
            .with_infinite_max_height()
            .with_vertical_alignment(alignment)
    }

    /// Creates a virtual frame that expands to fill as much horizontal space as possible.
    ///
    /// This is a shortcut for:
    ///
    /// ```
    /// # use buoyant::view::prelude::*;
    /// # let my_view = Rectangle;
    /// # let alignment = HorizontalAlignment::Center;
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

    /// Sets the foreground color of the modified view and its children.
    fn foreground_color<C>(self, color: C) -> ForegroundStyle<Self, C> {
        ForegroundStyle::new(color, self)
    }

    /// A virtual fixed-size frame that can be configured with fixed size dimensions.
    ///
    /// # Examples
    ///
    /// A circle with dimensions fixed to 100x100:
    ///
    /// ```
    /// use buoyant::view::prelude::*;
    /// use embedded_graphics::pixelcolor::Rgb565;
    ///
    /// fn circle() -> impl View<Rgb565, ()> {
    ///     Circle.frame().with_width(100).with_height(100)
    /// }
    /// ```
    fn frame(self) -> FixedFrame<Self> {
        FixedFrame::new(self)
    }

    /// A fixed size frame with the specified width and height.
    ///
    /// This is a shortcut for:
    ///
    /// ```
    /// # use buoyant::view::prelude::*;
    /// # let my_view = buoyant::view::shape::Rectangle;
    /// #
    /// my_view
    ///     .frame()
    ///     .with_width(100)
    ///     .with_height(100)
    /// # ;
    /// ```
    fn frame_sized(self, width: u32, height: u32) -> FixedFrame<Self> {
        FixedFrame::new(self).with_width(width).with_height(height)
    }

    /// Creates a new coordinate space under which views are positioned relative to a
    /// zero offset, allowing views within the coordinate space to animate relative
    /// to a shared origin.
    ///
    /// In the below implementation of a toggle button, the geometry group ensures
    /// the circle and capsule always animate together as one element. Without this,
    /// compound animations where the toggle frame moves as a result of a parent
    /// animation would result in the circle moving outside the capsule.
    ///
    /// Contrary to what intuition might suggest, simply moving the [`animated`] modifier
    /// to encompass the entire toggle would not resolve the issue.
    ///
    /// ```
    /// use core::time::Duration;
    /// use buoyant::view::prelude::*;
    /// use embedded_graphics::pixelcolor::Rgb565;
    /// use embedded_graphics::prelude::*;
    ///
    /// fn toggle_button(is_on: bool) -> impl View<Rgb565, ()> {
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
    ///             .padding(Edges::All, 2)
    ///             .animated(Animation::ease_in_out(Duration::from_millis(120)), is_on),
    ///     ))
    ///     .with_horizontal_alignment(alignment)
    ///     .geometry_group()
    ///     .frame_sized(50, 25)
    /// }
    /// ```
    ///
    /// [animated]: ViewModifier::animated
    fn geometry_group(self) -> GeometryGroup<Self> {
        GeometryGroup::new(self)
    }

    /// Lays out the view, but does not render it.
    ///
    /// The `.hidden()` modifier is occasionally useful for creating workarounds (read: hacks)
    /// that produce otherwise impossible layouts. It is typically used in combination with
    /// [`ViewModifier::background`] or [`ZStack`].
    ///
    /// This is intentionally not configurable with, e.g. `.hidden(true/false)`,
    /// as it entirely prunes the subtree from the render tree type, resulting in no additional
    /// memory or computation during rendering / animation.
    fn hidden(self) -> Hidden<Self> {
        Hidden::new(self)
    }

    /// Offsets a view by the specified values.
    ///
    /// This does not affect size calculations, and is only applied when rendering the view.
    fn offset(self, x: i32, y: i32) -> Offset<Self> {
        Offset::new(self, Point::new(x, y))
    }

    /// Overlay uses the modified view to compute bounds, and renders the overlay
    /// on top.
    ///
    /// # Examples
    ///
    /// An always-on toggle that overlays a circle on top of a capsule.
    /// The circle inherits the capsule's size, minus 3 points of padding.
    ///
    /// ```
    /// use buoyant::view::prelude::*;
    /// use embedded_graphics::{prelude::*, pixelcolor::Rgb888};
    ///
    /// fn on_toggle() -> impl View<Rgb888, ()> {
    ///     Capsule
    ///         .foreground_color(Rgb888::GREEN)
    ///         .overlay(
    ///             Alignment::Trailing,
    ///             Circle
    ///                 .padding(Edges::All, 3)
    ///                 .foreground_color(Rgb888::WHITE)
    ///         )
    ///         .frame_sized(50, 25)
    /// }
    /// ```
    ///
    /// A more complex example using the alignment along with offset to draw a badge
    /// that is shifted outside the bounds of the content view:
    ///
    /// ```
    /// use buoyant::view::prelude::*;
    /// use embedded_graphics::{prelude::*, pixelcolor::Rgb888, mono_font::ascii::FONT_9X15_BOLD};
    ///
    /// fn notification_content() -> impl View<Rgb888, ()> {
    ///     Text::new("Content", &FONT_9X15_BOLD)
    ///         .overlay(Alignment::TopTrailing, capsule_badge("99+").offset(4, -4))
    /// }
    ///
    /// fn capsule_badge(label: &str) -> impl View<Rgb888, ()> + use<'_> {
    ///     Text::new(label, &FONT_9X15_BOLD)
    ///         .foreground_color(Rgb888::WHITE)
    ///         .padding(Edges::All, 4)
    ///         .background(Alignment::Center, Capsule.foreground_color(Rgb888::RED))
    /// }
    /// ```
    fn overlay<U>(self, alignment: Alignment, overlay: U) -> OverlayView<Self, U> {
        OverlayView::new(self, overlay, alignment)
    }

    /// Applies padding to the specified edges
    fn padding(self, edges: padding::Edges, amount: u32) -> Padding<Self> {
        Padding::new(edges, amount, self)
    }

    /// Sets the priority of the view layout.
    ///
    /// Stacks lay out views in groups of priority, with higher priority views being laid out
    /// first. Each set of views in the stack with a given priority are laid out together, with the
    /// stack offering the remaining width divided by the number of views in the group.
    fn priority(self, priority: i8) -> Priority<Self> {
        Priority::new(priority, self)
    }
}
