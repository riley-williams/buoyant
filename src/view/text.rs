use crate::{
    environment::LayoutEnvironment,
    font::{Font, FontMetrics},
    layout::ResolvedLayout,
    primitives::{Point, ProposedDimension, ProposedDimensions, Size},
    render::{self},
    transition::Opacity,
    view::{ViewLayout, ViewMarker},
};
use core::fmt::Write;

mod break_word_wrap;
mod whitespace_wrap;

pub use break_word_wrap::BreakWordWrap;
pub use whitespace_wrap::WhitespaceWrap;

/// The strategy to use when wrapping text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WrapStrategy {
    /// Wrap at whitespace boundaries.
    Whitespace,
    /// Wrap at character boundaries.
    BreakWord,
}

// W is hardcoded elsewhere to WhitespaceWrap, leaving generic for future fix

/// Displays text in a given font.
///
/// Multiline text is leading aligned by default.
///
/// # Examples
///
/// ```
/// use buoyant::view::prelude::*;
/// use embedded_graphics::pixelcolor::Rgb888;
/// use embedded_graphics::mono_font::ascii::FONT_9X15;
///
/// fn view() -> impl View<Rgb888, ()> {
///     Text::new("Hello, world!", &FONT_9X15)
/// }
/// ```
#[derive(Debug, Clone)]
pub struct Text<'a, T, F> {
    #[allow(clippy::struct_field_names)]
    pub(crate) text: T,
    pub(crate) font: &'a F,
    pub(crate) alignment: HorizontalTextAlignment,
    pub(crate) wrap: WrapStrategy,
}

/// The alignment of multiline text. This has no effect on single-line text.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum HorizontalTextAlignment {
    /// Align multiline text to the leading edge.
    #[default]
    Leading,
    /// Center multiline text.
    Center,
    /// Align multiline text to the trailing edge.
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
    #[allow(missing_docs)]
    #[must_use]
    pub fn new(text: T, font: &'a F) -> Self {
        Self {
            text,
            font,
            alignment: HorizontalTextAlignment::default(),
            wrap: WrapStrategy::Whitespace,
        }
    }
    /// Sets the wrapping strategy for the text.
    #[must_use]
    pub fn with_wrap_strategy(mut self, strategy: WrapStrategy) -> Self {
        self.wrap = strategy;
        self
    }
}

impl<'a, F> Text<'a, (), F> {
    /// A convenience constructor for [`Text`] backed by an owned [`heapless::String<N>`]
    /// and formatted with the result of [`format_args!`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use buoyant::view::prelude::*;
    /// # use embedded_graphics::mono_font::ascii::FONT_9X15_BOLD;
    /// # use embedded_graphics::pixelcolor::Rgb888;
    /// #
    /// fn counter(count: i32) -> impl View<Rgb888, ()> {
    ///    Text::new_fmt::<32>(format_args!("Count: {count}"), &FONT_9X15_BOLD)
    /// }
    /// ```
    pub fn new_fmt<const N: usize>(
        args: core::fmt::Arguments<'_>,
        font: &'a F,
    ) -> Text<'a, heapless::String<N>, F> {
        let mut s = heapless::String::<N>::new();
        _ = s.write_fmt(args);
        Text::new(s, font)
    }
}

impl<T, F> Text<'_, T, F> {
    /// Sets the alignment of multiline text.
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

impl<'a, T: Clone, F> ViewMarker for Text<'a, T, F> {
    type Renderables = render::Text<'a, T, F, 8>;
    type Transition = Opacity;
}

impl<Captures: ?Sized, T, F> ViewLayout<Captures> for Text<'_, T, F>
where
    T: AsRef<str> + Clone,
    F: Font,
{
    type Sublayout = heapless::Vec<crate::render::text::Line, 8>;
    type State = ();

    fn transition(&self) -> Self::Transition {
        Opacity
    }

    fn build_state(&self, _captures: &mut Captures) -> Self::State {}

    fn layout(
        &self,
        offer: &ProposedDimensions,
        _env: &impl LayoutEnvironment,
        _captures: &mut Captures,
        _state: &mut Self::State,
    ) -> ResolvedLayout<Self::Sublayout> {
        let metrics = self.font.metrics();
        let line_height = metrics.default_line_height();
        let mut size = Size::zero();
        let line_ranges = heapless::Vec::new();
        let mut whitespace = WhitespaceWrap::new(self.text.as_ref(), offer.width, &metrics);
        let mut word = BreakWordWrap::new(self.text.as_ref(), offer.width, &metrics);
        let wrap = core::iter::from_fn(|| match self.wrap {
            WrapStrategy::Whitespace => whitespace.next(),
            WrapStrategy::BreakWord => word.next(),
        });
        // TODO: actually calculate this
        for line in wrap {
            // FIXME: WhitespaceWrap could return a `Line` type with more information
            // it's already done a width calculation
            size.width = core::cmp::max(size.width, metrics.str_width(line));
            size.height += line_height;
            if ProposedDimension::Exact(size.height) >= offer.height {
                break;
            }
        }

        ResolvedLayout {
            sublayouts: line_ranges,
            resolved_size: size.into(),
        }
    }

    fn render_tree(
        &self,
        layout: &ResolvedLayout<Self::Sublayout>,
        origin: Point,
        _env: &impl LayoutEnvironment,
        _captures: &mut Captures,
        _state: &mut Self::State,
    ) -> Self::Renderables {
        render::Text::new(
            origin,
            layout.resolved_size.into(),
            self.font,
            self.text.clone(),
            self.alignment,
            layout.sublayouts.clone(),
            self.wrap,
        )
    }
}

#[cfg(test)]
mod test {
    use crate::{
        environment::DefaultEnvironment,
        font::{Font, FontMetrics, FontRender},
        primitives::{Dimensions, ProposedDimension, ProposedDimensions, Size},
        view::{Text, ViewLayout},
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
        let mut captures = ();
        let layout = text.layout(&offer.into(), &env, &mut captures, &mut ());
        assert_eq!(layout.resolved_size, Dimensions::new(5, 10));
    }

    #[test]
    fn test_single_character_constrained() {
        let font = ArbitraryFont::new(10, 5);
        let text = Text::new("A", &font);
        let offer = Size::new(4, 10);
        let env = DefaultEnvironment::non_animated();
        let mut captures = ();
        let layout = text.layout(&offer.into(), &env, &mut captures, &mut ());
        assert_eq!(layout.resolved_size, Dimensions::new(5, 10));
    }

    #[test]
    fn test_text_layout() {
        let font = ArbitraryFont::new(10, 5);
        let text = Text::new("Hello, world!", &font);
        let offer = Size::new(100, 100);
        let env = DefaultEnvironment::non_animated();
        let mut captures = ();
        let layout = text.layout(&offer.into(), &env, &mut captures, &mut ());
        assert_eq!(layout.resolved_size, Dimensions::new(5 * 13, 10));
    }

    #[test]
    fn test_text_layout_wraps() {
        let font = ArbitraryFont::new(10, 5);
        let text = Text::new("Hello, world!", &font);
        let offer = Size::new(50, 100);
        let env = DefaultEnvironment::non_animated();
        let mut captures = ();
        let layout = text.layout(&offer.into(), &env, &mut captures, &mut ());
        assert_eq!(layout.resolved_size, Dimensions::new(6 * 5, 20));
    }

    #[test]
    fn test_wraps_partial_words() {
        let font = ArbitraryFont::new(10, 5);
        let text = Text::new("123412341234", &font);
        let offer = Size::new(20, 100);
        let env = DefaultEnvironment::non_animated();
        let mut captures = ();
        let layout = text.layout(&offer.into(), &env, &mut captures, &mut ());
        assert_eq!(layout.resolved_size, Dimensions::new(20, 30));
    }

    #[test]
    fn test_newline() {
        let font = ArbitraryFont::new(10, 5);
        let text = Text::new("1234\n12\n\n123\n", &font);
        let offer = Size::new(25, 100);
        let env = DefaultEnvironment::non_animated();
        let mut captures = ();
        let layout = text.layout(&offer.into(), &env, &mut captures, &mut ());
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
        let mut captures = ();
        let layout = text.layout(&offer, &env, &mut captures, &mut ());
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
        let mut captures = ();
        let layout = text.layout(&offer, &env, &mut captures, &mut ());
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
        let mut captures = ();
        let layout = text.layout(&offer, &env, &mut captures, &mut ());
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
        let mut captures = ();
        let layout = text.layout(&offer, &env, &mut captures, &mut ());
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
        let mut captures = ();
        let layout = text.layout(&offer, &env, &mut captures, &mut ());
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
        let mut captures = ();
        let layout = text.layout(&offer, &env, &mut captures, &mut ());
        assert_eq!(layout.resolved_size, Dimensions::new(8, 2));
    }
}
