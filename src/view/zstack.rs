use crate::{
    environment::LayoutEnvironment,
    layout::{HorizontalAlignment, Layout, ResolvedLayout, VerticalAlignment},
    primitives::{Point, ProposedDimensions},
    render::Renderable,
};

use paste::paste;

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
    pub fn horizontal_alignment(self, alignment: HorizontalAlignment) -> Self {
        Self {
            horizontal_alignment: alignment,
            ..self
        }
    }

    pub fn vertical_alignment(self, alignment: VerticalAlignment) -> Self {
        Self {
            vertical_alignment: alignment,
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
                    let [<layout$n>] = self.items.$n.layout(offer, env);
                )+
                let size = layout0.resolved_size $(.union([<layout$n>].resolved_size))+;

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
