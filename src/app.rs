//! [`App`] manages the view/render tree lifecycle and event handling.
//!
//! The [`Harness`] trait provides a set of convenience methods for navigating
//! and interacting with the UI, with default implementations built on top of
//! [`Harness::send`] and [`Harness::send_with_group`].

use core::time::Duration;

use crate::{
    environment::DefaultEnvironment,
    event::{Event, EventContext, EventResult},
    focus::{FocusTree, Role, RoleSet},
    primitives::{Point, Size, transform::LinearTransform},
    render::{AnimatedJoin, AnimationDomain, ContentShape, Render},
    render_target::{RenderTarget, SolidBrush, Stroke},
    view::{View, ViewLayout},
};

mod harness;
pub use harness::Harness;

mod tracked;
pub use tracked::Tracked;

mod trees;
pub use trees::Trees;

/// Manages the view/render tree lifecycle, focus state, and event handling.
///
/// [`App`] owns the view function, application state, and the source/target render
/// trees needed for animated transitions.
///
/// The [`Harness`] trait is implemented to provide a set of convenience methods for
/// sending events.
///
/// # Rendering
///
///
pub struct App<V, S, F>
where
    V: ViewLayout<S>,
{
    /// The application state
    state: S,
    view_fn: F,
    view: V,
    view_state: V::State,
    trees: Trees<V::Renderables>,
    focus_tree: V::FocusTree,
    env: DefaultEnvironment,
    /// Current focus shape for overlay rendering.
    focus_shape: ContentShape,
    roles: RoleSet,
    size: Size,
    elapsed: Duration,
    requires_redraw: bool,
    requires_rebuild: bool,
}

impl<V, S, F> core::fmt::Debug for App<V, S, F>
where
    V: ViewLayout<S> + core::fmt::Debug,
    V::State: core::fmt::Debug,
    V::Renderables: core::fmt::Debug,
    V::FocusTree: core::fmt::Debug,
    S: core::fmt::Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("App")
            .field("state", &self.state)
            .field("env", &self.env)
            .field("focus_shape", &self.focus_shape)
            .field("roles", &self.roles)
            .field("size", &self.size)
            .field("elapsed", &self.elapsed)
            .field("requires_redraw", &self.requires_redraw)
            .field("requires_rebuild", &self.requires_rebuild)
            .finish_non_exhaustive()
    }
}

impl<V, S, F> App<V, S, F>
where
    V: ViewLayout<S>,
    V::FocusTree: FocusTree,
    V::Renderables: AnimatedJoin,
    F: Fn(&S) -> V,
{
    /// Creates a new `App` with the given initial state, display size, and view function.
    pub fn new(state: S, size: Size, view_fn: F) -> Self
    where
        S: 'static,
    {
        let view = (view_fn)(&state);
        let env = DefaultEnvironment::non_animated();
        let mut state = state;
        let mut view_state = view.build_state(&mut state);
        let layout = view.layout(&size.into(), &env, &mut state, &mut view_state);
        let source_tree = view.render_tree(
            &layout.sublayouts,
            Point::zero(),
            &env,
            &mut state,
            &mut view_state,
        );
        let target_tree = view.render_tree(
            &layout.sublayouts,
            Point::zero(),
            &env,
            &mut state,
            &mut view_state,
        );
        let focus_tree = FocusTree::default_first();
        let roles = Role::Button | Role::Container;

        Self {
            state,
            view_fn,
            view,
            view_state,
            trees: Trees::new(source_tree, target_tree),
            focus_tree,
            env,
            roles,
            focus_shape: ContentShape::Empty,
            size,
            elapsed: Duration::default(),
            requires_redraw: true,
            requires_rebuild: false,
        }
    }

    /// Sets the default roles for focus navigation.
    ///
    /// Roles determine which types of focusable elements (buttons, containers, etc.)
    /// respond to focus events.
    #[must_use]
    pub fn with_roles(mut self, roles: impl Into<RoleSet>) -> Self {
        self.roles = roles.into();
        self
    }

    /// Initializes the focus tree to the last element.
    #[must_use]
    pub fn with_focus_at_end(mut self) -> Self {
        self.focus_tree = FocusTree::default_last();
        self
    }

    /// Returns a reference to the application state.
    #[must_use]
    pub fn state(&self) -> &S {
        &self.state
    }

    /// Returns a mutable reference to the application state.
    ///
    /// The state is wrapped in [`Tracked`] which will request a view rebuild on your behalf when
    /// accessing the inner value mutably.
    #[must_use]
    pub fn state_mut(&mut self) -> Tracked<'_, S> {
        Tracked::new(&mut self.state, &mut self.requires_rebuild)
    }

    /// The current display size.
    #[must_use]
    pub fn size(&self) -> Size {
        self.size
    }

    /// The current elapsed virtual time.
    #[must_use]
    pub fn elapsed(&self) -> Duration {
        self.elapsed
    }

    /// The content shape of the currently focused element.
    #[must_use]
    pub fn focus_shape(&self) -> &ContentShape {
        &self.focus_shape
    }

    /// Returns true if any element in the view currently holds focus.
    #[must_use]
    pub fn is_focused(&self) -> bool {
        self.focus_tree.is_focused()
    }

    /// Returns mutable references to both source and target render trees.
    #[must_use]
    pub fn render_trees(&self) -> &Trees<V::Renderables> {
        &self.trees
    }

    /// Returns mutable references to both source and target render trees.
    #[must_use]
    pub fn render_trees_mut(&mut self) -> &mut Trees<V::Renderables> {
        &mut self.trees
    }

    /// Advances the virtual clock by the given duration.
    ///
    /// Prefer setting time directly with [`set_time`](Self::set_time).
    pub fn advance_time(&mut self, duration: Duration) {
        self.elapsed += duration;
    }

    /// Sets the elapsed time since app start. This value is used to drive animation.
    pub fn set_time(&mut self, time: Duration) {
        self.elapsed = time;
    }

    /// Rebuilds the view and render trees.
    ///
    /// This is typically called automatically when an event triggers a view rebuild, but
    /// it may be necessary to call manually for state changes not tracked by Buoyant.
    ///
    /// See also: [`finalize_view`](Self::finalize_view).
    pub fn force_rebuild(&mut self) {
        self.requires_rebuild = false;

        let domain = AnimationDomain::top_level(self.elapsed);
        self.trees.swap();
        let (source_tree, target_tree) = self.trees.both_mut();
        source_tree.join_from(target_tree, &domain);

        // Create new view and target tree
        self.view = (self.view_fn)(&self.state);
        self.env = DefaultEnvironment::new(self.elapsed);

        let layout = self.view.layout(
            &self.size.into(),
            &self.env,
            &mut self.state,
            &mut self.view_state,
        );

        *target_tree = self.view.render_tree(
            &layout.sublayouts,
            Point::zero(),
            &self.env,
            &mut self.state,
            &mut self.view_state,
        );

        self.requires_redraw = true;
    }

    /// Updates the display size.
    pub fn resize(&mut self, new_size: Size) {
        self.size = new_size;
        self.requires_rebuild = true;
    }

    /// Rebuilds the view and render trees if it was requested as a result of event
    /// handling or tracked state changes.
    ///
    /// If no rebuild is pending, this method does nothing.
    pub fn finalize_view(&mut self) {
        if self.requires_rebuild {
            self.force_rebuild();
        }
    }

    /// Renders the view to the given render target.
    ///
    /// If a rebuild is pending, it will be performed before rendering.
    /// Rebuilds can be eagerly triggered by calling [`rebuild()`](Self::rebuild()).
    pub fn render_animated<T, C>(&mut self, target: &mut T, color: &C)
    where
        V: View<C, S>,
        T: RenderTarget<ColorFormat = C>,
    {
        self.finalize_view();

        let domain = AnimationDomain::top_level(self.elapsed);
        Render::render_animated(
            target,
            self.trees.source(),
            self.trees.target(),
            color,
            &domain,
        );
        self.requires_redraw = false;
    }

    /// Whether the view needs to be redrawn due to changes in state or focus.
    pub fn should_redraw(&mut self) -> bool {
        self.requires_redraw || self.requires_rebuild
    }

    /// Draws the focus as a stroked shape on the given render target.
    pub fn draw_focus_overlay<T, C>(&self, target: &mut T, color: C, line_width: u32)
    where
        T: RenderTarget<ColorFormat = C>,
        C: Copy,
    {
        let stroke = Stroke::new(line_width);
        let brush = SolidBrush::new(color);
        match &self.focus_shape {
            ContentShape::Rectangle(rect) => {
                target.stroke(&stroke, LinearTransform::identity(), &brush, None, rect);
            }
            ContentShape::RoundedRectangle(rrect) => {
                target.stroke(&stroke, LinearTransform::identity(), &brush, None, rrect);
            }
            ContentShape::Circle(circle) => {
                target.stroke(&stroke, LinearTransform::identity(), &brush, None, circle);
            }
            ContentShape::Empty => (),
        }
    }

    /// Updates the focus shape from an event result.
    fn update_focus_shape(&mut self, result: &EventResult) {
        if let EventResult::Handled { shape, .. } = result {
            self.focus_shape = shape.clone();
        }
    }
}

impl<V, S, F> Harness for App<V, S, F>
where
    V: ViewLayout<S>,
    V::FocusTree: FocusTree,
    V::Renderables: AnimatedJoin,
    F: Fn(&S) -> V,
{
    fn send(&mut self, event: impl Into<Event>) -> EventResult {
        let context = EventContext::new(self.elapsed).with_roles(self.roles);

        let event = event.into();
        let target_tree = self.trees.target_mut();

        let result = self.view.handle_event(
            &event,
            &context,
            target_tree,
            &mut self.state,
            &mut self.view_state,
            &mut self.focus_tree,
        );

        self.update_focus_shape(&result);

        if context.view_rebuild_requested.get() {
            self.requires_rebuild = true;
        }
        if context.redraw_requested.get() {
            self.requires_redraw = true;
        }
        result
    }
}
