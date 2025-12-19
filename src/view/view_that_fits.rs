use crate::{
    environment::LayoutEnvironment,
    event::EventResult,
    layout::ResolvedLayout,
    primitives::{Point, ProposedDimension, ProposedDimensions},
    render,
    transition::Opacity,
    view::{ViewLayout, ViewMarker},
};

use super::match_view::{Branch2, Branch3, Branch4};

/// The axis along which the view should be fit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FitAxis {
    /// Use the first view with a compact height that fits vertically.
    Vertical,
    /// Use the first view with a compact width that fits horizontally.
    Horizontal,
    /// Use the first view with a compact size that fits the space offered
    Both,
}

impl FitAxis {
    #[must_use]
    const fn components(self) -> (bool, bool) {
        match self {
            Self::Vertical => (false, true),
            Self::Horizontal => (true, false),
            Self::Both => (true, true),
        }
    }
}

/// Picks the first view that fits the available space by comparing each view's
/// preferred size to the available space.
///
/// If no other view fits, the last view is used.
///
/// Unlike other conditional views, [`ViewThatFits`] does not pass through the
/// empty and priority properties of its children.
///
/// If the branch which fits changes, the state will be rebuilt.
///
/// # Examples
///
/// ```
/// # use embedded_graphics::mono_font::MonoFont;
/// # const FONT: MonoFont<'_> = embedded_graphics::mono_font::ascii::FONT_5X8;
/// use buoyant::view::{ViewThatFits, FitAxis, Text};
///
/// let charge_view = || {
///     ViewThatFits::new(FitAxis::Vertical, {
///         Text::new("12 hours, 16 minutes, and 3 seconds to full charge", &FONT)
///     })
///     .or(Text::new("12h 16m 3s remaining", &FONT))
///     .or(Text::new("12h ⚡️", &FONT))
/// };
/// ```
///
/// In units of character size, this could produce the following results
/// for the given offered size:
///
/// > **100x1**
/// >
/// > 12 hours, 16 minutes, and 3 seconds to full charge
///
/// > **16x5**
/// >
/// > 12 hours, 16
/// > minutes, and 3
/// > seconds to full
/// > charge
///
/// > **10x2**
/// >
/// > 12h 16m 3s
/// > remaining
///
/// > **5x1**
/// >
/// > 12h ⚡️
#[derive(Debug, Clone)]
pub struct ViewThatFits<T> {
    axis: FitAxis,
    choices: T,
}

impl<T> ViewThatFits<(T,)> {
    #[allow(missing_docs)]
    #[must_use]
    pub const fn new(axis: FitAxis, view: T) -> Self {
        Self {
            axis,
            choices: (view,),
        }
    }
}

impl<T> ViewThatFits<(T,)> {
    /// An alternative view to use if the first one does not fit.
    #[must_use]
    pub fn or<V>(self, alternate: V) -> ViewThatFits<(T, V)> {
        ViewThatFits {
            axis: self.axis,
            choices: (self.choices.0, alternate),
        }
    }
}

macro_rules! derive_or {
    ($(($n:tt, $type:ident)),*) => {
        impl<T0, $($type),*> ViewThatFits<(T0, $($type),*)> {
            /// An alternative view to use if the first one does not fit.
            #[must_use]
            pub fn or<V>(self, alternate: V) -> ViewThatFits<(T0, $($type),*, V)> {
                ViewThatFits {
                    axis: self.axis,
                    choices: (self.choices.0, $(self.choices.$n),*, alternate),
                }
            }
        }
    };
}

// 4 is probably enough...? Making macros for this seems tricky
derive_or!((1, T1));
derive_or!((1, T1), (2, T2)); // this derives the 4-tuple variant

impl<T: ViewMarker> ViewMarker for ViewThatFits<(T,)> {
    type Renderables = T::Renderables;
    type Transition = Opacity;
}

impl<T, Captures: ?Sized> ViewLayout<Captures> for ViewThatFits<(T,)>
where
    T: ViewLayout<Captures>,
{
    type Sublayout = T::Sublayout;
    type State = T::State;

    fn transition(&self) -> Self::Transition {
        Opacity
    }

    fn build_state(&self, captures: &mut Captures) -> Self::State {
        self.choices.0.build_state(captures)
    }

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        self.choices.0.layout(offer, env, captures, state)
    }

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> Self::Renderables {
        self.choices
            .0
            .render_tree(layout, origin, env, captures, state)
    }

    fn handle_event(
        &self,
        event: &crate::view::Event,
        context: &crate::event::EventContext,
        render_tree: &mut Self::Renderables,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> EventResult {
        self.choices
            .0
            .handle_event(event, context, render_tree, captures, state)
    }
}

const fn make_compact_offer(from_offer: ProposedDimensions, axis: FitAxis) -> ProposedDimensions {
    match axis {
        FitAxis::Vertical => ProposedDimensions {
            width: from_offer.width,
            height: ProposedDimension::Compact,
        },
        FitAxis::Horizontal => ProposedDimensions {
            width: ProposedDimension::Compact,
            height: from_offer.height,
        },
        FitAxis::Both => ProposedDimensions {
            width: ProposedDimension::Compact,
            height: ProposedDimension::Compact,
        },
    }
}

impl<T0, T1> ViewMarker for ViewThatFits<(T0, T1)>
where
    T0: ViewMarker,
    T1: ViewMarker,
{
    type Renderables = render::OneOf2<T0::Renderables, T1::Renderables>;
    type Transition = Opacity;
}

impl<T0, T1, Captures: ?Sized> ViewLayout<Captures> for ViewThatFits<(T0, T1)>
where
    T0: ViewLayout<Captures>,
    T1: ViewLayout<Captures>,
{
    type Sublayout = Branch2<ResolvedLayout<T0::Sublayout>, ResolvedLayout<T1::Sublayout>>;
    type State = Branch2<T0::State, T1::State>;

    fn transition(&self) -> Self::Transition {
        Opacity
    }

    fn build_state(&self, captures: &mut Captures) -> Self::State {
        Branch2::V0(self.choices.0.build_state(captures))
    }

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        let (horizontal, vertical) = self.axis.components();
        let subview_offer = make_compact_offer(*offer, self.axis);

        // Try first choice with compact offer to see if it fits, creating new state if necessary
        if let Branch2::V0(state0) = state {
            let layout = self.choices.0.layout(&subview_offer, env, captures, state0);
            if offer.contains(layout.resolved_size, horizontal, vertical) {
                // We don't actually want the compact version, so layout again with the original offer
                let layout = self.choices.0.layout(offer, env, captures, state0);
                return ResolvedLayout {
                    resolved_size: layout.resolved_size,
                    sublayouts: Branch2::V0(layout),
                };
            }
        } else {
            let mut state0 = self.choices.0.build_state(captures);
            let layout = self
                .choices
                .0
                .layout(&subview_offer, env, captures, &mut state0);
            if offer.contains(layout.resolved_size, horizontal, vertical) {
                let layout = self.choices.0.layout(offer, env, captures, &mut state0);
                *state = Branch2::V0(state0);
                return ResolvedLayout {
                    resolved_size: layout.resolved_size,
                    sublayouts: Branch2::V0(layout),
                };
            }
        }

        // Use second choice
        if let Branch2::V1(state1) = state {
            let layout = self.choices.1.layout(offer, env, captures, state1);
            return ResolvedLayout {
                resolved_size: layout.resolved_size,
                sublayouts: Branch2::V1(layout),
            };
        }
        let mut state1 = self.choices.1.build_state(captures);
        let layout = self.choices.1.layout(offer, env, captures, &mut state1);
        *state = Branch2::V1(state1);
        ResolvedLayout {
            resolved_size: layout.resolved_size,
            sublayouts: Branch2::V1(layout),
        }
    }

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> Self::Renderables {
        match (&layout.sublayouts, state) {
            (Branch2::V0(l0), Branch2::V0(s0)) => {
                render::OneOf2::V0(self.choices.0.render_tree(l0, origin, env, captures, s0))
            }
            (Branch2::V1(l1), Branch2::V1(s1)) => {
                render::OneOf2::V1(self.choices.1.render_tree(l1, origin, env, captures, s1))
            }
            _ => panic!("Layout/state branch mismatch"),
        }
    }

    fn handle_event(
        &self,
        event: &crate::view::Event,
        context: &crate::event::EventContext,
        render_tree: &mut Self::Renderables,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> EventResult {
        match (state, render_tree) {
            (Branch2::V0(s0), render::OneOf2::V0(t0)) => self
                .choices
                .0
                .handle_event(event, context, t0, captures, s0),
            (Branch2::V1(s1), render::OneOf2::V1(t1)) => self
                .choices
                .1
                .handle_event(event, context, t1, captures, s1),
            _ => {
                // FIXME: I think it's better here to build new state, leaving to see what
                // breaks...
                panic!("Layout/state branch mismatch");
            }
        }
    }
}

impl<T0, T1, T2> ViewMarker for ViewThatFits<(T0, T1, T2)>
where
    T0: ViewMarker,
    T1: ViewMarker,
    T2: ViewMarker,
{
    type Renderables = render::OneOf3<T0::Renderables, T1::Renderables, T2::Renderables>;
    type Transition = Opacity;
}

impl<T0, T1, T2, Captures: ?Sized> ViewLayout<Captures> for ViewThatFits<(T0, T1, T2)>
where
    T0: ViewLayout<Captures>,
    T1: ViewLayout<Captures>,
    T2: ViewLayout<Captures>,
{
    type Sublayout = Branch3<
        ResolvedLayout<T0::Sublayout>,
        ResolvedLayout<T1::Sublayout>,
        ResolvedLayout<T2::Sublayout>,
    >;
    type State = Branch3<T0::State, T1::State, T2::State>;

    fn transition(&self) -> Self::Transition {
        Opacity
    }

    fn build_state(&self, captures: &mut Captures) -> Self::State {
        Branch3::V0(self.choices.0.build_state(captures))
    }

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        let (horizontal, vertical) = self.axis.components();
        let subview_offer = make_compact_offer(*offer, self.axis);

        // Try first choice with compact offer to see if it fits, creating new state if necessary
        if let Branch3::V0(state0) = state {
            let layout = self.choices.0.layout(&subview_offer, env, captures, state0);
            if offer.contains(layout.resolved_size, horizontal, vertical) {
                let layout = self.choices.0.layout(offer, env, captures, state0);
                return ResolvedLayout {
                    resolved_size: layout.resolved_size,
                    sublayouts: Branch3::V0(layout),
                };
            }
        } else {
            let mut state0 = self.choices.0.build_state(captures);
            let layout = self
                .choices
                .0
                .layout(&subview_offer, env, captures, &mut state0);
            if offer.contains(layout.resolved_size, horizontal, vertical) {
                let layout = self.choices.0.layout(offer, env, captures, &mut state0);
                *state = Branch3::V0(state0);
                return ResolvedLayout {
                    resolved_size: layout.resolved_size,
                    sublayouts: Branch3::V0(layout),
                };
            }
        }

        // Try second choice
        if let Branch3::V1(state1) = state {
            let layout = self.choices.1.layout(&subview_offer, env, captures, state1);
            if offer.contains(layout.resolved_size, horizontal, vertical) {
                let layout = self.choices.1.layout(offer, env, captures, state1);
                return ResolvedLayout {
                    resolved_size: layout.resolved_size,
                    sublayouts: Branch3::V1(layout),
                };
            }
        } else {
            let mut state1 = self.choices.1.build_state(captures);
            let layout = self
                .choices
                .1
                .layout(&subview_offer, env, captures, &mut state1);
            if offer.contains(layout.resolved_size, horizontal, vertical) {
                let layout = self.choices.1.layout(offer, env, captures, &mut state1);
                *state = Branch3::V1(state1);
                return ResolvedLayout {
                    resolved_size: layout.resolved_size,
                    sublayouts: Branch3::V1(layout),
                };
            }
        }

        // Use third choice (fallback)
        if let Branch3::V2(state2) = state {
            let layout = self.choices.2.layout(offer, env, captures, state2);
            return ResolvedLayout {
                resolved_size: layout.resolved_size,
                sublayouts: Branch3::V2(layout),
            };
        }
        let mut state2 = self.choices.2.build_state(captures);
        let layout = self.choices.2.layout(offer, env, captures, &mut state2);
        *state = Branch3::V2(state2);
        ResolvedLayout {
            resolved_size: layout.resolved_size,
            sublayouts: Branch3::V2(layout),
        }
    }

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> Self::Renderables {
        match (&layout.sublayouts, state) {
            (Branch3::V0(l0), Branch3::V0(s0)) => {
                render::OneOf3::V0(self.choices.0.render_tree(l0, origin, env, captures, s0))
            }
            (Branch3::V1(l1), Branch3::V1(s1)) => {
                render::OneOf3::V1(self.choices.1.render_tree(l1, origin, env, captures, s1))
            }
            (Branch3::V2(l2), Branch3::V2(s2)) => {
                render::OneOf3::V2(self.choices.2.render_tree(l2, origin, env, captures, s2))
            }
            _ => panic!("Layout/state branch mismatch"),
        }
    }

    fn handle_event(
        &self,
        event: &crate::view::Event,
        context: &crate::event::EventContext,
        render_tree: &mut Self::Renderables,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> EventResult {
        match (state, render_tree) {
            (Branch3::V0(s0), render::OneOf3::V0(t0)) => self
                .choices
                .0
                .handle_event(event, context, t0, captures, s0),
            (Branch3::V1(s1), render::OneOf3::V1(t1)) => self
                .choices
                .1
                .handle_event(event, context, t1, captures, s1),
            (Branch3::V2(s2), render::OneOf3::V2(t2)) => self
                .choices
                .2
                .handle_event(event, context, t2, captures, s2),
            _ => {
                // FIXME: I think it's better here to build new state
                panic!("Layout/state branch mismatch");
            }
        }
    }
}

impl<T0, T1, T2, T3> ViewMarker for ViewThatFits<(T0, T1, T2, T3)>
where
    T0: ViewMarker,
    T1: ViewMarker,
    T2: ViewMarker,
    T3: ViewMarker,
{
    type Renderables =
        render::OneOf4<T0::Renderables, T1::Renderables, T2::Renderables, T3::Renderables>;
    type Transition = Opacity;
}

impl<T0, T1, T2, T3, Captures: ?Sized> ViewLayout<Captures> for ViewThatFits<(T0, T1, T2, T3)>
where
    T0: ViewLayout<Captures>,
    T1: ViewLayout<Captures>,
    T2: ViewLayout<Captures>,
    T3: ViewLayout<Captures>,
{
    type Sublayout = Branch4<
        ResolvedLayout<T0::Sublayout>,
        ResolvedLayout<T1::Sublayout>,
        ResolvedLayout<T2::Sublayout>,
        ResolvedLayout<T3::Sublayout>,
    >;
    type State = Branch4<T0::State, T1::State, T2::State, T3::State>;

    fn transition(&self) -> Self::Transition {
        Opacity
    }

    fn build_state(&self, captures: &mut Captures) -> Self::State {
        Branch4::V0(self.choices.0.build_state(captures))
    }

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        let (horizontal, vertical) = self.axis.components();
        let subview_offer = make_compact_offer(*offer, self.axis);

        // Try first choice with compact offer to see if it fits, creating new state if necessary
        if let Branch4::V0(state0) = state {
            let layout = self.choices.0.layout(&subview_offer, env, captures, state0);
            if offer.contains(layout.resolved_size, horizontal, vertical) {
                let layout = self.choices.0.layout(offer, env, captures, state0);
                return ResolvedLayout {
                    resolved_size: layout.resolved_size,
                    sublayouts: Branch4::V0(layout),
                };
            }
        } else {
            let mut state0 = self.choices.0.build_state(captures);
            let layout = self
                .choices
                .0
                .layout(&subview_offer, env, captures, &mut state0);
            if offer.contains(layout.resolved_size, horizontal, vertical) {
                let layout = self.choices.0.layout(offer, env, captures, &mut state0);
                *state = Branch4::V0(state0);
                return ResolvedLayout {
                    resolved_size: layout.resolved_size,
                    sublayouts: Branch4::V0(layout),
                };
            }
        }

        // Try second choice
        if let Branch4::V1(state1) = state {
            let layout = self.choices.1.layout(&subview_offer, env, captures, state1);
            if offer.contains(layout.resolved_size, horizontal, vertical) {
                let layout = self.choices.1.layout(offer, env, captures, state1);
                return ResolvedLayout {
                    resolved_size: layout.resolved_size,
                    sublayouts: Branch4::V1(layout),
                };
            }
        } else {
            let mut state1 = self.choices.1.build_state(captures);
            let layout = self
                .choices
                .1
                .layout(&subview_offer, env, captures, &mut state1);
            if offer.contains(layout.resolved_size, horizontal, vertical) {
                let layout = self.choices.1.layout(offer, env, captures, &mut state1);
                *state = Branch4::V1(state1);
                return ResolvedLayout {
                    resolved_size: layout.resolved_size,
                    sublayouts: Branch4::V1(layout),
                };
            }
        }

        // Try third choice
        if let Branch4::V2(state2) = state {
            let layout = self.choices.2.layout(&subview_offer, env, captures, state2);
            if offer.contains(layout.resolved_size, horizontal, vertical) {
                let layout = self.choices.2.layout(offer, env, captures, state2);
                return ResolvedLayout {
                    resolved_size: layout.resolved_size,
                    sublayouts: Branch4::V2(layout),
                };
            }
        } else {
            let mut state2 = self.choices.2.build_state(captures);
            let layout = self
                .choices
                .2
                .layout(&subview_offer, env, captures, &mut state2);
            if offer.contains(layout.resolved_size, horizontal, vertical) {
                let layout = self.choices.2.layout(offer, env, captures, &mut state2);
                *state = Branch4::V2(state2);
                return ResolvedLayout {
                    resolved_size: layout.resolved_size,
                    sublayouts: Branch4::V2(layout),
                };
            }
        }

        // Use fourth choice (fallback)
        if let Branch4::V3(state3) = state {
            let layout = self.choices.3.layout(offer, env, captures, state3);
            return ResolvedLayout {
                resolved_size: layout.resolved_size,
                sublayouts: Branch4::V3(layout),
            };
        }
        let mut state3 = self.choices.3.build_state(captures);
        let layout = self.choices.3.layout(offer, env, captures, &mut state3);
        *state = Branch4::V3(state3);
        ResolvedLayout {
            resolved_size: layout.resolved_size,
            sublayouts: Branch4::V3(layout),
        }
    }

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        env: &impl LayoutEnvironment,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> Self::Renderables {
        match (&layout.sublayouts, state) {
            (Branch4::V0(l0), Branch4::V0(s0)) => {
                render::OneOf4::V0(self.choices.0.render_tree(l0, origin, env, captures, s0))
            }
            (Branch4::V1(l1), Branch4::V1(s1)) => {
                render::OneOf4::V1(self.choices.1.render_tree(l1, origin, env, captures, s1))
            }
            (Branch4::V2(l2), Branch4::V2(s2)) => {
                render::OneOf4::V2(self.choices.2.render_tree(l2, origin, env, captures, s2))
            }
            (Branch4::V3(l3), Branch4::V3(s3)) => {
                render::OneOf4::V3(self.choices.3.render_tree(l3, origin, env, captures, s3))
            }
            _ => panic!("Layout/state branch mismatch"),
        }
    }

    fn handle_event(
        &self,
        event: &crate::view::Event,
        context: &crate::event::EventContext,
        render_tree: &mut Self::Renderables,
        captures: &mut Captures,
        state: &mut Self::State,
    ) -> EventResult {
        match (state, render_tree) {
            (Branch4::V0(s0), render::OneOf4::V0(t0)) => self
                .choices
                .0
                .handle_event(event, context, t0, captures, s0),
            (Branch4::V1(s1), render::OneOf4::V1(t1)) => self
                .choices
                .1
                .handle_event(event, context, t1, captures, s1),
            (Branch4::V2(s2), render::OneOf4::V2(t2)) => self
                .choices
                .2
                .handle_event(event, context, t2, captures, s2),
            (Branch4::V3(s3), render::OneOf4::V3(t3)) => self
                .choices
                .3
                .handle_event(event, context, t3, captures, s3),
            _ => {
                // FIXME: I think it's better here to build new state
                panic!("Layout/state branch mismatch");
            }
        }
    }
}
