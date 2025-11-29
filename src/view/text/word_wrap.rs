use crate::{
    font::FontMetrics,
    primitives::{Point, ProposedDimension, Size, geometry::Rectangle},
    view::text::WrappedLine,
};

#[derive(Debug, Clone)]
pub struct WordWrap<'a, F> {
    remaining: &'a str,
    overflow: &'a str,
    available_width: ProposedDimension,
    font: &'a F,
    calculate_precise_bounds: bool,
    current_y: i32,
    first_non_empty_line: Option<(&'a str, i32)>,
    last_non_empty_line: Option<(&'a str, i32)>,
}

impl<'a, F: FontMetrics> WordWrap<'a, F> {
    pub fn new(
        text: &'a str,
        available_width: impl Into<ProposedDimension>,
        font: &'a F,
        calculate_precise_bounds: bool,
    ) -> Self {
        Self {
            remaining: text,
            overflow: "",
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

    /// Calculate width for a line when we don't already have it from iteration.
    fn calculate_width(&self, text: &str) -> u32 {
        text.chars().map(|ch| self.font.advance(ch)).sum()
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

        self.current_y += self.font.default_line_height() as i32;

        WrappedLine {
            content,
            width,
            precise_width,
            min_x,
            max_x,
        }
    }

    /// Helper function to find force split position, returns `(split_pos, width_up_to_split)`
    fn find_split_pos(&self, text: &str) -> Option<(usize, u32)> {
        let mut width = 0;
        for (pos, ch) in text.char_indices() {
            let char_width = self.font.advance(ch);
            if ProposedDimension::Exact(width + char_width) > self.available_width {
                return Some((if pos > 0 { pos } else { 1 }, width));
            }
            width += char_width;
        }
        None
    }
}

impl<'a, F: FontMetrics + 'a> Iterator for WordWrap<'a, F> {
    type Item = WrappedLine<'a>;

    #[allow(clippy::too_many_lines)]
    fn next(&mut self) -> Option<Self::Item> {
        // Handle overflow first
        if !self.overflow.is_empty() {
            // Check if overflow needs to be split further
            if let Some((split_pos, width)) = self.find_split_pos(self.overflow) {
                let (result, rest) = self.overflow.split_at(split_pos);
                self.overflow = rest;
                return Some(self.make_wrapped_line(result, width));
            }
            let result = self.overflow;
            self.overflow = "";
            let width = self.calculate_width(result);
            return Some(self.make_wrapped_line(result, width));
        }

        // Return None if no more text
        if self.remaining.is_empty() {
            return None;
        }

        let mut width = 0;
        let mut last_space: Option<(usize, u32)> = None;

        // Single pass through the string to find split points
        for (pos, ch) in self.remaining.char_indices() {
            // Check for newline first
            if ch == '\n' {
                let (line, rest) = self.remaining.split_at(pos);
                // This is safe because ch == \n which is 1 byte
                self.remaining = &rest[1..];

                // Handle empty lines and spaces after newlines
                if line.is_empty() {
                    return Some(self.make_wrapped_line(line, 0));
                }

                // Check if the line before newline needs force-splitting
                if let Some((split_pos, width_before_split)) = self.find_split_pos(line) {
                    let (result, rest) = line.split_at(split_pos);
                    self.overflow = rest;
                    return Some(self.make_wrapped_line(result, width_before_split));
                }

                let line = line.trim_end();
                // Use width tracked up to the newline, then recalculate after trim
                let line_width = self.calculate_width(line);
                return Some(self.make_wrapped_line(line, line_width));
            }

            let char_width = self.font.advance(ch);

            if ch.is_whitespace() {
                last_space = Some((pos, width));
            }

            width += char_width;

            // Check for force split
            if ProposedDimension::Exact(width) > self.available_width {
                if let Some((space_pos, _width_at_space)) = last_space {
                    // Split at last space
                    let (result, rest) = self.remaining.split_at(space_pos);
                    self.remaining = rest.trim_start();
                    let result = result.trim_end();
                    // Use width we tracked up to the space, then recalculate after trim
                    // (trim_end may remove characters so we need to recalculate)
                    let result_width = self.calculate_width(result);
                    return Some(self.make_wrapped_line(result, result_width));
                }
                // Force split the word
                let split_pos = if pos > 0 {
                    pos
                } else {
                    let Some(p) = self.remaining.char_indices().nth(1) else {
                        let last_char = self.remaining;
                        self.remaining = "";
                        let char_width = self.calculate_width(last_char);
                        return Some(self.make_wrapped_line(last_char, char_width));
                    };
                    p.0
                };
                let (result, rest) = self.remaining.split_at(split_pos);
                self.remaining = rest;
                // width - char_width is the width up to (but not including) the current char
                let result_width = width - char_width;
                return Some(self.make_wrapped_line(result, result_width));
            }
        }

        // Handle whitespace-only remaining text
        if self.remaining.chars().all(char::is_whitespace) {
            let mut end = self.remaining.len();
            let mut width = 0;
            for (pos, ch) in self.remaining.char_indices() {
                let char_width = self.font.advance(ch);
                if ProposedDimension::Exact(width + char_width) > self.available_width {
                    end = pos;
                    break;
                }
                width += char_width;
            }
            let result = &self.remaining[..end];
            self.remaining = "";
            return Some(self.make_wrapped_line(result, width));
        }

        // No wrap needed - return all remaining text
        let result = self.remaining.trim_end(); // FIXME: Is this right?
        self.remaining = "";
        // Use the width we've been tracking
        Some(self.make_wrapped_line(result, width))
    }
}

#[cfg(test)]
mod tests {
    use crate::environment::DefaultEnvironment;
    use crate::font::{Font, FontMetrics, FontRender};
    use crate::primitives::geometry::Rectangle;
    use crate::primitives::{Point, ProposedDimensions, Size};
    use crate::surface::Surface;
    use crate::view::{Text, ViewLayout};
    use crate::{font::CharacterBufferFont, primitives::ProposedDimension};
    use core::cell::RefCell;
    use std::vec;
    use std::vec::Vec;
    // a basic font for which all characters are 1 unit wide
    static FONT: CharacterBufferFont = CharacterBufferFont;

    /// Helper function to calculate expected precise width for a line of text.
    /// This checks first and last non-whitespace characters to determine tight width.
    /// Returns 0 for empty or whitespace-only lines.
    fn calculate_expected_precise_width(text: &str, metrics: &impl FontMetrics) -> u32 {
        if text.is_empty() {
            return 0;
        }

        let mut chars = text.chars().peekable();

        // Find first non-whitespace character
        let mut advance = 0u32;
        let mut first_char = None;
        let mut first_advance = 0u32;

        for ch in chars.by_ref() {
            if !ch.is_whitespace() {
                first_char = Some(ch);
                first_advance = advance;
                break;
            }
            advance += metrics.advance(ch);
        }

        let Some(first_char) = first_char else {
            return 0; // All whitespace
        };

        // Find last non-whitespace character
        let mut last_char = first_char;
        let mut last_advance = first_advance;
        advance = first_advance + metrics.advance(first_char);

        for ch in chars {
            if !ch.is_whitespace() {
                last_char = ch;
                last_advance = advance;
            }
            advance += metrics.advance(ch);
        }

        // Get rendered bounds
        let Some(first_bounds) = metrics.rendered_size(first_char) else {
            return 0;
        };
        let Some(last_bounds) = metrics.rendered_size(last_char) else {
            return 0;
        };

        let left_offset = first_bounds.origin.x;
        let right_extent =
            last_advance as i32 + last_bounds.origin.x + last_bounds.size.width as i32;

        (right_extent - first_advance as i32 - left_offset) as u32
    }

    #[test]
    fn empty_text() {
        let metrics = &FONT.metrics();
        let wrap = super::WordWrap::new("", 10, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<&str>>(),
            Vec::<&str>::new()
        );
    }

    #[ignore = "Not sure how much I care about this behavior"]
    #[test]
    fn only_whitespace_lines_are_retained_up_to_wrapping_width() {
        let metrics = &FONT.metrics();
        let wrap = super::WordWrap::new(" ", 5, metrics, false);
        assert_eq!(wrap.map(|l| l.content).collect::<Vec<_>>(), vec![" "]);
        let wrap = super::WordWrap::new("    ", 5, metrics, false);
        assert_eq!(wrap.map(|l| l.content).collect::<Vec<_>>(), vec!["    "]);
        let wrap = super::WordWrap::new("     ", 5, metrics, false);
        assert_eq!(wrap.map(|l| l.content).collect::<Vec<_>>(), vec!["     "]);
        let wrap = super::WordWrap::new("      ", 5, metrics, false);
        assert_eq!(wrap.map(|l| l.content).collect::<Vec<_>>(), vec!["     "]);
        let wrap = super::WordWrap::new("       ", 5, metrics, false);
        assert_eq!(wrap.map(|l| l.content).collect::<Vec<_>>(), vec!["     "]);
    }

    #[ignore = "Not sure how much I care about this behavior"]
    #[test]
    fn only_whitespace_lines_are_retained_up_to_wrapping_width_after_newline() {
        let metrics = &FONT.metrics();
        let wrap = super::WordWrap::new("hello\n ", 5, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            vec!["hello", " "]
        );
        let wrap = super::WordWrap::new("hello\n    ", 5, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            vec!["hello", "    "]
        );
        let wrap = super::WordWrap::new("hello\n     ", 5, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            vec!["hello", "     "]
        );
        let wrap = super::WordWrap::new("hello\n      ", 5, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            vec!["hello", "     "]
        );
        let wrap = super::WordWrap::new("hello\n       ", 5, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            vec!["hello", "     "]
        );
    }

    #[test]
    fn single_word() {
        let metrics = &FONT.metrics();

        let wrap = super::WordWrap::new("hello", 10, metrics, false);
        assert_eq!(wrap.map(|l| l.content).collect::<Vec<_>>(), vec!["hello"]);
    }

    #[test]
    fn multiple_words_fit() {
        let metrics = &FONT.metrics();
        let wrap = super::WordWrap::new("hello world", 11, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            vec!["hello world"]
        );
    }

    #[test]
    fn multiple_words_wrap() {
        let metrics = &FONT.metrics();
        let wrap = super::WordWrap::new("hello world", 10, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            vec!["hello", "world"]
        );
    }

    #[test]
    fn leading_whitespace_is_retained() {
        let metrics = &FONT.metrics();
        let wrap = super::WordWrap::new("  hello", 10, metrics, false);
        assert_eq!(wrap.map(|l| l.content).collect::<Vec<_>>(), vec!["  hello"]);
    }

    #[test]
    fn trailing_whitespace_is_dropped_even_when_it_fits() {
        let metrics = &FONT.metrics();
        let wrap = super::WordWrap::new("hello  ", 10, metrics, false);
        assert_eq!(wrap.map(|l| l.content).collect::<Vec<_>>(), vec!["hello"]);
    }

    #[test]
    fn trailing_whitespace_is_dropped_instead_of_wrapped() {
        let metrics = &FONT.metrics();
        let wrap = super::WordWrap::new("hello  ", 6, metrics, false);
        assert_eq!(wrap.map(|l| l.content).collect::<Vec<_>>(), vec!["hello"]);
    }

    #[test]
    fn multiple_whitespace_is_dropped_when_wrapped() {
        let metrics = &FONT.metrics();
        (5..=12).for_each(|available_width| {
            let wrap = super::WordWrap::new("hello   world", available_width, metrics, false);
            assert_eq!(
                wrap.map(|l| l.content).collect::<Vec<_>>(),
                vec!["hello", "world"]
            );
        });
    }

    #[test]
    fn partial_words_are_wrapped_1() {
        let metrics = &FONT.metrics();
        let wrap = super::WordWrap::new("hello world", 1, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            vec!["h", "e", "l", "l", "o", "w", "o", "r", "l", "d"]
        );
    }

    #[test]
    fn partial_words_are_wrapped_2() {
        let metrics = &FONT.metrics();
        let wrap = super::WordWrap::new("hello world", 2, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            // @typos-ignore
            vec!["he", "ll", "o", "wo", "rl", "d"]
        );
    }

    #[test]
    fn partial_words_are_wrapped_3() {
        let metrics = &FONT.metrics();
        let wrap = super::WordWrap::new("hello world", 3, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            // @typos-ignore
            vec!["hel", "lo", "wor", "ld"]
        );
    }

    #[test]
    fn newlines_are_always_wrapped() {
        let metrics = &FONT.metrics();
        let wrap = super::WordWrap::new("hello\nworld", 10, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            vec!["hello", "world"]
        );
    }

    #[test]
    fn multiple_consecutive_newlines_produce_empty_lines() {
        let metrics = &FONT.metrics();
        let wrap = super::WordWrap::new("hello\n\nworld", 10, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            vec!["hello", "", "world"]
        );
    }

    #[test]
    fn spaces_after_newlines_are_retained() {
        let metrics = &FONT.metrics();
        let wrap = super::WordWrap::new("hello \n world", 10, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            vec!["hello", " world"]
        );
    }

    #[test]
    fn newlines_on_wrap_boundary_do_not_produce_empty_lines() {
        let metrics = &FONT.metrics();
        let wrap = super::WordWrap::new("hello\nworld", 5, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            vec!["hello", "world"]
        );
    }

    #[test]
    fn newlines_wrap_after_forced_overflow() {
        let metrics = &FONT.metrics();
        let wrap = super::WordWrap::new("hello\nworld", 4, metrics, false);
        // @typos-ignore
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            // @typos-ignore
            vec!["hell", "o", "worl", "d"]
        );
    }

    #[test]
    fn unicode_wraps_correctly() {
        let metrics = &FONT.metrics();
        let wrap = super::WordWrap::new("mº🦀º🦀 🦀ºº ºº 🦀", 4, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            vec!["mº🦀º", "🦀", "🦀ºº", "ºº 🦀"]
        );
    }

    /// Characters are 1 unit, whitespace is 2 units, and digits are the width of the digit value
    struct VariableWidthFont;
    struct VariableWidthFontMetrics;

    impl FontMetrics for VariableWidthFontMetrics {
        fn rendered_size(&self, c: char) -> Option<Rectangle> {
            Some(Rectangle::new(Point::zero(), Size::new(self.advance(c), 1)))
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
            Size::new(9, 1)
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
    fn variable_width_wrapping() {
        let metrics = &VariableWidthFont.metrics();
        let wrap = super::WordWrap::new("1 2 3 4 5 6", 5, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            vec!["1 2", "3", "4", "5", "6"]
        );
    }

    #[test]
    fn compact_width_offer_never_wraps() {
        let metrics = &FONT.metrics();
        let wrap = super::WordWrap::new("hello world", ProposedDimension::Compact, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            vec!["hello world"]
        );
    }

    #[test]
    fn infinite_width_offer_never_wraps() {
        let metrics = &FONT.metrics();
        let wrap = super::WordWrap::new("hello world", ProposedDimension::Infinite, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            vec!["hello world"]
        );
    }

    #[test]
    fn compact_width_offer_only_wraps_explicit_newlines() {
        let metrics = &FONT.metrics();
        let wrap = super::WordWrap::new("hello\nworld", ProposedDimension::Compact, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            vec!["hello", "world"]
        );
    }

    #[test]
    fn infinite_width_offer_only_wraps_explicit_newlines() {
        let metrics = &FONT.metrics();
        let wrap =
            super::WordWrap::new("hello\nworld", ProposedDimension::Infinite, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            vec!["hello", "world"]
        );
    }

    #[test]
    fn width_is_calculated_correctly() {
        let metrics = &FONT.metrics();
        let wrap = super::WordWrap::new("hello world", 10, metrics, false);
        let lines: Vec<_> = wrap.collect();
        assert_eq!(lines[0].width, 5);
        assert_eq!(lines[1].width, 5);
    }

    #[test]
    fn width_is_calculated_with_variable_width() {
        let metrics = &VariableWidthFont.metrics();
        let wrap = super::WordWrap::new("1 2 3 4 5 6", 5, metrics, false);
        let lines: Vec<_> = wrap.collect();
        assert_eq!(lines[0].width, 5); // "1 2" = 1 + 2 + 2 = 5
        assert_eq!(lines[1].width, 3); // "3" = 3
        assert_eq!(lines[2].width, 4); // "4" = 4
        assert_eq!(lines[3].width, 5); // "5" = 5
        assert_eq!(lines[4].width, 6); // "6" = 6
    }

    #[test]
    fn precise_width_not_calculated_when_disabled() {
        let metrics = &FONT.metrics();
        let wrap = super::WordWrap::new("hello world", 10, metrics, false);
        let lines: Vec<_> = wrap.collect();
        assert_eq!(lines[0].precise_width, 0);
        assert_eq!(lines[1].precise_width, 0);
    }

    #[test]
    fn precise_width_calculated_when_enabled() {
        let metrics = &FONT.metrics();
        let wrap = super::WordWrap::new("hello world", 10, metrics, true);
        let lines: Vec<_> = wrap.collect();

        // First line "hello" should have precise width
        let expected = calculate_expected_precise_width("hello", metrics);
        assert_eq!(lines[0].precise_width, expected);

        // Second line "world" should have precise width
        let expected = calculate_expected_precise_width("world", metrics);
        assert_eq!(lines[1].precise_width, expected);
    }

    #[test]
    fn precise_width_handles_empty_lines() {
        let metrics = &FONT.metrics();
        let wrap = super::WordWrap::new("hello\n\nworld", 10, metrics, true);
        let lines: Vec<_> = wrap.collect();

        let expected = calculate_expected_precise_width("hello", metrics);
        assert_eq!(lines[0].precise_width, expected);

        assert_eq!(lines[1].precise_width, 0); // Empty line has 0 width

        let expected = calculate_expected_precise_width("world", metrics);
        assert_eq!(lines[2].precise_width, expected);
    }

    #[test]
    fn precise_width_with_variable_width_font() {
        let metrics = &VariableWidthFont.metrics();
        let wrap = super::WordWrap::new("1 2 3 4 5 6", 5, metrics, true);
        let lines: Vec<_> = wrap.collect();

        // Verify each line's precise width matches expected
        assert_eq!(
            lines[0].precise_width,
            calculate_expected_precise_width("1 2", metrics)
        );
        assert_eq!(
            lines[1].precise_width,
            calculate_expected_precise_width("3", metrics)
        );
        assert_eq!(
            lines[2].precise_width,
            calculate_expected_precise_width("4", metrics)
        );
        assert_eq!(
            lines[3].precise_width,
            calculate_expected_precise_width("5", metrics)
        );
        assert_eq!(
            lines[4].precise_width,
            calculate_expected_precise_width("6", metrics)
        );
    }

    #[test]
    fn precise_width_with_forced_line_break() {
        let metrics = &FONT.metrics();
        let wrap = super::WordWrap::new("hello\nworld", 4, metrics, true);
        let lines: Vec<_> = wrap.collect();

        assert_eq!(
            lines[0].precise_width,
            calculate_expected_precise_width("hell", metrics)
        );
        assert_eq!(
            lines[1].precise_width,
            calculate_expected_precise_width("o", metrics)
        );
        assert_eq!(
            lines[2].precise_width,
            // @typos-ignore
            calculate_expected_precise_width("worl", metrics)
        );
        assert_eq!(
            lines[3].precise_width,
            calculate_expected_precise_width("d", metrics)
        );
    }

    #[test]
    fn first_and_last_line_tracked() {
        let metrics = &FONT.metrics();
        let mut wrap = super::WordWrap::new("hello", 10, metrics, true);

        let lines: Vec<_> = (&mut wrap).collect();
        assert_eq!(lines.len(), 1);

        assert_eq!(*wrap.first_non_empty_line(), Some(("hello", 0)));
        assert_eq!(*wrap.last_non_empty_line(), Some(("hello", 0)));
    }

    #[test]
    fn trailing_newlines_ignored() {
        let metrics = &FONT.metrics();
        let mut wrap = super::WordWrap::new("hello\n\nworld\n\n", 10, metrics, true);

        let _lines: Vec<_> = (&mut wrap).collect();

        // Empty line at the end should be skipped
        assert_eq!(*wrap.first_non_empty_line(), Some(("hello", 0)));
        assert_eq!(*wrap.last_non_empty_line(), Some(("world", 2)));
    }

    struct FontTrace<F> {
        rendered_size_calls: RefCell<u32>,
        advance_calls: RefCell<u32>,
        inner: F,
    }

    struct TraceMetrics<'a, F> {
        rendered_size_calls: &'a RefCell<u32>,
        advance_calls: &'a RefCell<u32>,
        inner: F,
    }

    impl<F: FontMetrics> FontMetrics for TraceMetrics<'_, F> {
        fn rendered_size(&self, c: char) -> Option<Rectangle> {
            *self.rendered_size_calls.borrow_mut() += 1;
            self.inner.rendered_size(c)
        }

        fn default_line_height(&self) -> u32 {
            self.inner.default_line_height()
        }

        fn advance(&self, character: char) -> u32 {
            *self.advance_calls.borrow_mut() += 1;
            self.inner.advance(character)
        }

        fn maximum_character_size(&self) -> Size {
            self.inner.maximum_character_size()
        }
    }

    impl<F: Font> Font for FontTrace<F> {
        fn metrics(&self) -> impl crate::font::FontMetrics {
            TraceMetrics {
                rendered_size_calls: &self.rendered_size_calls,
                advance_calls: &self.advance_calls,
                inner: self.inner.metrics(),
            }
        }
    }

    impl<F> crate::font::Sealed for FontTrace<F> {}

    impl<C, F: FontRender<C>> FontRender<C> for FontTrace<F> {
        fn draw(&self, _: char, _: C, _: &mut impl Surface<Color = C>) {}
    }

    #[test]
    fn metric_calls_in_iter() {
        let traced_font = FontTrace {
            rendered_size_calls: RefCell::new(0),
            advance_calls: RefCell::new(0),
            inner: FONT,
        };

        let traced_font_metrics = traced_font.metrics();

        let wrap = super::WordWrap::new("aaaaa\nbbbbb\nccccc", 5, &traced_font_metrics, true);

        let _: Vec<_> = wrap.collect();
        let rendered_size_calls = *traced_font.rendered_size_calls.borrow();
        let advance_calls = *traced_font.advance_calls.borrow();
        // The iterator only checks the first/last characters in each line
        assert_eq!(rendered_size_calls, 6);
        // This value is arbitrary, will be > total chars (15)
        // Evaluate failures for changes in performance
        assert_eq!(advance_calls, 38);
    }

    #[test]
    fn non_precise_has_no_rendered_dimensions_calls_in_iter() {
        let traced_font = FontTrace {
            rendered_size_calls: RefCell::new(0),
            advance_calls: RefCell::new(0),
            inner: FONT,
        };

        let traced_font_metrics = traced_font.metrics();

        let wrap = super::WordWrap::new("aaaaa\nbbbbb\nccccc", 5, &traced_font_metrics, false);

        let _: Vec<_> = wrap.collect();
        let rendered_size_calls = *traced_font.rendered_size_calls.borrow();
        let advance_calls = *traced_font.advance_calls.borrow();
        assert_eq!(rendered_size_calls, 0);
        // This value is arbitrary, will be > total chars (15)
        // Evaluate failures for changes in performance
        assert_eq!(advance_calls, 35);
    }

    #[test]
    fn metric_calls_in_precise_text_layout() {
        let traced_font = FontTrace {
            rendered_size_calls: RefCell::new(0),
            advance_calls: RefCell::new(0),
            inner: FONT,
        };

        let text = Text {
            text: "aaaaa\nbbbbb\nccccc\nddddd",
            font: &traced_font,
            alignment: crate::view::HorizontalTextAlignment::Leading,
            precise_character_bounds: true,
            wrap: crate::view::WrapStrategy::Word,
        };
        text.layout(
            &ProposedDimensions::new(5, 5),
            &DefaultEnvironment::default(),
            &mut (),
            &mut (),
        );
        let rendered_size_calls = *traced_font.rendered_size_calls.borrow();
        let advance_calls = *traced_font.advance_calls.borrow();
        // Theoretical minimum is 2w + 2h - 4,
        // current implementation is 2w + 2h
        assert_eq!(rendered_size_calls, 18);
        // This value is arbitrary, will be > total chars
        // Evaluate failures for changes in performance
        assert_eq!(advance_calls, 54);
    }

    #[test]
    fn metric_calls_in_non_precise_text_layout() {
        let traced_font = FontTrace {
            rendered_size_calls: RefCell::new(0),
            advance_calls: RefCell::new(0),
            inner: FONT,
        };

        let text = Text {
            text: "aaaaa\nbbbbb\nccccc\nddddd",
            font: &traced_font,
            alignment: crate::view::HorizontalTextAlignment::Leading,
            precise_character_bounds: false,
            wrap: crate::view::WrapStrategy::Word,
        };
        text.layout(
            &ProposedDimensions::new(5, 5),
            &DefaultEnvironment::default(),
            &mut (),
            &mut (),
        );
        let rendered_size_calls = *traced_font.rendered_size_calls.borrow();
        let advance_calls = *traced_font.advance_calls.borrow();
        assert_eq!(rendered_size_calls, 0);
        // This value is arbitrary, will be > total chars
        // Evaluate failures for changes in performance
        assert_eq!(advance_calls, 50);
    }
}
