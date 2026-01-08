//! The view module provides a set of building blocks for creating user interfaces in Buoyant.
//!

#[cfg(feature = "embedded-graphics")]
mod as_drawable;
pub mod button;
mod capturing;
mod divider;
mod empty_view;
mod foreach;
mod geometry_reader;
mod hstack;
#[cfg(feature = "embedded-graphics")]
mod image;
#[allow(missing_docs)]
pub mod match_view;
mod modifier;
pub mod scroll_view;
pub mod shape;
mod spacer;
mod table;
mod text;
mod view_that_fits;
mod vstack;
mod zstack;

#[cfg(feature = "embedded-graphics")]
pub use as_drawable::AsDrawable;
pub use button::Button;
pub use capturing::Lens;
pub use divider::Divider;
pub use empty_view::EmptyView;
pub use foreach::ForEach;
pub use geometry_reader::GeometryReader;
pub use hstack::HStack;
#[cfg(feature = "embedded-graphics")]
pub use image::Image;
pub use modifier::aspect_ratio;
pub use modifier::padding;
pub use scroll_view::ScrollView;
pub use spacer::Spacer;
pub use table::{Table, TableAlgorithm, TableIndex, TableView};
pub(crate) use text::{CharacterWrap, WordWrap};
pub use text::{HorizontalTextAlignment, Text, WrapStrategy};
pub use view_that_fits::{FitAxis, ViewThatFits};
pub use vstack::VStack;
pub use zstack::ZStack;

/// A collection of commonly used types for building views.
pub mod prelude {
    pub use super::aspect_ratio::{ContentMode, Ratio};
    pub use super::modifier::ViewModifier;
    #[cfg(feature = "embedded-graphics")]
    pub use super::{AsDrawable, Image};
    pub use super::{
        Button, Divider, EmptyView, ForEach, GeometryReader, HStack, Lens, ScrollView, Spacer,
        Text, VStack, View, ViewLayout, ViewThatFits, ZStack,
    };
    pub use super::{FitAxis, HorizontalTextAlignment, padding::Edges};
    pub use crate::animation::Animation;
    pub use crate::layout::{Alignment, HorizontalAlignment, VerticalAlignment};
    pub use crate::view::shape::*;
}

use crate::transition::Transition;
use crate::{
    environment::LayoutEnvironment,
    event::{Event, EventContext, EventResult},
    layout::ResolvedLayout,
    primitives::{Point, ProposedDimensions},
    render::Render,
};

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
    type Renderables: Clone;

    /// The transition to use when this subtree appears or disappears
    type Transition: Transition;
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
    type State: Default + 'static;

    /// The computed layout of the view and its subviews.
    ///
    /// Size is represented here, but placement is deferred to the render tree.
    type Sublayout: Default + Clone + PartialEq + 'static;

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

    /// The transition to use when this view appears or disappears.
    fn transition(&self) -> Self::Transition;

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

    /// Process an event
    fn handle_event(
        &self,
        _event: &Event,
        _context: &EventContext,
        _render_tree: &mut Self::Renderables,
        _captures: &mut Captures,
        _state: &mut Self::State,
    ) -> EventResult {
        EventResult::default()
    }
}

impl<T> ViewMarker for &T
where
    T: ViewMarker,
{
    type Renderables = T::Renderables;
    type Transition = T::Transition;
}

impl<T, Captures: ?Sized> ViewLayout<Captures> for &T
where
    T: ViewLayout<Captures>,
{
    type State = T::State;
    type Sublayout = T::Sublayout;

    fn transition(&self) -> Self::Transition {
        (*self).transition()
    }

    fn build_state(&self, captures: &mut Captures) -> Self::State {
        (*self).build_state(captures)
    }

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        (*self).layout(offer, env, captures, state)
    }

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> Self::Renderables {
        (*self).render_tree(layout, origin, env, captures, state)
    }

    fn priority(&self) -> i8 {
        (*self).priority()
    }

    fn is_empty(&self) -> bool {
        (*self).is_empty()
    }

    fn handle_event(
        &self,
        event: &Event,
        context: &EventContext,
        render_tree: &mut Self::Renderables,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> EventResult {
        (*self).handle_event(event, context, render_tree, captures, state)
    }
}
