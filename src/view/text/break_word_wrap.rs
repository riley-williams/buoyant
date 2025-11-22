use crate::{font::FontMetrics, primitives::ProposedDimension};

/// Breaks lines at maximum width, ignoring word boundaries
///
/// Example:
///
/// "Build a bunch of buoyant boats"
///
/// Breaking at 7 characters wide will produce:
///
/// "Build a"
/// " bunch "
/// "of buoy"
/// "ant boa"
/// "ts"
#[derive(Debug, Clone)]
pub struct BreakWordWrap<'a, F> {
    remaining: &'a str,
    available_width: ProposedDimension,
    font: &'a F,
}

impl<'a, F: FontMetrics> BreakWordWrap<'a, F> {
    pub fn new(text: &'a str, available_width: impl Into<ProposedDimension>, font: &'a F) -> Self {
        Self {
            remaining: text,
            available_width: available_width.into(),
            font,
        }
    }
}

impl<'a, F: FontMetrics + 'a> Iterator for BreakWordWrap<'a, F> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        // Return as many characters as fit within available width, always at least one (or exit)
        let mut remaining_iter = self.remaining.char_indices();
        let (mut split_pos, mut ch) = remaining_iter.next()?;

        let mut width = 0u32;

        loop {
            // Newlines always break the line
            if ch == '\n' {
                let (line, rest) = self.remaining.split_at(split_pos);
                // Skip the newline character itself
                // This is safe because we know \n is 1 byte
                self.remaining = &rest[1..];

                return Some(line);
            }
            width += self.font.advance(ch);

            if let Some((idx, character)) = remaining_iter.next() {
                split_pos = idx;
                ch = character;
            } else {
                split_pos = self.remaining.len();
                break;
            }
            if ProposedDimension::Exact(width) >= self.available_width {
                break;
            }
        }

        // If the next character is a newline, consume it as well because we
        // are naturally breaking here. However if this is the last character,
        // We should still output one more empty line
        if ch == '\n' && split_pos != self.remaining.len() - 1 {
            let (line, rest) = self.remaining.split_at(split_pos);
            // Skip the newline character itself
            // This is safe because we know \n is 1 byte
            self.remaining = &rest[1..];

            return Some(line);
        }

        let (result, rest) = self.remaining.split_at(split_pos);
        self.remaining = rest;
        Some(result)
    }
}

#[cfg(test)]
mod tests {
    use super::BreakWordWrap;
    use crate::font::{CharacterBufferFont, Font, FontMetrics, FontRender};
    use crate::primitives::Size;
    use crate::primitives::geometry::Rectangle;
    use crate::primitives::{Point, ProposedDimension};
    use crate::surface::Surface;
    use std::vec::Vec;
    use std::{self, vec};

    static FONT: CharacterBufferFont = CharacterBufferFont;

    #[test]
    fn single_word() {
        let metrics = &FONT.metrics();
        let wrap = BreakWordWrap::new("hello", 10, metrics);
        assert_eq!(wrap.collect::<Vec<&str>>(), vec!["hello"]);
    }

    #[test]
    fn breaks_anywhere_not_at_space() {
        let metrics = &FONT.metrics();
        // "hello world" is 11 chars -> width 11
        // available_width = 10 -> should break after 10 bytes: "hello worl", "d"
        let wrap = BreakWordWrap::new("hello world", 10, metrics);
        assert_eq!(wrap.collect::<Vec<&str>>(), vec!["hello worl", "d"]);
    }

    #[test]
    fn partial_words_are_wrapped_2() {
        let metrics = &FONT.metrics();
        let wrap = BreakWordWrap::new("hello world", 2, metrics);
        assert_eq!(
            wrap.collect::<Vec<_>>(),
            vec!["he", "ll", "o ", "wo", "rl", "d"]
        );
    }

    #[test]
    fn newlines_are_respected() {
        let metrics = &FONT.metrics();
        let wrap = BreakWordWrap::new("hello\nworld", 3, metrics);
        assert_eq!(wrap.collect::<Vec<_>>(), vec!["hel", "lo", "wor", "ld"]);
    }

    #[test]
    fn compact_and_infinite_do_not_wrap_unless_newline() {
        let metrics = &FONT.metrics();
        let wrap = BreakWordWrap::new("hello world", ProposedDimension::Compact, metrics);
        assert_eq!(wrap.collect::<Vec<_>>(), vec!["hello world"]);

        let wrap = BreakWordWrap::new("hello\nworld", ProposedDimension::Compact, metrics);
        assert_eq!(wrap.collect::<Vec<_>>(), vec!["hello", "world"]);

        let wrap = BreakWordWrap::new("hello world", ProposedDimension::Infinite, metrics);
        assert_eq!(wrap.collect::<Vec<_>>(), vec!["hello world"]);
    }

    // Optional: variable-width font test (keeps behavior when advance != 1)
    struct VariableWidthFont;
    struct VariableWidthFontMetrics;

    impl FontMetrics for VariableWidthFontMetrics {
        fn rendered_size(&self, c: char) -> Option<Rectangle> {
            let size = Size::new(self.advance(c), 1);
            Some(Rectangle::new(Point::zero(), size))
        }

        fn default_line_height(&self) -> u32 {
            1
        }

        fn advance(&self, character: char) -> u32 {
            if character.is_whitespace() {
                2
            } else if character.is_ascii_digit() {
                character.to_digit(10).unwrap_or(1)
            } else {
                1
            }
        }

        fn maximum_character_size(&self) -> Size {
            Size::new(1, 1)
        }
    }

    impl Font for VariableWidthFont {
        fn metrics(&self) -> impl crate::font::FontMetrics {
            VariableWidthFontMetrics
        }
    }

    impl crate::font::Sealed for VariableWidthFont {}

    impl<C> FontRender<C> for VariableWidthFont {
        fn draw(&self, _: char, _: C, _: &mut impl Surface<Color = C>) {}
    }

    #[test]
    fn variable_width_respected() {
        let metrics = &VariableWidthFont.metrics();
        // digits have widths equal to their value, spaces width 2.
        // text "1 2 3 4 5 6" -> widths: 1,2,2,3,2,4,2,5,2,6 (approx; message is conceptual)
        let wrap = BreakWordWrap::new("1 2 3 4 5 6", 5, metrics);
        // We ensure it breaks according to the accumulated widths (exact expected values may differ
        // depending on how you count digits); test kept simple to show behavior composes with metric.
        let parts = wrap.collect::<Vec<_>>();
        assert_eq!(parts, vec!["1 2", " 3", " 4", " 5", " 6"]);
    }

    #[test]
    fn zero_sized_offer() {
        // The behavior of newlines in zero-width offers should be the same as with 1-width offers
        let metrics = &FONT.metrics();
        let wrap_0 = BreakWordWrap::new("he\nllo", 0, metrics);
        assert_eq!(wrap_0.collect::<Vec<_>>(), vec!["h", "e", "l", "l", "o"]);
        let wrap_1 = BreakWordWrap::new("he\nllo", 1, metrics);
        assert_eq!(wrap_1.collect::<Vec<_>>(), vec!["h", "e", "l", "l", "o"]);
    }

    #[test]
    fn natural_breaks_consume_explicit_newlines() {
        // When breaking naturally before a newline, it should not produce an extra line,
        // except for a trailing newline
        let metrics = &FONT.metrics();
        let wrap = BreakWordWrap::new("1\n\n3\n", 1, metrics);
        assert_eq!(wrap.collect::<Vec<_>>(), vec!["1", "", "3", ""]);
    }

    #[test]
    fn unicode_wraps_correctly() {
        let metrics = &FONT.metrics();
        let wrap = BreakWordWrap::new("rº🦀_🦀 🦀\nyº ºº\t🦀", 4, metrics);
        assert_eq!(
            wrap.collect::<Vec<_>>(),
            vec!["rº🦀_", "🦀 🦀", "yº º", "º\t🦀"]
        );
    }
}
