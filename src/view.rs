//! View types used for building interfaces

mod divider;
mod empty_view;
mod foreach;
mod hstack;
#[cfg(feature = "embedded-graphics")]
mod image;
pub mod match_view;
mod modifier;
pub mod shape;
mod spacer;
mod text;
mod view_that_fits;
mod vstack;
mod zstack;

pub use divider::Divider;
pub use empty_view::EmptyView;
pub use foreach::ForEach;
pub use hstack::HStack;
#[cfg(feature = "embedded-graphics")]
pub use image::Image;
pub use modifier::aspect_ratio;
pub use modifier::padding;
pub use spacer::Spacer;
pub(crate) use text::WhitespaceWrap;
pub use text::{HorizontalTextAlignment, Text};
pub use view_that_fits::{FitAxis, ViewThatFits};
pub use vstack::VStack;
pub use zstack::ZStack;

pub mod prelude {
    pub use super::aspect_ratio::{ContentMode, Ratio};
    pub use super::{padding::Edges, FitAxis, HorizontalTextAlignment};
    pub use super::{
        shape::*, Divider, EmptyView, ForEach, HStack, Spacer, Text, VStack, View, ViewExt,
        ViewThatFits, ZStack,
    };
    #[cfg(feature = "embedded-graphics")]
    pub use super::{AsDrawable, Image};
    pub use crate::layout::{Alignment, HorizontalAlignment, VerticalAlignment};
}

use modifier::{
    Animated, AspectRatio, BackgroundView, FixedFrame, FixedSize, FlexFrame, ForegroundStyle,
    GeometryGroup, Hidden, Offset, OverlayView, Padding, Priority,
};

use crate::{
    animation::Animation,
    layout::{Alignment, HorizontalAlignment, VerticalAlignment},
    primitives::Point,
    render::{Render, Renderable},
};

#[cfg(feature = "embedded-graphics")]
use crate::{
    environment::DefaultEnvironment,
    primitives::{Interpolate, ProposedDimensions},
};

pub trait View<C>: Renderable<Renderables: Render<C>> {}
impl<C, T: Renderable<Renderables: Render<C>>> View<C> for T {}

/// Modifiers that extend the functionality of views.
pub trait ViewExt: Sized {
    /// Constrains the dimensions to the specified aspect ratio.
    ///
    /// # Examples
    ///
    /// ```
    /// # use buoyant::view::prelude::*;
    /// # use embedded_graphics::pixelcolor::Rgb565;
    /// # use embedded_graphics::prelude::RgbColor;
    ///
    /// // A 16:9 aspect ratio rectangle that will scale to fit the available space
    /// fn widescreen_rectangle() -> impl View<Rgb565> {
    ///     Rectangle
    ///         .aspect_ratio(
    ///             Ratio::Fixed(16, 9),
    ///             ContentMode::Fit
    ///         )
    /// }
    ///
    /// // Ideal aspect ratio maintains the child's ideal aspect ratio
    ///
    /// /// An icon that maintains its aspect ratio while fitting within a 100x100 area
    /// fn profile_icon() -> impl View<Rgb565> {
    ///     image()
    ///         .aspect_ratio(Ratio::Ideal, ContentMode::Fit)
    ///         .flex_frame()
    ///         .with_max_size(100, 100)
    /// }
    ///
    /// /// (Equivalent to) A flexible 2:3 ratio image
    /// fn image() -> impl View<Rgb565> {
    ///     Rectangle
    ///         .flex_frame()
    ///         .with_ideal_size(40, 60)
    /// }
    ///
    /// ```
    fn aspect_ratio(
        self,
        aspect_ratio: aspect_ratio::Ratio,
        content_mode: aspect_ratio::ContentMode,
    ) -> AspectRatio<Self> {
        AspectRatio::new(self, aspect_ratio, content_mode)
    }

    /// Applies padding to the specified edges
    fn padding(self, edges: padding::Edges, amount: u32) -> Padding<Self> {
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
    /// # use buoyant::view::ViewExt as _;
    /// # let my_view = buoyant::view::shape::Rectangle;
    /// # let width = 100;
    /// # let height = 100;
    /// my_view
    ///     .frame()
    ///     .with_width(width)
    ///     .with_height(height)
    /// # ;
    /// ```
    fn frame_sized(self, width: u32, height: u32) -> FixedFrame<Self> {
        FixedFrame::new(self).with_width(width).with_height(height)
    }

    /// A virtual frame that can be configured with flexible dimensions.
    ///
    /// Examples:
    ///
    /// ```
    /// # use buoyant::view::ViewExt as _;
    /// # use buoyant::layout::Alignment;
    /// # let my_view = buoyant::view::shape::Rectangle;
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

    /// Creates a virtual frame that expands to fill as much horizontal space as possible.
    ///
    /// This is a shortcut for:
    ///
    /// ```
    /// # use buoyant::view::ViewExt as _;
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
    /// # use buoyant::view::ViewExt as _;
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
            .with_infinite_max_height()
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
    /// use buoyant::view::{shape::{Circle, Capsule}, ZStack, padding::Edges, View, ViewExt as _};
    /// use buoyant::animation::Animation;
    /// use buoyant::layout::HorizontalAlignment;
    /// use buoyant::render::Renderable;
    /// use embedded_graphics::pixelcolor::Rgb565;
    /// use embedded_graphics::prelude::*;
    ///
    /// fn toggle_button(is_on: bool) -> impl View<Rgb565> {
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
    fn geometry_group(self) -> GeometryGroup<Self> {
        GeometryGroup::new(self)
    }

    /// Background uses the layout of the foreground view and renders the background
    /// behind it.
    ///
    /// Example:
    ///
    /// ```
    /// use buoyant::view::{padding::Edges, shape::RoundedRectangle, Text, View, ViewExt as _};
    /// use buoyant::layout::Alignment;
    /// use embedded_graphics::{prelude::*, pixelcolor::Rgb565, mono_font::ascii::FONT_9X15_BOLD};
    ///
    /// fn bordered_button() -> impl View<Rgb565> {
    ///     Text::new("Press me", &FONT_9X15_BOLD)
    ///         .foreground_color(Rgb565::WHITE)
    ///         .padding(Edges::All, 10)
    ///         .background(Alignment::default(), || {
    ///             RoundedRectangle::new(10)
    ///                 .foreground_color(Rgb565::BLUE)
    ///         })
    /// }
    /// ```
    fn background<U>(
        self,
        alignment: Alignment,
        background: impl FnOnce() -> U,
    ) -> BackgroundView<Self, U> {
        BackgroundView::new(self, background(), alignment)
    }

    /// Overlay uses the modified view to compute bounds, and renders the overlay
    /// on top.
    ///
    /// Example:
    ///
    /// ```
    /// use buoyant::view::prelude::*;
    /// use embedded_graphics::{prelude::*, pixelcolor::Rgb888, mono_font::ascii::FONT_9X15_BOLD};
    ///
    /// fn notification_badge() -> impl View<Rgb888> {
    ///     Text::new("Content", &FONT_9X15_BOLD)
    ///         .overlay(
    ///             Alignment::TopTrailing,
    ///             Text::new("!", &FONT_9X15_BOLD)
    ///                 .foreground_color(Rgb888::WHITE)
    ///                 .padding(Edges::All, 4)
    ///                 .background(Alignment::Center, || Circle.foreground_color(Rgb888::RED))
    ///                 .offset(4, -4)
    ///         )
    /// }
    /// ```
    fn overlay<U>(self, alignment: Alignment, overlay: U) -> OverlayView<Self, U> {
        OverlayView::new(self, overlay, alignment)
    }

    /// Offsets a view by the specified values.
    ///
    /// This does not affect size calculations, and is only applied when rendering the view.
    fn offset(self, x: i32, y: i32) -> Offset<Self> {
        Offset::new(self, Point::new(x, y))
    }

    /// Lays out the view, but does not render it.
    ///
    /// The `.hidden()` modifier is occasionally useful for creating workarounds (read: hacks)
    /// that produce otherwise impossible layouts. It is typically used in combination with
    /// `.background(|| ...)` or [`ZStack`].
    ///
    /// This is intentionally not configurable with, e.g. `.hidden(true/false)`,
    /// as it entirely prunes the subtree from the render tree type, resulting in no additional
    /// memory or computation during rendering / animation.
    fn hidden(self) -> Hidden<Self> {
        Hidden::new(self)
    }

    /// Sets the foreground color
    fn foreground_color<C>(self, color: C) -> ForegroundStyle<Self, C> {
        ForegroundStyle::new(color, self)
    }
}

impl<T: crate::layout::Layout> ViewExt for T {}

#[deprecated(
    since = "0.5.0",
    note = "`RenderExtensions` has been replaced with `ViewExt`"
)]
pub trait RenderExtensions: ViewExt {
    fn foreground_color<C>(self, color: C) -> ForegroundStyle<Self, C> {
        <Self as ViewExt>::foreground_color(self, color)
    }
}

#[allow(deprecated)]
impl<T: crate::layout::Layout> RenderExtensions for T {}

#[deprecated(
    since = "0.5.0",
    note = "`LayoutExtensions` has been replaced with `ViewExt`"
)]
pub trait LayoutExtensions: ViewExt {
    fn padding(self, edges: padding::Edges, amount: u32) -> Padding<Self> {
        <Self as ViewExt>::padding(self, edges, amount)
    }

    fn frame(self) -> FixedFrame<Self> {
        <Self as ViewExt>::frame(self)
    }

    fn frame_sized(self, width: u32, height: u32) -> FixedFrame<Self> {
        <Self as ViewExt>::frame_sized(self, width, height)
    }

    fn flex_frame(self) -> FlexFrame<Self> {
        <Self as ViewExt>::flex_frame(self)
    }

    fn flex_infinite_width(self, alignment: HorizontalAlignment) -> FlexFrame<Self> {
        <Self as ViewExt>::flex_infinite_width(self, alignment)
    }

    fn flex_infinite_height(self, alignment: VerticalAlignment) -> FlexFrame<Self> {
        <Self as ViewExt>::flex_infinite_height(self, alignment)
    }

    fn fixed_size(self, horizontal: bool, vertical: bool) -> FixedSize<Self> {
        <Self as ViewExt>::fixed_size(self, horizontal, vertical)
    }

    fn priority(self, priority: i8) -> Priority<Self> {
        <Self as ViewExt>::priority(self, priority)
    }

    fn animated<T: PartialEq + Clone>(self, animation: Animation, value: T) -> Animated<Self, T> {
        <Self as ViewExt>::animated(self, animation, value)
    }

    fn geometry_group(self) -> GeometryGroup<Self> {
        <Self as ViewExt>::geometry_group(self)
    }

    fn background<U>(
        self,
        alignment: Alignment,
        background: impl FnOnce() -> U,
    ) -> BackgroundView<Self, U> {
        <Self as ViewExt>::background(self, alignment, background)
    }

    fn hidden(self) -> Hidden<Self> {
        <Self as ViewExt>::hidden(self)
    }
}

#[allow(deprecated)]
impl<T: crate::layout::Layout> LayoutExtensions for T {}

/// Convert a view into an object that can be drawn with embedded-graphics.
///
/// This trait provides a convenient way to draw Buoyant views directly using the embedded-graphics
/// drawing API, without manually handling layout and rendering stages.
///
/// # Example
///
/// ```rust
/// # use buoyant::view::{AsDrawable as _, Text, ViewExt as _};
/// # use embedded_graphics::{mono_font::ascii::FONT_10X20, pixelcolor::Rgb888, prelude::*};
/// # use embedded_graphics_simulator::{OutputSettings, SimulatorDisplay, Window};
///
/// let mut display: SimulatorDisplay<Rgb888> = SimulatorDisplay::new(Size::new(480, 320));
///
/// // Create a simple view
/// let view = Text::new("Hello Buoyant!", &FONT_10X20)
///     .foreground_color(Rgb888::GREEN);
///
/// // Draw the view directly to the display using AsDrawable
/// view.as_drawable(display.size(), Rgb888::BLACK)
///     .draw(&mut display)
///     .unwrap();
/// ```
#[cfg(feature = "embedded-graphics")]
pub trait AsDrawable<Color> {
    fn as_drawable(
        &self,
        size: impl Into<ProposedDimensions>,
        default_color: Color,
    ) -> impl embedded_graphics_core::Drawable<Color = Color, Output = ()>;
}

#[cfg(feature = "embedded-graphics")]
impl<C: embedded_graphics_core::pixelcolor::PixelColor + Interpolate, T: View<C>> AsDrawable<C>
    for T
{
    fn as_drawable(
        &self,
        size: impl Into<ProposedDimensions>,
        default_color: C,
    ) -> impl embedded_graphics_core::Drawable<Color = C, Output = ()> {
        let env = DefaultEnvironment::non_animated();
        let layout = self.layout(&size.into(), &env);
        let render_tree = self.render_tree(&layout, Point::zero(), &env);
        DrawableView {
            render_tree,
            default_color,
        }
    }
}

#[cfg(feature = "embedded-graphics")]
struct DrawableView<T: Render<C>, C> {
    render_tree: T,
    default_color: C,
}

#[cfg(feature = "embedded-graphics")]
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
            .render(&mut embedded_target, &self.default_color, Point::zero());
        Ok(())
    }
}
