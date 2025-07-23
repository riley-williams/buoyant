//! The view module provides a set of building blocks for creating user interfaces in Buoyant.
//!

mod button;
mod capturing;
mod divider;
mod empty_view;
mod foreach;
mod hstack;
#[cfg(feature = "embedded-graphics")]
mod image;
#[allow(missing_docs)]
pub mod match_view;
mod modifier;
/// Shape primitives
pub mod shape;
mod spacer;
mod text;
mod view_that_fits;
mod vstack;
mod zstack;

pub use button::Button;
pub use capturing::Lens;
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

/// A collection of commonly used types for building views.
pub mod prelude {
    pub use super::aspect_ratio::{ContentMode, Ratio};
    pub use super::{padding::Edges, FitAxis, HorizontalTextAlignment};
    #[cfg(feature = "embedded-graphics")]
    pub use super::{AsDrawable, Image};
    pub use super::{
        Button, Divider, EmptyView, ForEach, HStack, Lens, Spacer, Text, VStack, View, ViewExt,
        ViewLayout, ViewThatFits, ZStack,
    };
    pub use crate::animation::Animation;
    pub use crate::layout::{Alignment, HorizontalAlignment, VerticalAlignment};
    pub use crate::view::shape::*;
}

use modifier::{
    Animated, AspectRatio, BackgroundView, EraseCaptures, FixedFrame, FixedSize, FlexFrame,
    ForegroundStyle, GeometryGroup, Hidden, Offset, OverlayView, Padding, Priority,
};

use crate::{
    animation::Animation,
    environment::LayoutEnvironment,
    event::Event,
    layout::{Alignment, HorizontalAlignment, ResolvedLayout, VerticalAlignment},
    primitives::{Point, ProposedDimensions},
    render::Render,
};

#[cfg(feature = "embedded-graphics")]
use crate::{environment::DefaultEnvironment, primitives::Interpolate};

/// A view that can be rendered with a specific color type.
///
/// The first generic, `Color`, is the pixel color type used for rendering (e.g., `Rgb888`, `Rgb565`).
/// The second, `Captures`, refers to external mutable state that the view can access when handling events.
///
/// # Examples
///
/// A simple view that renders a red rectangle:
///
/// ```
/// use buoyant::view::prelude::*;
/// use embedded_graphics::pixelcolor::{Rgb888, RgbColor};
///
/// fn red_rectangle() -> impl View<Rgb888, ()> {
///     Rectangle.foreground_color(Rgb888::RED)
/// }
/// ```
pub trait View<Color, Captures: ?Sized>: ViewLayout<Captures, Renderables: Render<Color>> {}

impl<T, Color, Captures: ?Sized> View<Color, Captures> for T where
    Self: ViewLayout<Captures, Renderables: Render<Color>>
{
}

/// A marker trait for all views, independent of color or captures.
///
/// View extension traits can be implemented for all `T: ViewMarker` to prevent issues arising from
/// ambiguity around color and captures generics of [`View`].
pub trait ViewMarker: Sized {
    /// The renderable output of this view.
    ///
    /// This is a concrete snapshot of the view which can be drawn to a render target and
    /// interpolated.
    type Renderables;
}

/// Layout and state management behavior for views.
///
/// This trait defines the core functionality of all views, including state management,
/// layout calculation, producing a render tree, and event handling.
///
/// It's not generally necessary or recommended to implement this trait directly. Most views
/// can be built by composing existing views and applying modifiers.
pub trait ViewLayout<Captures: ?Sized>: ViewMarker {
    /// The internal state that is maintained between layout and render cycles.
    ///
    /// This state is created once when the view is first initialized and is intended
    /// to persist across multiple layout/render cycles.
    type State;

    /// The computed layout of the view and its subviews.
    ///
    /// Size is represented here, but placement is deferred to the render tree.
    type Sublayout: Clone + PartialEq;

    /// The layout priority of the view. Higher priority views are more likely to
    /// be given the size they want
    fn priority(&self) -> i8 {
        0
    }

    /// Returns true if the view should not included in layout.
    ///
    /// Stacks will not add spacing around this view, and it should not be rendered.
    fn is_empty(&self) -> bool {
        false
    }

    /// Constructs a default initial state before a view is presented.
    ///
    /// This should be called once after view init, but may be called again if new
    /// view subtrees are created. This most commonly happens due to the branches of
    /// [`match_view`][crate::match_view!] or [`if_view`][crate::if_view!] changing.
    fn build_state(&self, captures: &mut Captures) -> Self::State;

    /// Computes the size of the view, given the offer
    ///
    /// Generally, views should attempt to produce layouts that fit within the offer.
    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout>;

    /// Creates the render tree for this view based on the resolved layout.
    ///
    /// This method is called after layout to place views and produce the actual
    /// renderable objects that will be drawn to the screen.
    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> Self::Renderables;

    /// Process an event, returning true if the event was handled.
    fn handle_event(
        &mut self,
        _event: &Event,
        _render_tree: &mut Self::Renderables,
        _captures: &mut Captures,
        _state: &mut Self::State,
    ) -> bool {
        false
    }
}

/// Modifiers that extend the functionality of views.
pub trait ViewExt: Sized {
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
    /// See [`ViewExt::geometry_group`] for creating correct compound animations.
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
    /// Where possible, prefer increasing the [`priority`][`ViewExt::priority`] of
    /// the problematic text view subtree in the stack as [`ViewExt::fixed_size`]
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
    /// Contrary to what intuition might suggest, simply moving the [`.animated`][a] modifier
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
    /// [a]: ViewExt::animated
    fn geometry_group(self) -> GeometryGroup<Self> {
        GeometryGroup::new(self)
    }

    /// Lays out the view, but does not render it.
    ///
    /// The `.hidden()` modifier is occasionally useful for creating workarounds (read: hacks)
    /// that produce otherwise impossible layouts. It is typically used in combination with
    /// [`ViewExt::background`] or [`ZStack`].
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

impl<T> ViewExt for T where T: ViewMarker {}

/// A view that can be converted into an embedded-graphics drawable.
#[cfg(feature = "embedded-graphics")]
pub trait AsDrawable<Color, Captures: ?Sized> {
    #[allow(clippy::doc_markdown)]
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
    fn as_drawable(
        &self,
        size: impl Into<ProposedDimensions>,
        default_color: Color,
        captures: &mut Captures,
    ) -> impl embedded_graphics_core::Drawable<Color = Color, Output = ()>;
}

#[cfg(feature = "embedded-graphics")]
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

#[cfg(feature = "embedded-graphics")]
struct DrawableView<T, C> {
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
