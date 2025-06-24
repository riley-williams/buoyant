use crate::{
    environment::LayoutEnvironment,
    layout::{Alignment, HorizontalAlignment, ResolvedLayout, VerticalAlignment},
    primitives::{Point, ProposedDimension, ProposedDimensions},
    view::{ViewLayout, ViewMarker},
};

use paste::paste;

/// A stack of heterogeneous views that arranges its views from back to front.
///
/// The parent size is first offered to each subview. If any offered dimension is
/// ``ProposedDimension::Compact``, ``ZStack`` will offer a new frame that is the
/// union of all the resolved frame sizes from the previous pass.
///
/// ```rust
/// use buoyant::font::CharacterBufferFont;
/// use buoyant::layout::Alignment;
/// use buoyant::view::{ZStack, shape::Rectangle, Text, ViewExt as _};
///
/// /// A fish at the bottom right corner of an 'o'cean
/// let font = CharacterBufferFont {};
/// let stack = ZStack::new((
///         Rectangle,
///         Text::new("><>", &font),
///     ))
///     .with_alignment(Alignment::BottomTrailing)
///     .foreground_color('o');
/// ```
#[derive(Debug, Clone)]
pub struct ZStack<T> {
    items: T,
    horizontal_alignment: HorizontalAlignment,
    vertical_alignment: VerticalAlignment,
}

impl<T> PartialEq for ZStack<T> {
    fn eq(&self, other: &Self) -> bool {
        self.horizontal_alignment == other.horizontal_alignment
            && self.vertical_alignment == other.vertical_alignment
    }
}

impl<T> ZStack<T> {
    #[must_use]
    pub fn with_horizontal_alignment(self, alignment: HorizontalAlignment) -> Self {
        Self {
            horizontal_alignment: alignment,
            ..self
        }
    }

    #[must_use]
    pub fn with_vertical_alignment(self, alignment: VerticalAlignment) -> Self {
        Self {
            vertical_alignment: alignment,
            ..self
        }
    }

    #[must_use]
    pub fn with_alignment(self, alignment: Alignment) -> Self {
        Self {
            horizontal_alignment: alignment.horizontal(),
            vertical_alignment: alignment.vertical(),
            ..self
        }
    }
}

impl<T> ZStack<T> {
    pub fn new(items: T) -> Self {
        Self {
            items,
            horizontal_alignment: HorizontalAlignment::default(),
            vertical_alignment: VerticalAlignment::default(),
        }
    }
}

macro_rules! impl_view_for_zstack {
    ($(($n:tt, $type:ident)),+) => {
        paste! {
        impl<$($type),+> ViewMarker for ZStack<($($type),+)>
        where
            $($type: ViewMarker),+
        {
            type Renderables = ($($type::Renderables),+);
        }

        impl<Captures: ?Sized, $($type),+> ViewLayout<Captures> for ZStack<($($type),+)>
        where
            $($type: ViewLayout<Captures>),+
        {
            type Sublayout = ($(ResolvedLayout<$type::Sublayout>),+);
            type State = ($($type::State),+);

            fn build_state(&self, captures: &mut Captures) -> Self::State {
                ($(self.items.$n.build_state(captures)),+)
            }

            fn layout(
                &self,
                offer: &ProposedDimensions,
                env: &impl LayoutEnvironment,
                captures: &mut Captures,
                state: &mut Self::State,
            ) -> ResolvedLayout<Self::Sublayout> {
                $(
                    let mut [<layout$n>] = self.items.$n.layout(offer, env, captures, &mut state.$n);
                )+
                let mut size = layout0.resolved_size $(.union([<layout$n>].resolved_size))+;

                if matches!(offer.width, ProposedDimension::Compact) || matches!(offer.height, ProposedDimension::Compact) {
                    // FIXME: The `.into()` here is almost certainly wrong.
                    // While it would be unusual for a view to respond requesting infinite
                    // width or height in response to a compact request, this does not
                    // effectively handle it. This also increases the likelihood of overflow
                    // due to the way Dimension is implemented
                    let offer = ProposedDimensions {
                        width: ProposedDimension::Exact(size.width.into()),
                        height: ProposedDimension::Exact(size.height.into()),
                    };
                    $(
                        [<layout$n>] = self.items.$n.layout(&offer, env, captures, &mut state.$n);
                    )+
                    size = layout0.resolved_size $(.union([<layout$n>].resolved_size))+;
                }

                ResolvedLayout {
                    sublayouts: ($(
                        [<layout$n>]
                    ),+),
                    resolved_size: size.intersecting_proposal(offer),
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
                $(
                    let [<offset_$n>] = origin
                        + Point::new(
                            self.horizontal_alignment.align(
                                layout.resolved_size.width.into(),
                                layout.sublayouts.$n.resolved_size.width.into(),
                            ),
                            self.vertical_alignment.align(
                                layout.resolved_size.height.into(),
                                layout.sublayouts.$n.resolved_size.height.into(),
                            ),
                        );
                )+

                (
                    $(
                        self.items.$n.render_tree(&layout.sublayouts.$n, [<offset_$n>], env, captures, &mut state.$n)
                    ),+
                )
            }

            fn handle_event(
                &mut self,
                event: &crate::view::Event,
                render_tree: &mut Self::Renderables,
                captures: &mut Captures,
                state: &mut Self::State,
            ) -> bool {
                $(
                    if self.items.$n.handle_event(event, &mut render_tree.$n, captures, &mut state.$n) {
                        return true;
                    }
                )+
                false
            }
        }
    }
    }
}

impl_view_for_zstack!((0, T0), (1, T1));
impl_view_for_zstack!((0, T0), (1, T1), (2, T2));
impl_view_for_zstack!((0, T0), (1, T1), (2, T2), (3, T3));
impl_view_for_zstack!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4));
impl_view_for_zstack!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5));
impl_view_for_zstack!(
    (0, T0),
    (1, T1),
    (2, T2),
    (3, T3),
    (4, T4),
    (5, T5),
    (6, T6)
);
impl_view_for_zstack!(
    (0, T0),
    (1, T1),
    (2, T2),
    (3, T3),
    (4, T4),
    (5, T5),
    (6, T6),
    (7, T7)
);
impl_view_for_zstack!(
    (0, T0),
    (1, T1),
    (2, T2),
    (3, T3),
    (4, T4),
    (5, T5),
    (6, T6),
    (7, T7),
    (8, T8)
);
impl_view_for_zstack!(
    (0, T0),
    (1, T1),
    (2, T2),
    (3, T3),
    (4, T4),
    (5, T5),
    (6, T6),
    (7, T7),
    (8, T8),
    (9, T9)
);
