use crate::{
    environment::LayoutEnvironment,
    layout::{Alignment, HorizontalAlignment, Layout, ResolvedLayout, VerticalAlignment},
    primitives::{Point, ProposedDimension, ProposedDimensions},
    render::Renderable,
};

use paste::paste;

/// A stack of heterogeneous views that arranges its views from back to front.
/// The parent size is first offered to each subview. If any offered dimension is
/// ``ProposedDimension::Compact``, ``ZStack`` will offer a new frame that is the
/// union of all the resolved frame sizes from the previous pass.
///
/// ```rust
/// use buoyant::font::CharacterBufferFont;
/// use buoyant::layout::Alignment;
/// use buoyant::view::{ZStack, shape::Rectangle, Text, RenderExtensions as _};
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
        ZStack {
            items,
            horizontal_alignment: HorizontalAlignment::default(),
            vertical_alignment: VerticalAlignment::default(),
        }
    }
}

macro_rules! impl_layout_for_zstack {
    ($(($n:tt, $type:ident)),+) => {
        paste! {
        impl<$($type: Layout),+> Layout for ZStack<($($type),+)> {
            type Sublayout = ($(ResolvedLayout<$type::Sublayout>),+);

            fn layout(
                &self,
                offer: &ProposedDimensions,
                env: &impl LayoutEnvironment,
            ) -> ResolvedLayout<Self::Sublayout> {
                $(
                    let mut [<layout$n>] = self.items.$n.layout(offer, env);
                )+
                let mut size = layout0.resolved_size $(.union([<layout$n>].resolved_size))+;

                if matches!(offer.width, ProposedDimension::Compact) || matches!(offer.height, ProposedDimension::Compact) {
                    // FIXME: The `.into()` here is almost certainly wrong.
                    // While it would be unusual for a view to respond requesting infinite
                    // width or height in response to a compact request, this does not
                    // effectively handle it. This also increases the liklihood of overflow
                    // due to the way Dimension is implemented
                    let offer = ProposedDimensions {
                        width: ProposedDimension::Exact(size.width.into()),
                        height: ProposedDimension::Exact(size.height.into()),
                    };
                    $(
                        [<layout$n>] = self.items.$n.layout(&offer, env);
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
        }

        impl<$($type: Renderable<C>),+, C> Renderable<C> for ZStack<($($type),+)> {
            type Renderables = ($($type::Renderables),+);

            fn render_tree(
                &self,
                layout: &ResolvedLayout<Self::Sublayout>,
                origin: Point,
                env: &impl LayoutEnvironment,
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
                        self.items.$n.render_tree(&layout.sublayouts.$n, [<offset_$n>], env)
                    ),+
                )
            }
        }
    }
    }
}

impl_layout_for_zstack!((0, T0), (1, T1));
impl_layout_for_zstack!((0, T0), (1, T1), (2, T2));
impl_layout_for_zstack!((0, T0), (1, T1), (2, T2), (3, T3));
impl_layout_for_zstack!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4));
impl_layout_for_zstack!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5));
impl_layout_for_zstack!(
    (0, T0),
    (1, T1),
    (2, T2),
    (3, T3),
    (4, T4),
    (5, T5),
    (6, T6)
);
impl_layout_for_zstack!(
    (0, T0),
    (1, T1),
    (2, T2),
    (3, T3),
    (4, T4),
    (5, T5),
    (6, T6),
    (7, T7)
);
impl_layout_for_zstack!(
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
impl_layout_for_zstack!(
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
