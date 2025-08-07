use crate::{
    primitives::Point,
    render::{AnimatedJoin, Render},
    render_target::RenderTarget,
};

use super::AnimationDomain;

impl<T: AnimatedJoin> AnimatedJoin for Option<T> {
    fn join_from(&mut self, source: &Self, domain: &AnimationDomain) {
        match (source, self) {
            (Some(source), Some(target)) => target.join_from(source, domain),
            (_, None) => (),
            (None, Some(_target)) => (),
        }
    }
}

impl<T: Render<Color>, Color> Render<Color> for Option<T> {
    fn render(
        &self,
        render_target: &mut impl RenderTarget<ColorFormat = Color>,
        style: &Color,
        offset: Point,
    ) {
        if let Some(view) = self {
            view.render(render_target, style, offset);
        }
    }

    fn render_animated(
        render_target: &mut impl RenderTarget<ColorFormat = Color>,
        source: &Self,
        target: &Self,
        style: &Color,
        offset: Point,
        domain: &AnimationDomain,
    ) {
        match (source, target) {
            (Some(source), Some(target)) => {
                T::render_animated(render_target, source, target, style, offset, domain);
            }
            (_, None) => {}
            (None, Some(target)) => {
                target.render(render_target, style, offset);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{primitives::Size, render::shape::Rect, render_target::FixedTextBuffer};
    use std::{string::String, string::ToString, time::Duration};

    #[test]
    fn some_option_renders() {
        let mut buffer = FixedTextBuffer::<1, 1>::default();
        Some(Rect::new(Point::zero(), Size::new(1, 1))).render(&mut buffer, &'X', Point::zero());
        assert_eq!(buffer.text[0][0], 'X');
    }

    #[test]
    fn none_option_does_not_render() {
        let mut buffer = FixedTextBuffer::<1, 1>::default();
        Option::<Rect>::None.render(&mut buffer, &'X', Point::zero());
        assert_eq!(buffer.text[0][0], ' ');
    }

    #[test]
    fn animated_render_some_to_some() {
        let mut buffer = FixedTextBuffer::<3, 1>::default();
        let source = Some(Rect::new(Point::zero(), Size::new(1, 1)));
        let target = Some(Rect::new(Point::new(2, 0), Size::new(1, 1)));
        let domain = AnimationDomain::new(128, Duration::from_millis(100));
        Option::render_animated(&mut buffer, &source, &target, &'O', Point::zero(), &domain);
        assert_eq!(buffer.text[0].iter().collect::<String>(), " O ".to_string());
    }

    #[test]
    fn animated_render_some_to_none() {
        let mut buffer = FixedTextBuffer::<1, 1>::default();
        let source = Some(Rect::new(Point::zero(), Size::new(1, 1)));
        let domain = AnimationDomain::new(128, Duration::from_millis(100));
        Option::render_animated(&mut buffer, &source, &None, &'X', Point::zero(), &domain);
        assert_eq!(buffer.text[0][0], ' ');
    }

    #[test]
    fn animated_render_none_to_some() {
        let mut buffer = FixedTextBuffer::<1, 1>::default();
        let target = Some(Rect::new(Point::zero(), Size::new(1, 1)));
        let domain = AnimationDomain::new(128, Duration::from_millis(100));
        Option::render_animated(&mut buffer, &None, &target, &'Y', Point::zero(), &domain);
        assert_eq!(buffer.text[0][0], 'Y');
    }

    #[test]
    fn animated_render_none_to_none() {
        let mut buffer = FixedTextBuffer::<1, 1>::default();
        let domain = AnimationDomain::new(128, Duration::from_millis(100));
        Option::<Rect>::render_animated(&mut buffer, &None, &None, &'Z', Point::zero(), &domain);
        assert_eq!(buffer.text[0][0], ' ');
    }

    #[test]
    fn animated_join_some_to_some() {
        let source = Some(Rect::new(Point::zero(), Size::new(10, 10)));
        let mut target = Some(Rect::new(Point::new(20, 20), Size::new(30, 30)));
        let domain = AnimationDomain::new(128, Duration::from_millis(100));
        target.join_from(&source, &domain);
        assert_eq!(
            target,
            Some(Rect::new(Point::new(10, 10), Size::new(20, 20)))
        );
    }

    #[test]
    fn animated_join_some_to_none() {
        let source = Some(Rect::new(Point::zero(), Size::new(1, 1)));
        let mut target = None;
        let domain = AnimationDomain::new(128, Duration::from_millis(100));
        target.join_from(&source, &domain);
        assert_eq!(target, None);
    }

    #[test]
    fn animated_join_none_to_some() {
        let source = None;
        let mut target = Some(Rect::new(Point::zero(), Size::new(1, 1)));
        let domain = AnimationDomain::new(128, Duration::from_millis(100));
        target.join_from(&source, &domain);
        assert_eq!(target, Some(Rect::new(Point::zero(), Size::new(1, 1))));
    }

    #[test]
    fn animated_join_none_to_none() {
        let source = None;
        let mut target = None;
        let domain = AnimationDomain::new(128, Duration::from_millis(100));
        Option::<Rect>::join_from(&mut target, &source, &domain);
        assert_eq!(target, None);
    }
}
