use crate::{
    font::FontMetrics,
    primitives::{Point, ProposedDimension, Size, geometry::Rectangle},
    view::text::WrappedLine,
};

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
pub struct CharacterWrap<'a, F> {
    remaining: &'a str,
    available_width: ProposedDimension,
    font: &'a F,
    calculate_precise_bounds: bool,
    current_y: i32,
    first_non_empty_line: Option<(&'a str, i32)>,
    last_non_empty_line: Option<(&'a str, i32)>,
}

impl<'a, F: FontMetrics> CharacterWrap<'a, F> {
    pub fn new(
        text: &'a str,
        available_width: impl Into<ProposedDimension>,
        font: &'a F,
        calculate_precise_bounds: bool,
    ) -> Self {
        Self {
            remaining: text,
            available_width: available_width.into(),
            font,
            calculate_precise_bounds,
            current_y: 0,
            first_non_empty_line: None,
            last_non_empty_line: None,
        }
    }

    /// Get the first non-empty line and its Y offset.
    #[expect(clippy::ref_option)]
    pub fn first_non_empty_line(&self) -> &'_ Option<(&'a str, i32)> {
        &self.first_non_empty_line
    }

    /// Get the last non-empty line and its Y offset.
    #[expect(clippy::ref_option)]
    pub fn last_non_empty_line(&self) -> &'_ Option<(&'a str, i32)> {
        &self.last_non_empty_line
    }

    /// Calculate precise width for a line by checking only first and last character.
    /// This is much faster than unioning all character bounds.
    /// Returns the tight width by subtracting leading/trailing space from advance-based width.
    /// The advance width is used for whitespace characters which have no intrinsic size
    fn calculate_precise_width_and_extents(
        &self,
        text: &str,
        advance_width: u32,
    ) -> (u32, i32, i32) {
        if advance_width == 0 {
            return (0, 0, 0);
        }

        let Some(first_char) = text.chars().next() else {
            return (0, 0, 0);
        };

        let Some(last_char) = text.chars().next_back() else {
            return (0, 0, 0);
        };

        // Get rendered bounds for first and last characters
        let first_bounds = self.font.rendered_size(first_char).unwrap_or_else(|| {
            Rectangle::new(Point::zero(), Size::new(self.font.advance(first_char), 0))
        });
        let last_char_advance = self.font.advance(last_char);
        let last_bounds = self
            .font
            .rendered_size(last_char)
            .unwrap_or_else(|| Rectangle::new(Point::zero(), Size::new(last_char_advance, 0)));

        let min_x = first_bounds.origin.x;
        let max_x = advance_width as i32 - last_char_advance as i32
            + last_bounds.origin.x
            + last_bounds.size.width as i32;

        let precise_width =
            (advance_width as i32 - first_bounds.origin.x - last_char_advance as i32
                + last_bounds.origin.x
                + last_bounds.size.width as i32)
                .max(0) as u32;

        (precise_width, min_x, max_x)
    }

    /// Helper to create a `WrappedLine` with appropriate precise width.
    fn make_wrapped_line(&mut self, content: &'a str, width: u32) -> WrappedLine<'a> {
        let (precise_width, min_x, max_x) = if self.calculate_precise_bounds {
            self.calculate_precise_width_and_extents(content, width)
        } else {
            (0, 0, 0)
        };

        // Track first and last non-empty lines for later vertical bounds calculation
        if self.calculate_precise_bounds && !content.is_empty() {
            if self.first_non_empty_line.is_none() {
                self.first_non_empty_line = Some((content, self.current_y));
            }
            self.last_non_empty_line = Some((content, self.current_y));
        }

        self.current_y += self.font.vertical_metrics().line_height() as i32;

        WrappedLine {
            content,
            width,
            precise_width,
            min_x,
            max_x,
        }
    }
}

impl<'a, F: FontMetrics + 'a> Iterator for CharacterWrap<'a, F> {
    type Item = WrappedLine<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        // Return as many characters as fit within available width, always at least one (or exit)
        let mut remaining_iter = self.remaining.char_indices();
        let (mut split_pos, mut ch) = remaining_iter.next()?;

        let mut width = self.font.advance(ch);

        loop {
            // Newlines always break the line
            if ch == '\n' {
                let (line, rest) = self.remaining.split_at(split_pos);
                // Skip the newline character itself
                // This is safe because we know \n is 1 byte
                self.remaining = &rest[1..];

                return Some(self.make_wrapped_line(line, width));
            }

            if let Some((idx, character)) = remaining_iter.next() {
                let new_width = width + self.font.advance(character);
                ch = character;
                split_pos = idx;
                if ProposedDimension::Exact(new_width) > self.available_width {
                    break;
                }
                width = new_width;
            } else {
                split_pos = self.remaining.len();
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

            return Some(self.make_wrapped_line(line, width));
        }

        let (result, rest) = self.remaining.split_at(split_pos);
        self.remaining = rest;
        Some(self.make_wrapped_line(result, width))
    }
}

#[cfg(test)]
mod tests {
    use super::CharacterWrap;
    use crate::font::{CharacterBufferFont, Font, FontMetrics, FontRender};
    use crate::primitives::Size;
    use crate::primitives::geometry::Rectangle;
    use crate::primitives::{Point, ProposedDimension};
    use crate::render_target::Surface;
    use std::vec::Vec;
    use std::{self, vec};

    static FONT: CharacterBufferFont = CharacterBufferFont;

    #[test]
    fn single_word() {
        let metrics = &FONT.metrics(&());
        let wrap = CharacterWrap::new("hello", 10, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<&str>>(),
            vec!["hello"]
        );
    }

    #[test]
    fn breaks_anywhere_not_at_space() {
        let metrics = &FONT.metrics(&());
        // "hello world" is 11 chars -> width 11
        // @typos-ignore
        // available_width = 10 -> should break after 10 bytes: "hello worl", "d"
        let wrap = CharacterWrap::new("hello world", 10, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<&str>>(),
            // @typos-ignore
            vec!["hello worl", "d"]
        );
    }

    #[test]
    fn partial_words_are_wrapped_2() {
        let metrics = &FONT.metrics(&());
        let wrap = CharacterWrap::new("hello world", 2, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            vec!["he", "ll", "o ", "wo", "rl", "d"]
        );
    }

    #[test]
    fn newlines_are_respected() {
        let metrics = &FONT.metrics(&());
        let wrap = CharacterWrap::new("hello\nworld", 3, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            // @typos-ignore
            vec!["hel", "lo", "wor", "ld"]
        );
    }

    #[test]
    fn compact_and_infinite_do_not_wrap_unless_newline() {
        let metrics = &FONT.metrics(&());
        let wrap = CharacterWrap::new("hello world", ProposedDimension::Compact, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            vec!["hello world"]
        );

        let wrap = CharacterWrap::new("hello\nworld", ProposedDimension::Compact, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            vec!["hello", "world"]
        );

        let wrap = CharacterWrap::new("hello world", ProposedDimension::Infinite, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            vec!["hello world"]
        );
    }

    // Optional: variable-width font test (keeps behavior when advance != 1)
    struct VariableWidthFont;
    struct VariableWidthFontMetrics;

    impl FontMetrics for VariableWidthFontMetrics {
        fn rendered_size(&self, c: char) -> Option<Rectangle> {
            let size = Size::new(self.advance(c), 1);
            Some(Rectangle::new(Point::zero(), size))
        }

        fn vertical_metrics(&self) -> crate::font::VMetrics {
            crate::font::VMetrics {
                ascent: 1,
                descent: 0,
                line_spacing: 0,
            }
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
    }

    impl Font for VariableWidthFont {
        type Attributes = ();
        fn metrics(&self, _attributes: &Self::Attributes) -> impl crate::font::FontMetrics {
            VariableWidthFontMetrics
        }
    }

    impl crate::font::Sealed for VariableWidthFont {}

    impl<C> FontRender<C> for VariableWidthFont {
        fn draw(
            &self,
            _: char,
            _offset: Point,
            _: C,
            _: Option<C>,
            _attributes: &Self::Attributes,
            _: &mut impl Surface<Color = C>,
        ) {
        }
    }

    #[test]
    fn variable_width_respected() {
        let metrics = &VariableWidthFont.metrics(&());
        // digits have widths equal to their value, spaces width 2.
        // -----
        // 1  22
        //   333
        //
        // 4444
        //
        // 55555
        //
        // 666666
        let wrap = CharacterWrap::new("1 2 3 4 5 6", 5, metrics, false);
        // We ensure it breaks according to the accumulated widths (exact expected values may differ
        // depending on how you count digits); test kept simple to show behavior composes with metric.
        let parts = wrap.map(|l| l.content).collect::<Vec<_>>();
        assert_eq!(parts, vec!["1 2", " 3", " ", "4", " ", "5", " ", "6"]);
    }

    #[test]
    fn zero_sized_offer() {
        // The behavior of newlines in zero-width offers should be the same as with 1-width offers
        let metrics = &FONT.metrics(&());
        let wrap_0 = CharacterWrap::new("he\nllo", 0, metrics, false);
        assert_eq!(
            wrap_0.map(|l| l.content).collect::<Vec<_>>(),
            vec!["h", "e", "l", "l", "o"]
        );
        let wrap_1 = CharacterWrap::new("he\nllo", 1, metrics, false);
        assert_eq!(
            wrap_1.map(|l| l.content).collect::<Vec<_>>(),
            vec!["h", "e", "l", "l", "o"]
        );
    }

    #[test]
    fn natural_breaks_consume_explicit_newlines() {
        // When breaking naturally before a newline, it should not produce an extra line,
        // except for a trailing newline
        let metrics = &FONT.metrics(&());
        let wrap = CharacterWrap::new("1\n\n3\n", 1, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            vec!["1", "", "3", ""]
        );
    }

    #[test]
    fn unicode_wraps_correctly() {
        let metrics = &FONT.metrics(&());
        let wrap = CharacterWrap::new("rº🦀_🦀 🦀\nyº ºº\t🦀", 4, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            vec!["rº🦀_", "🦀 🦀", "yº º", "º\t🦀"]
        );
    }
}
