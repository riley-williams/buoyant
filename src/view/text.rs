use crate::{
    environment::LayoutEnvironment,
    font::{Font, FontMetrics},
    layout::{Layout, ResolvedLayout},
    primitives::{Point, ProposedDimension, ProposedDimensions, Size},
    render::{self, Renderable},
};
use core::marker::PhantomData;

pub use wrap::WhitespaceWrap;

mod wrap;

// W is hardcoded elsewhere to WhitespaceWrap, leaving generic for future fix

#[derive(Debug, Clone)]
pub struct Text<'a, T, F, W = WhitespaceWrap<'a, F>> {
    pub(crate) text: T,
    pub(crate) font: &'a F,
    pub(crate) alignment: HorizontalTextAlignment,
    pub(crate) _wrap: PhantomData<W>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HorizontalTextAlignment {
    #[default]
    Leading,
    Center,
    Trailing,
}

impl HorizontalTextAlignment {
    pub(crate) const fn align(self, available: i32, content: i32) -> i32 {
        match self {
            Self::Leading => 0,
            Self::Center => (available - content) / 2,
            Self::Trailing => available - content,
        }
    }
}

impl<'a, T: AsRef<str>, F> Text<'a, T, F> {
    #[must_use]
    pub fn new(text: T, font: &'a F) -> Self {
        Self {
            text,
            font,
            alignment: HorizontalTextAlignment::default(),
            _wrap: PhantomData,
        }
    }
}

impl<T, F> Text<'_, T, F> {
    #[must_use]
    pub fn multiline_text_alignment(self, alignment: HorizontalTextAlignment) -> Self {
        Text { alignment, ..self }
    }
}

impl<T: PartialEq, F> PartialEq for Text<'_, T, F> {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
    }
}

impl<T: AsRef<str>, F: Font> Layout for Text<'_, T, F> {
    // this could be used to store the precalculated line breaks
    type Sublayout = ();

    fn layout(
        &self,
        offer: &ProposedDimensions,
        _env: &impl LayoutEnvironment,
    ) -> ResolvedLayout<Self::Sublayout> {
        let metrics = self.font.metrics();
        let line_height = metrics.default_line_height();
        let wrap = WhitespaceWrap::new(self.text.as_ref(), offer.width, &metrics);
        let mut size = Size::zero();
        for line in wrap {
            size.width = core::cmp::max(size.width, metrics.str_width(line));
            size.height += line_height;
            if ProposedDimension::Exact(size.height) >= offer.height {
                break;
            }
        }

        ResolvedLayout {
            sublayouts: (),
            resolved_size: size.into(),
        }
    }
}

impl<'a, T: AsRef<str> + Clone, F: Font> Renderable for Text<'a, T, F> {
    type Renderables = render::Text<'a, T, F>;

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        _env: &impl LayoutEnvironment,
    ) -> Self::Renderables {
        render::Text {
            text: self.text.clone(),
            font: self.font,
            origin,
            size: layout.resolved_size.into(),
            alignment: self.alignment,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{
        environment::DefaultEnvironment,
        font::{Font, FontMetrics, FontRender},
        layout::Layout as _,
        primitives::{Dimensions, ProposedDimension, ProposedDimensions, Size},
        view::Text,
    };

    #[derive(Debug)]
    struct ArbitraryFont {
        metrics: ArbitraryFontMetrics,
    }

    impl ArbitraryFont {
        fn new(line_height: u32, character_width: u32) -> Self {
            Self {
                metrics: ArbitraryFontMetrics {
                    line_height,
                    character_width,
                },
            }
        }
    }

    impl Font for ArbitraryFont {
        fn metrics(&self) -> impl FontMetrics {
            &self.metrics
        }
    }

    impl crate::font::Sealed for ArbitraryFont {}

    impl<C> FontRender<C> for ArbitraryFont {
        fn draw(
            &self,
            _character: char,
            _color: C,
            _surface: &mut impl crate::surface::Surface<Color = C>,
        ) {
        }
    }

    #[derive(Debug)]
    struct ArbitraryFontMetrics {
        line_height: u32,
        character_width: u32,
    }

    impl FontMetrics for ArbitraryFontMetrics {
        fn rendered_size(&self, _: char) -> Size {
            Size::new(self.character_width, self.line_height)
        }

        fn default_line_height(&self) -> u32 {
            self.line_height
        }

        fn advance(&self, _: char) -> u32 {
            self.character_width
        }

        fn baseline(&self) -> u32 {
            self.line_height
        }
    }

    #[test]
    fn test_single_character() {
        let font = ArbitraryFont::new(10, 5);
        let text = Text::new("A", &font);
        let offer = Size::new(100, 100);
        let env = DefaultEnvironment::non_animated();
        let layout = text.layout(&offer.into(), &env);
        assert_eq!(layout.resolved_size, Dimensions::new(5, 10));
    }

    #[test]
    fn test_single_character_constrained() {
        let font = ArbitraryFont::new(10, 5);
        let text = Text::new("A", &font);
        let offer = Size::new(4, 10);
        let env = DefaultEnvironment::non_animated();
        let layout = text.layout(&offer.into(), &env);
        assert_eq!(layout.resolved_size, Dimensions::new(5, 10));
    }

    #[test]
    fn test_text_layout() {
        let font = ArbitraryFont::new(10, 5);
        let text = Text::new("Hello, world!", &font);
        let offer = Size::new(100, 100);
        let env = DefaultEnvironment::non_animated();
        let layout = text.layout(&offer.into(), &env);
        assert_eq!(layout.resolved_size, Dimensions::new(5 * 13, 10));
    }

    #[test]
    fn test_text_layout_wraps() {
        let font = ArbitraryFont::new(10, 5);
        let text = Text::new("Hello, world!", &font);
        let offer = Size::new(50, 100);
        let env = DefaultEnvironment::non_animated();
        let layout = text.layout(&offer.into(), &env);
        assert_eq!(layout.resolved_size, Dimensions::new(6 * 5, 20));
    }

    #[test]
    fn test_wraps_partial_words() {
        let font = ArbitraryFont::new(10, 5);
        let text = Text::new("123412341234", &font);
        let offer = Size::new(20, 100);
        let env = DefaultEnvironment::non_animated();
        let layout = text.layout(&offer.into(), &env);
        assert_eq!(layout.resolved_size, Dimensions::new(20, 30));
    }

    #[test]
    fn test_newline() {
        let font = ArbitraryFont::new(10, 5);
        let text = Text::new("1234\n12\n\n123\n", &font);
        let offer = Size::new(25, 100);
        let env = DefaultEnvironment::non_animated();
        let layout = text.layout(&offer.into(), &env);
        assert_eq!(layout.resolved_size, Dimensions::new(20, 40));
    }

    #[test]
    fn test_infinite_width() {
        let font = ArbitraryFont::new(1, 1);
        let text = Text::new("abc defg", &font);
        let offer = ProposedDimensions {
            width: ProposedDimension::Infinite,
            height: 100.into(),
        };
        let env = DefaultEnvironment::non_animated();
        let layout = text.layout(&offer, &env);
        assert_eq!(layout.resolved_size, Dimensions::new(8, 1));
    }

    #[test]
    fn test_compact_width() {
        let font = ArbitraryFont::new(1, 1);
        let text = Text::new("abc defg", &font);
        let offer = ProposedDimensions {
            width: ProposedDimension::Compact,
            height: 100.into(),
        };
        let env = DefaultEnvironment::non_animated();
        let layout = text.layout(&offer, &env);
        assert_eq!(layout.resolved_size, Dimensions::new(8, 1));
    }

    #[test]
    fn test_infinite_height() {
        let font = ArbitraryFont::new(1, 1);
        let text = Text::new("abc defg h", &font);
        let offer = ProposedDimensions {
            width: 10.into(),
            height: ProposedDimension::Infinite,
        };
        let env = DefaultEnvironment::non_animated();
        let layout = text.layout(&offer, &env);
        assert_eq!(layout.resolved_size, Dimensions::new(10, 1));
    }

    #[test]
    fn test_compact_height() {
        let font = ArbitraryFont::new(1, 1);
        let text = Text::new("abc defg h", &font);
        let offer = ProposedDimensions {
            width: 10.into(),
            height: ProposedDimension::Compact,
        };
        let env = DefaultEnvironment::non_animated();
        let layout = text.layout(&offer, &env);
        assert_eq!(layout.resolved_size, Dimensions::new(10, 1));
    }

    #[test]
    fn test_infinite_height_wrapping() {
        let font = ArbitraryFont::new(1, 1);
        let text = Text::new("abc defg hij", &font);
        let offer = ProposedDimensions {
            width: 10.into(),
            height: ProposedDimension::Infinite,
        };
        let env = DefaultEnvironment::non_animated();
        let layout = text.layout(&offer, &env);
        assert_eq!(layout.resolved_size, Dimensions::new(8, 2));
    }

    #[test]
    fn test_compact_height_wrapping() {
        let font = ArbitraryFont::new(1, 1);
        let text = Text::new("abc defg hij", &font);
        let offer = ProposedDimensions {
            width: 10.into(),
            height: ProposedDimension::Compact,
        };
        let env = DefaultEnvironment::non_animated();
        let layout = text.layout(&offer, &env);
        assert_eq!(layout.resolved_size, Dimensions::new(8, 2));
    }
}
