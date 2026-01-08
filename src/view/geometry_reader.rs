use core::marker::PhantomData;

use crate::{
    environment::LayoutEnvironment,
    event::{EventContext, EventResult},
    layout::ResolvedLayout,
    primitives::{Dimensions, Frame, ProposedDimensions, Size},
    render::Container,
    transition::Opacity,
    view::{Event, ViewLayout, ViewMarker},
};

/// Allows a view to read its own geometry for use in the construction of its children.
///
/// The layout behavior of this view may be surprising! It behaves *exactly* the same
/// as [`Rectangle`][`crate::view::shape::Rectangle`], which greedily expands to fill
/// all the offered space.
///
/// When proposed a [`Compact`] dimension, such as when used inside a [`ScrollView`] or
/// [`ViewModifier::fixed_size()`] modifier, this view will shrink to only 1 pixel along
/// the compact dimension because it has no "ideal" size.
/// [`GeometryReader`] does not clip its children to its bounds, so when the size
/// is misconfigured, the inner view may unexpectedly overlap other views.
///
/// The contents of the inner view have absolutely no bearing on the frame that
/// [`GeometryReader`] resolves to.
///
/// The [`ViewModifier::frame()`] and [`ViewModifier::flex_frame()`] modifiers are
/// good tools when applied outside the [`GeometryReader`] to control the size of
/// this view when it is presented in a compacting layout.
///
/// If the inner view is larger or smaller than the geometry reader's frame, it will be
/// placed with [`Alignment::TopLeading`] alignment.
///
/// Examples:
///
/// A capsule progress bar which is generic over color and captures.
///
/// This will take on a height of 10 pixels when presented under a [`ScrollView`],
/// but will expand to fill all the available space when used in a non-compacting layout.
///
/// ```
/// use buoyant::view::prelude::*;
/// use buoyant::primitives::Interpolate;
/// use embedded_graphics::pixelcolor::RgbColor;
///
/// fn progress_bar<Color: RgbColor + Interpolate, T>(progress: f32) -> impl View<Color, T> {
///     GeometryReader::new(move |size| {
///         Capsule
///             .frame_sized(size.width, size.height)
///             .foreground_color(Color::WHITE)
///             .overlay(
///                 Alignment::Leading,
///                 Capsule
///                     .foreground_color(Color::GREEN)
///                     .frame()
///                     .with_width((size.width as f32 * progress) as u32),
///             )
///     })
///     .flex_frame()
///     .with_ideal_height(10)
///     .with_min_width(20)
/// }
/// ```
///
/// [`Compact`]: crate::primitives::ProposedDimension::Compact
/// [`ViewModifier::fixed_size()`]: crate::view::ViewModifier::fixed_size
/// [`ViewModifier::frame()`]: crate::view::ViewModifier::frame()
/// [`ViewModifier::flex_frame()`]: crate::view::ViewModifier::flex_frame()
/// [`Alignment::TopLeading`]: crate::layout::Alignment::TopLeading
/// [`ScrollView`]: crate::view::ScrollView
#[derive(Debug, Clone)]
pub struct GeometryReader<ViewFn, Inner> {
    inner: ViewFn,
    _view_marker: PhantomData<Inner>,
}

impl<ViewFn: Fn(Size) -> Inner, Inner: ViewMarker> GeometryReader<ViewFn, Inner> {
    #[allow(missing_docs)]
    #[must_use]
    pub fn new(inner: ViewFn) -> Self {
        Self {
            inner,
            _view_marker: PhantomData,
        }
    }
}

impl<ViewFn, Inner: ViewMarker> ViewMarker for GeometryReader<ViewFn, Inner> {
    type Renderables = Container<Inner::Renderables>;
    type Transition = Opacity;
}

impl<Captures, Inner, ViewFn> ViewLayout<Captures> for GeometryReader<ViewFn, Inner>
where
    Captures: ?Sized,
    Inner: ViewLayout<Captures>,
    ViewFn: Fn(Size) -> Inner,
{
    type State = Option<Inner::State>;
    type Sublayout = Dimensions;

    fn transition(&self) -> Self::Transition {
        Opacity
    }

    fn build_state(&self, _captures: &mut Captures) -> Self::State {
        None
    }

    fn layout(
        &self,
        offer: &ProposedDimensions,
        _env: &impl LayoutEnvironment,
        _captures: &mut Captures,
        _state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        // Defer actual layout to the render_tree call when we have the final size to avoid
        // doing wasted work.
        let size = offer.resolve_most_flexible(0, 1);
        ResolvedLayout {
            resolved_size: size,
            sublayouts: size,
        }
    }

    fn render_tree(
        &self,
        layout: &Self::Sublayout,
        origin: crate::primitives::Point,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> Self::Renderables {
        let size = (*layout).into();
        let view = (self.inner)(size);
        let proposal = (*layout).into();
        let frame = Frame::new(origin, size);
        if let Some(inner_state) = state {
            let layout = view.layout(&proposal, env, captures, inner_state);
            Container::new(
                frame,
                view.render_tree(&layout.sublayouts, origin, env, captures, inner_state),
            )
        } else {
            let mut inner_state = view.build_state(captures);
            let layout = view.layout(&proposal, env, captures, &mut inner_state);
            let renderables =
                view.render_tree(&layout.sublayouts, origin, env, captures, &mut inner_state);
            *state = Some(inner_state);
            Container::new(frame, renderables)
        }
    }

    fn handle_event(
        &self,
        event: &Event,
        context: &EventContext,
        render_tree: &mut Self::Renderables,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> EventResult {
        // TODO: Rebuilding the view here seems inefficient, maybe cache the view in the state?
        let view = (self.inner)(render_tree.frame.size);
        if let Some(inner_state) = state {
            view.handle_event(
                event,
                context,
                &mut render_tree.child,
                captures,
                inner_state,
            )
        } else {
            let mut inner_state = view.build_state(captures);
            let result = view.handle_event(
                event,
                context,
                &mut render_tree.child,
                captures,
                &mut inner_state,
            );
            *state = Some(inner_state);
            result
        }
    }
}
