use crate::{
    environment::LayoutEnvironment,
    layout::{Layout, LayoutDirection, ResolvedLayout},
    primitives::{Dimensions, Point, ProposedDimensions},
    render::Renderable,
};

pub struct Divider {
    pub weight: u16,
}

impl Divider {
    pub fn new(weight: u16) -> Self {
        Self { weight }
    }
}

impl Default for Divider {
    fn default() -> Self {
        Self::new(1)
    }
}

impl PartialEq for Divider {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl Layout for Divider {
    type Sublayout = ();

    fn layout(
        &self,
        offer: &ProposedDimensions,
        env: &impl LayoutEnvironment,
    ) -> ResolvedLayout<()> {
        let size = match env.layout_direction() {
            LayoutDirection::Vertical => Dimensions {
                width: offer.width.resolve_most_flexible(0, 10),
                height: self.weight.into(),
            },
            LayoutDirection::Horizontal => Dimensions {
                width: self.weight.into(),
                height: offer.height.resolve_most_flexible(0, 10),
            },
        };
        ResolvedLayout {
            sublayouts: (),
            resolved_size: size,
        }
    }

    fn priority(&self) -> i8 {
        i8::MAX
    }
}

impl<C> Renderable<C> for Divider {
    type Renderables = crate::render::primitives::Rect;

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        _env: &impl LayoutEnvironment,
    ) -> Self::Renderables {
        crate::render::primitives::Rect {
            origin,
            size: layout.resolved_size.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Divider;
    use crate::environment::mock::TestEnv;
    use crate::layout::{Layout, LayoutDirection};
    use crate::primitives::{Point, Size};
    use crate::render::{Render, Renderable as _};
    use crate::render_target::{FixedTextBuffer, RenderTarget, TxtColor};
    use crate::view::RenderExtensions as _;

    #[test]
    fn test_horizontal_layout() {
        let divider = Divider::new(2);
        let offer = Size::new(100, 100).into();
        let env = TestEnv::default().with_direction(LayoutDirection::Horizontal);
        let layout = divider.layout(&offer, &env);
        assert_eq!(layout.resolved_size, Size::new(2, 100).into());
    }

    #[test]
    fn test_vertical_layout() {
        let divider = Divider::new(2);
        let offer = Size::new(100, 100).into();
        let env = TestEnv::default().with_direction(LayoutDirection::Vertical);
        let layout = divider.layout(&offer, &env);
        assert_eq!(layout.resolved_size, Size::new(100, 2).into());
    }

    #[test]
    fn test_horizontal_render() {
        let divider = Divider::new(1).foreground_color(TxtColor::new('|'));
        let mut buffer = FixedTextBuffer::<5, 5>::default();
        let env = TestEnv::default().with_direction(LayoutDirection::Horizontal);
        let layout = divider.layout(&buffer.size().into(), &env);
        let tree = divider.render_tree(&layout, Point::new(0, 0), &env);
        tree.render(&mut buffer, &TxtColor::default());
        assert_eq!(buffer.text[0][0], '|');
        assert_eq!(buffer.text[4][0], '|');
        assert_eq!(buffer.text[0][1], ' ');
    }

    #[test]
    fn test_vertical_render() {
        let divider = Divider::new(1).foreground_color(TxtColor::new('-'));
        let mut buffer = FixedTextBuffer::<5, 5>::default();
        let env = TestEnv::default().with_direction(LayoutDirection::Vertical);
        let layout = divider.layout(&buffer.size().into(), &env);
        let tree = divider.render_tree(&layout, Point::new(0, 0), &env);
        tree.render(&mut buffer, &TxtColor::default());
        assert_eq!(buffer.text[0][0], '-');
        assert_eq!(buffer.text[0][4], '-');
        assert_eq!(buffer.text[1][0], ' ');
    }
}
