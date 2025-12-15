use super::{AnimatedJoin, AnimationDomain, Render, RenderTarget};

macro_rules! impl_join_for_collections {
    ($(($n:tt, $type:ident)),+) => {
        impl<$($type: crate::render::AnimatedJoin),+> crate::render::AnimatedJoin for ($($type),+) {
            fn join_from(
                &mut self,
                source: &Self,
                domain: &crate::render::AnimationDomain
            ) {
                $(
                    self.$n.join_from(
                        &source.$n,
                        domain
                    );
                )+
            }
        }
    };
}

#[rustfmt::skip]
mod impl_join {
    impl_join_for_collections!((0, T0), (1, T1));
    impl_join_for_collections!((0, T0), (1, T1), (2, T2));
    impl_join_for_collections!((0, T0), (1, T1), (2, T2), (3, T3));
    impl_join_for_collections!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4));
    impl_join_for_collections!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5));
    impl_join_for_collections!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6));
    impl_join_for_collections!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7));
    impl_join_for_collections!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7), (8, T8));
    impl_join_for_collections!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7), (8, T8), (9, T9));
}

impl<T: AnimatedJoin> AnimatedJoin for [T] {
    fn join_from(&mut self, source: &Self, domain: &AnimationDomain) {
        self.iter_mut().zip(source).for_each(|(target, source)| {
            target.join_from(source, domain);
        });
    }
}

impl<T: AnimatedJoin, const N: usize> AnimatedJoin for [T; N] {
    fn join_from(&mut self, source: &Self, domain: &AnimationDomain) {
        self.as_mut_slice().join_from(source.as_slice(), domain);
    }
}

impl<T: AnimatedJoin, const N: usize> AnimatedJoin for heapless::Vec<T, N> {
    fn join_from(&mut self, source: &Self, domain: &AnimationDomain) {
        self.as_mut_slice().join_from(source.as_slice(), domain);
    }
}

macro_rules! impl_render_for_collections {
    ($(($n:tt, $type:ident)),+) => {
        impl<Color, $($type: crate::render::Render<Color> ),+> crate::render::Render<Color> for ($($type),+) {
            fn render(
                &self,
                target: &mut impl crate::render_target::RenderTarget<ColorFormat = Color>,
                style: &Color,
            ) {
                $(
                    self.$n.render(target, style);
                )+
            }

            fn render_animated(
                render_target: &mut impl crate::render_target::RenderTarget<ColorFormat = Color>,
                source: &Self,
                target: &Self,
                style: &Color,
                domain: &crate::render::AnimationDomain,
            ) {
                $(
                    $type::render_animated(
                        render_target,
                        &source.$n,
                        &target.$n,
                        style,
                        domain,
                    );
                )+
            }
        }
    };
}

#[rustfmt::skip]
mod impl_render {
    impl_render_for_collections!((0, T0), (1, T1));
    impl_render_for_collections!((0, T0), (1, T1), (2, T2));
    impl_render_for_collections!((0, T0), (1, T1), (2, T2), (3, T3));
    impl_render_for_collections!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4));
    impl_render_for_collections!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5));
    impl_render_for_collections!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6));
    impl_render_for_collections!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7));
    impl_render_for_collections!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7), (8, T8));
    impl_render_for_collections!((0, T0), (1, T1), (2, T2), (3, T3), (4, T4), (5, T5), (6, T6), (7, T7), (8, T8), (9, T9));
}

impl<Color, T: Render<Color>> Render<Color> for [T] {
    fn render(&self, render_target: &mut impl RenderTarget<ColorFormat = Color>, style: &Color) {
        for item in self {
            item.render(render_target, style);
        }
    }

    fn render_animated(
        render_target: &mut impl RenderTarget<ColorFormat = Color>,
        source: &Self,
        target: &Self,
        style: &Color,
        domain: &AnimationDomain,
    ) {
        source
            .iter()
            .zip(target.iter())
            .for_each(|(source, target)| {
                T::render_animated(render_target, source, target, style, domain);
            });
    }
}

impl<Color, T: Render<Color>, const N: usize> Render<Color> for [T; N] {
    #[inline]
    fn render(&self, render_target: &mut impl RenderTarget<ColorFormat = Color>, style: &Color) {
        self.as_slice().render(render_target, style);
    }

    #[inline]
    fn render_animated(
        render_target: &mut impl RenderTarget<ColorFormat = Color>,
        source: &Self,
        target: &Self,
        style: &Color,
        domain: &AnimationDomain,
    ) {
        <[T]>::render_animated(render_target, source, target, style, domain);
    }
}

impl<Color, T: Render<Color>, const N: usize> Render<Color> for heapless::Vec<T, N> {
    #[inline]
    fn render(&self, render_target: &mut impl RenderTarget<ColorFormat = Color>, style: &Color) {
        self.as_slice().render(render_target, style);
    }

    #[inline]
    fn render_animated(
        render_target: &mut impl RenderTarget<ColorFormat = Color>,
        source: &Self,
        target: &Self,
        style: &Color,
        domain: &AnimationDomain,
    ) {
        <[T]>::render_animated(render_target, source, target, style, domain);
    }
}

/// Make sure tuples render in the correct order
#[cfg(test)]
mod render_order_tests {
    use crate::render::{AnimationDomain, Render};
    use std;
    use std::vec;
    use std::{cell::RefCell, rc::Rc, vec::Vec};

    #[derive(Debug, Clone)]
    struct OrderTracker {
        order: Rc<RefCell<Vec<usize>>>,
        id: usize,
    }

    impl OrderTracker {
        fn new(id: usize, order: Rc<RefCell<Vec<usize>>>) -> Self {
            Self { order, id }
        }
    }

    impl super::AnimatedJoin for OrderTracker {
        fn join_from(&mut self, _source: &Self, _domain: &AnimationDomain) {}
    }

    impl Render<char> for OrderTracker {
        fn render(
            &self,
            _render_target: &mut impl crate::render_target::RenderTarget<ColorFormat = char>,
            _style: &char,
        ) {
            self.order.borrow_mut().push(self.id);
        }

        fn render_animated(
            _render_target: &mut impl crate::render_target::RenderTarget<ColorFormat = char>,
            source: &Self,
            target: &Self,
            _style: &char,
            _domain: &AnimationDomain,
        ) {
            source.order.borrow_mut().push(source.id);
            target.order.borrow_mut().push(target.id);
        }
    }

    macro_rules! test_tuple_render_order {
        ($($n:tt),+) => {
            paste::paste! {
                #[test]
                fn [<tuple_render_order_$($n)*>]() {
                    let order = Rc::new(RefCell::new(Vec::new()));
                    let tuple = (
                        $(
                            OrderTracker::new($n, order.clone()),
                        )+
                    );

                    let mut mock_target = crate::render_target::FixedTextBuffer::<10, 10>::default();
                    tuple.render(&mut mock_target, &'x');

                    let expected_order: Vec<usize> = vec![$($n),+];
                    assert_eq!(*order.borrow(), expected_order);
                }

                #[test]
                fn [<tuple_animated_render_order_$($n)*>]() {
                    let order = Rc::new(RefCell::new(Vec::new()));

                    let mut mock_target = crate::render_target::FixedTextBuffer::<10, 10>::default();

                    let source_tuple = (
                        $(
                            OrderTracker::new($n, order.clone()),
                        )+
                    );
                    let target_tuple = (
                        $(
                            OrderTracker::new($n + 100, order.clone()),
                        )+
                    );

                    let domain = AnimationDomain::new(128, std::time::Duration::from_secs(1));
                    Render::render_animated(
                        &mut mock_target,
                        &source_tuple,
                        &target_tuple,
                        &'x',
                        &domain,
                    );

                    let expected_order: Vec<usize> = vec![$($n, $n + 100),+];
                    assert_eq!(*order.borrow(), expected_order);
                }
            }
        };
    }

    test_tuple_render_order!(0, 1);
    test_tuple_render_order!(0, 1, 2);
    test_tuple_render_order!(0, 1, 2, 3);
    test_tuple_render_order!(0, 1, 2, 3, 4);
    test_tuple_render_order!(0, 1, 2, 3, 4, 5);
    test_tuple_render_order!(0, 1, 2, 3, 4, 5, 6);
    test_tuple_render_order!(0, 1, 2, 3, 4, 5, 6, 7);
    test_tuple_render_order!(0, 1, 2, 3, 4, 5, 6, 7, 8);
    test_tuple_render_order!(0, 1, 2, 3, 4, 5, 6, 7, 8, 9);
}
