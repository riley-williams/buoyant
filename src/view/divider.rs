use crate::{
    layout::{Environment, Layout, LayoutDirection, PreRender},
    primitives::{Point, Size},
    render::Render,
    render_target::RenderTarget,
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

impl Layout for Divider {
    type Cache<'a> = ();
    fn layout(&self, offer: Size, env: &dyn Environment) -> PreRender<'_, Self, ()> {
        let size = match env.layout_direction() {
            LayoutDirection::Horizontal => Size::new(self.weight, offer.height),
            LayoutDirection::Vertical => Size::new(offer.width, self.weight),
        };
        PreRender {
            source_view: self,
            layout_cache: (),
            resolved_size: size,
        }
    }

    fn priority(&self) -> i8 {
        i8::MAX
    }
}

impl<Cache> Render<char, Cache> for Divider {
    fn render(
        &self,
        target: &mut impl RenderTarget<char>,
        _cache: &Cache,
        resolved_size: Size,
        env: &dyn Environment,
    ) {
        match env.layout_direction() {
            LayoutDirection::Horizontal => {
                for y in 0..resolved_size.height {
                    target.draw(Point::new(0, y as i16), '|');
                }
            }
            LayoutDirection::Vertical => {
                for x in 0..resolved_size.width {
                    target.draw(Point::new(x as i16, 0), '-');
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::layout::{Layout, LayoutDirection};
    use crate::primitives::Size;
    use crate::render_target::FixedTextBuffer;

    struct TestEnv {
        direction: LayoutDirection,
    }
    impl Environment for TestEnv {
        fn layout_direction(&self) -> LayoutDirection {
            self.direction
        }
    }

    #[test]
    fn test_horizontal_layout() {
        let divider = Divider::new(2);
        let offer = Size::new(100, 100);
        let env = TestEnv {
            direction: LayoutDirection::Horizontal,
        };
        let pre_render = divider.layout(offer, &env);
        assert_eq!(pre_render.resolved_size, Size::new(2, 100));
    }

    #[test]
    fn test_vertical_layout() {
        let divider = Divider::new(2);
        let offer = Size::new(100, 100);
        let env = TestEnv {
            direction: LayoutDirection::Vertical,
        };
        let pre_render = divider.layout(offer, &env);
        assert_eq!(pre_render.resolved_size, Size::new(100, 2));
    }

    #[test]
    fn test_horizontal_render() {
        let divider = Divider::new(1);
        let mut buffer = FixedTextBuffer::<5, 5>::default();
        let env = TestEnv {
            direction: LayoutDirection::Horizontal,
        };
        let layout = divider.layout(buffer.size(), &env);
        divider.render(&mut buffer, &layout, layout.resolved_size, &env);
        assert_eq!(buffer.text[0][0], '|');
        assert_eq!(buffer.text[4][0], '|');
        assert_eq!(buffer.text[0][1], ' ');
    }

    #[test]
    fn test_vertical_render() {
        let divider = Divider::new(1);
        let mut buffer = FixedTextBuffer::<5, 5>::default();
        let env = TestEnv {
            direction: LayoutDirection::Vertical,
        };
        let layout = divider.layout(buffer.size(), &env);
        divider.render(&mut buffer, &layout, layout.resolved_size, &env);
        assert_eq!(buffer.text[0][0], '-');
        assert_eq!(buffer.text[0][4], '-');
        assert_eq!(buffer.text[1][0], ' ');
    }
}
