use crate::{
    environment::LayoutEnvironment,
    font::{CustomSize, Font, FontMetrics},
    layout::ResolvedLayout,
    primitives::{Point, ProposedDimension, ProposedDimensions, Size},
    render::{self},
    transition::Opacity,
    view::{ViewLayout, ViewMarker},
};
use core::fmt::Write;

mod character_wrap;
mod word_wrap;

pub use character_wrap::CharacterWrap;
pub use word_wrap::WordWrap;

/// The strategy to use when wrapping text.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WrapStrategy {
    /// Wrap at word/whitespace boundaries.
    Word,
    /// Wrap at character boundaries.
    Character,
}

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
pub struct Text<'a, T, F: Font> {
    #[allow(clippy::struct_field_names)]
    pub(crate) text: T,
    pub(crate) font: &'a F,
    pub(crate) attributes: F::Attributes,
    pub(crate) alignment: HorizontalTextAlignment,
    pub(crate) precise_character_bounds: bool,
    pub(crate) wrap: WrapStrategy,
}

#[derive(Debug, PartialEq, Eq)]
pub struct WrappedLine<'a> {
    pub content: &'a str,
    pub width: u32,
    pub precise_width: u32,
    pub min_x: i32,
    pub max_x: i32,
}

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct Sublayout {
    manual_offset: (i16, i16),
    wrap_size: (u16, u16),
    line_count: u16,
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

impl<'a, T: AsRef<str>, F: Font> Text<'a, T, F> {
    #[allow(missing_docs)]
    #[must_use]
    pub fn new(text: T, font: &'a F) -> Self {
        Self {
            text,
            font,
            attributes: F::Attributes::default(),
            alignment: HorizontalTextAlignment::default(),
            precise_character_bounds: false,
            wrap: WrapStrategy::Word,
        }
    }

    /// Sets the wrapping strategy for the text.
    #[must_use]
    pub fn with_wrap_strategy(mut self, strategy: WrapStrategy) -> Self {
        self.wrap = strategy;
        self
    }
}

/// Calculate the vertical extent (min y, max y) for a line of text.
/// This is used for first and last lines to determine vertical bounds.
fn calculate_vertical_extent(
    metrics: &impl FontMetrics,
    text: &str,
    y_offset: i32,
) -> Option<(i32, i32)> {
    if text.is_empty() {
        return None;
    }

    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    for ch in text.chars() {
        if let Some(char_bounds) = metrics.rendered_size(ch) {
            let top = char_bounds.origin.y + y_offset;
            let bottom = top + char_bounds.size.height as i32;
            min_y = core::cmp::min(min_y, top);
            max_y = core::cmp::max(max_y, bottom);
        }
    }

    if min_y <= max_y {
        Some((min_y, max_y))
    } else {
        None
    }
}

impl<'a, F: Font> Text<'a, (), F> {
    /// A convenience constructor for [`Text`] backed by an owned [`heapless::String<N, u8>`]
    /// and formatted with the result of [`format_args!`].
    ///
    /// Use [`Text::new_fmt_long<N, L>`] when more than 255 characters are needed.
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
    ) -> Text<'a, heapless::String<N, u8>, F> {
        const {
            assert!(
                u8::MAX as usize >= N,
                "N is larger than 255, use `Text::new_fmt_long::<N, L>` instead"
            );
        };
        let mut s = heapless::String::<N, u8>::new();
        _ = s.write_fmt(args);
        Text::new(s, font)
    }

    /// A convenience constructor for [`Text`] backed by an owned [`heapless::String<N, LenT>`]
    /// and formatted with the result of [`format_args!`].
    ///
    /// Use this variant when more than 255 characters are needed.
    ///
    /// # Examples
    ///
    /// ```
    /// # use buoyant::view::prelude::*;
    /// # use embedded_graphics::mono_font::ascii::FONT_9X15_BOLD;
    /// # use embedded_graphics::pixelcolor::Rgb888;
    /// #
    /// fn legal_disclaimer(content: &str) -> impl View<Rgb888, ()> + use<> {
    ///    Text::new_fmt_long::<10_000, u16>(format_args!("Nothing suspiscious here: {content}"), &FONT_9X15_BOLD)
    /// }
    /// ```
    pub fn new_fmt_long<const N: usize, LenT: heapless::LenType>(
        args: core::fmt::Arguments<'_>,
        font: &'a F,
    ) -> Text<'a, heapless::String<N, LenT>, F> {
        const {
            assert!(
                LenT::MAX_USIZE >= N,
                "N is larger than what `LenT` can hold, use a larger `LenT` type or reduce the capacity"
            );
        };

        let mut s = heapless::String::<N, LenT>::new();
        _ = s.write_fmt(args);
        Text::new(s, font)
    }
}

impl<T, F: Font> Text<'_, T, F> {
    /// Sets the alignment of multiline text.
    #[must_use]
    pub fn multiline_text_alignment(self, alignment: HorizontalTextAlignment) -> Self {
        Text { alignment, ..self }
    }

    /// Enable calculation of precise character boundaries.
    ///
    /// This option is particularly useful for displaying tightly bordered
    /// text.
    ///
    /// Not all fonts support precise character bounds.
    ///
    /// Note that when using precision bounds, the baselines of text
    /// arranged horizontally are no longer guaranteed to align.
    #[must_use]
    pub fn with_precise_bounds(mut self) -> Self {
        self.precise_character_bounds = true;
        self
    }
}

impl<T, F: Font<Attributes: CustomSize>> Text<'_, T, F> {
    /// Sets the font size
    #[must_use]
    pub fn with_font_size(self, size: u32) -> Self {
        Text {
            attributes: self.attributes.with_size(size),
            ..self
        }
    }
}

impl<T: PartialEq, F: Font> PartialEq for Text<'_, T, F> {
    fn eq(&self, other: &Self) -> bool {
        self.text == other.text
    }
}

impl<'a, T: Clone, F: Font> ViewMarker for Text<'a, T, F> {
    type Renderables = render::Text<'a, T, F>;
    type Transition = Opacity;
}

impl<Captures: ?Sized, T, F> ViewLayout<Captures> for Text<'_, T, F>
where
    T: AsRef<str> + Clone,
    F: Font,
{
    type Sublayout = Sublayout;
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
        let metrics = self.font.metrics(&self.attributes);
        let line_height = metrics.vertical_metrics().line_height();

        let max_line_count = match offer.height {
            ProposedDimension::Exact(h) => h / line_height,
            _ => u32::MAX,
        };

        let mut size = Size::zero();

        let mut whitespace = WordWrap::new(
            self.text.as_ref(),
            offer.width,
            &metrics,
            self.precise_character_bounds,
        );
        let mut word = CharacterWrap::new(
            self.text.as_ref(),
            offer.width,
            &metrics,
            self.precise_character_bounds,
        );
        let mut wrap = core::iter::from_fn(|| match self.wrap {
            WrapStrategy::Word => whitespace.next(),
            WrapStrategy::Character => word.next(),
        });

        let mut line_count: u32 = 0;

        // Iterate through lines, tracking width and horizontal extents
        // Always use advance-based width for wrapping consistency
        let mut max_precise_width = 0u32;
        let mut global_min_x = 0i32;
        let mut global_max_x = 0i32;
        let mut has_content = false;

        for line in (&mut wrap).take(max_line_count as usize) {
            line_count += 1;
            size.width = core::cmp::max(size.width, line.width);
            max_precise_width = core::cmp::max(max_precise_width, line.precise_width);

            // Track horizontal extents across all lines for precise bounds
            // The WrappedLine already calculated these during iteration
            if self.precise_character_bounds && !line.content.is_empty() {
                if has_content {
                    global_min_x = core::cmp::min(global_min_x, line.min_x);
                    global_max_x = core::cmp::max(global_max_x, line.max_x);
                } else {
                    global_min_x = line.min_x;
                    global_max_x = line.max_x;
                    has_content = true;
                }
            }
        }

        size.height = line_count * line_height;

        // Calculate vertical extent from first and last non-empty lines
        let mut min_y = 0i32;
        let mut max_y = 0i32;
        let mut has_vertical_extent = false;

        if self.precise_character_bounds {
            let (first_non_empty, last_non_empty) = match self.wrap {
                WrapStrategy::Word => (
                    whitespace.first_non_empty_line(),
                    whitespace.last_non_empty_line(),
                ),
                WrapStrategy::Character => {
                    (word.first_non_empty_line(), word.last_non_empty_line())
                }
            };

            if let Some((first_line, first_y)) = first_non_empty
                && let Some((first_min_y, first_max_y)) =
                    calculate_vertical_extent(&metrics, first_line, *first_y)
            {
                min_y = first_min_y;
                max_y = first_max_y;
                has_vertical_extent = true;
            }

            if let Some((last_line, last_y)) = last_non_empty
                && let Some((last_min_y, last_max_y)) =
                    calculate_vertical_extent(&metrics, last_line, *last_y)
            {
                if has_vertical_extent {
                    // Union with first line extent
                    min_y = core::cmp::min(min_y, last_min_y);
                    max_y = core::cmp::max(max_y, last_max_y);
                } else {
                    // Only last line exists (first was empty)
                    min_y = last_min_y;
                    max_y = last_max_y;
                    has_vertical_extent = true;
                }
            }
        }
        let mut manual_offset = Point::zero();
        // Track the original size before applying the precise bounds adjustment
        // This allows rendering to wrap lines in the correct amount of space
        let wrap_size = size;
        if self.precise_character_bounds && has_content && has_vertical_extent {
            // Calculate manual offset from the minimum x and y coordinates across all lines
            manual_offset = Point::new(-global_min_x, -min_y);

            // Use the horizontal extent across all lines and vertical extent from boundary lines
            // The width is calculated from the global min/max x, accounting for all lines
            let precise_width = (global_max_x - global_min_x) as u32;
            let precise_height = (max_y - min_y) as u32;

            size = Size::new(precise_width, precise_height);
        }

        let sublayouts = Sublayout {
            manual_offset: (manual_offset.x as i16, manual_offset.y as i16),
            wrap_size: (wrap_size.width as u16, wrap_size.height as u16),
            line_count: line_count as u16,
        };

        ResolvedLayout {
            sublayouts,
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
        let Sublayout {
            manual_offset,
            wrap_size,
            line_count,
        } = &layout.sublayouts;
        render::Text {
            origin: origin + Point::new(manual_offset.0.into(), manual_offset.1.into()),
            size: *wrap_size,
            font: self.font,
            text: self.text.clone(),
            attributes: self.attributes.clone(),
            alignment: self.alignment,
            max_lines: *line_count,
            wrap: self.wrap,
        }
    }
}

/// ```compile_fail
/// # use buoyant::view::prelude::*;
/// # use embedded_graphics::mono_font::ascii::FONT_9X15_BOLD;
/// let _ = Text::new_fmt::<256>(format_args!("abc {}", 123), &FONT_9X15_BOLD);
/// ```
///
/// ```compile_fail
/// # use buoyant::view::prelude::*;
/// # use embedded_graphics::mono_font::ascii::FONT_9X15_BOLD;
/// let _ = Text::new_fmt_long::<100_000, u16>(format_args!("abc {}", 123), &FONT_9X15_BOLD);
/// ```
#[expect(unused)]
struct CheckLengthCompileError;

#[cfg(test)]
mod test {
    use crate::{
        environment::DefaultEnvironment,
        font::{Font, FontMetrics, FontRender},
        primitives::{
            Dimensions, Point, ProposedDimension, ProposedDimensions, Size, geometry::Rectangle,
        },
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
        type Attributes = ();
        fn metrics(&self, _customization: &Self::Attributes) -> impl FontMetrics {
            &self.metrics
        }
    }

    impl crate::font::Sealed for ArbitraryFont {}

    impl<C> FontRender<C> for ArbitraryFont {
        fn draw(
            &self,
            _character: char,
            _offset: Point,
            _color: C,
            _background_color: Option<C>,
            _customization: &Self::Attributes,
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
        fn rendered_size(&self, _: char) -> Option<Rectangle> {
            Some(Rectangle::new(
                Point::zero(),
                Size::new(self.character_width, self.line_height),
            ))
        }

        fn vertical_metrics(&self) -> crate::font::VMetrics {
            crate::font::VMetrics {
                ascent: self.line_height as i32,
                descent: 0,
                line_spacing: 0,
            }
        }

        fn advance(&self, _: char) -> u32 {
            self.character_width
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

    #[test]
    fn test_height_cutoff() {
        let font = ArbitraryFont::new(1, 1);
        let text = Text::new("abc defg hij", &font).with_precise_bounds();
        let offer = ProposedDimensions {
            width: 3.into(),
            height: 2.into(),
        };
        let env = DefaultEnvironment::non_animated();
        let mut captures = ();
        let layout = text.layout(&offer, &env, &mut captures, &mut ());
        assert_eq!(layout.resolved_size, Dimensions::new(3, 2));
    }

    #[ignore = "Is there a use case where this matters?"]
    #[test]
    fn zero_height_lines_retain_width() {
        let font = ArbitraryFont::new(2, 1);
        let text = Text::new("abc defg hij", &font).with_precise_bounds();
        let offer = ProposedDimensions {
            width: 3.into(),
            height: 1.into(),
        };
        let env = DefaultEnvironment::non_animated();
        let mut captures = ();
        let layout = text.layout(&offer, &env, &mut captures, &mut ());
        assert_eq!(layout.resolved_size, Dimensions::new(3, 0));
    }
}
