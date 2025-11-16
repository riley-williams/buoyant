use crate::{
    font::FontMetrics,
    primitives::{Point, ProposedDimension, geometry::Rectangle},
    view::text::WrappedLine,
};

#[derive(Debug, Clone)]
pub struct WhitespaceWrap<'a, F> {
    remaining: &'a str,
    overflow: &'a str,
    available_width: ProposedDimension,
    font: &'a F,
    calculate_precise_bounds: bool,
    current_y: i32,
}

impl<'a, F: FontMetrics> WhitespaceWrap<'a, F> {
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
        }
    }

    // Helper function to calculate width and optionally precise bounds for a line
    fn calculate_line_metrics(&self, text: &str) -> (u32, Option<Rectangle>) {
        let mut width = 0;
        let mut precise_bounds: Option<Rectangle> = None;

        if self.calculate_precise_bounds {
            for ch in text.chars() {
                if let Some(mut char_bounds) = self.font.rendered_size(ch) {
                    char_bounds.origin += Point::new(width as i32, self.current_y);
                    precise_bounds = precise_bounds.map_or(Some(char_bounds.clone()), |rect| {
                        Some(rect.union(&char_bounds))
                    });
                }
                width += self.font.advance(ch);
            }
        } else {
            // Just calculate width
            for ch in text.chars() {
                width += self.font.advance(ch);
            }
        }

        (width, precise_bounds)
    }

    // Helper function to find force split position, returns (split_pos, width_up_to_split)
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

impl<'a, F: FontMetrics + 'a> Iterator for WhitespaceWrap<'a, F> {
    type Item = WrappedLine<'a>;

    #[allow(clippy::too_many_lines)]
    fn next(&mut self) -> Option<Self::Item> {
        // Handle overflow first
        if !self.overflow.is_empty() {
            // Check if overflow needs to be split further
            if let Some((split_pos, _)) = self.find_split_pos(self.overflow) {
                let (result, rest) = self.overflow.split_at(split_pos);
                self.overflow = rest;
                let (width, precise_bounds) = self.calculate_line_metrics(result);
                self.current_y += self.font.default_line_height() as i32;
                return Some(WrappedLine {
                    content: result,
                    width,
                    precise_bounds,
                });
            }
            let result = self.overflow;
            self.overflow = "";
            let (width, precise_bounds) = self.calculate_line_metrics(result);
            self.current_y += self.font.default_line_height() as i32;
            return Some(WrappedLine {
                content: result,
                width,
                precise_bounds,
            });
        }

        // Return None if no more text
        if self.remaining.is_empty() {
            return None;
        }

        let mut width = 0;
        let mut last_space = None;

        // Single pass through the string to find split points
        for (pos, ch) in self.remaining.char_indices() {
            // Check for newline first
            if ch == '\n' {
                let (line, rest) = self.remaining.split_at(pos);
                // This is safe because ch == \n which is 1 byte
                self.remaining = &rest[1..];

                // Handle empty lines and spaces after newlines
                if line.is_empty() {
                    self.current_y += self.font.default_line_height() as i32;
                    return Some(WrappedLine {
                        content: line,
                        width: 0,
                        precise_bounds: None,
                    });
                }

                // Check if the line before newline needs force-splitting
                if let Some((split_pos, width_before_split)) = self.find_split_pos(line) {
                    let (result, rest) = line.split_at(split_pos);
                    self.overflow = rest;
                    let (_, precise_bounds) = if self.calculate_precise_bounds {
                        self.calculate_line_metrics(result)
                    } else {
                        (width_before_split, None)
                    };
                    self.current_y += self.font.default_line_height() as i32;
                    return Some(WrappedLine {
                        content: result,
                        width: width_before_split,
                        precise_bounds,
                    });
                }

                let line = line.trim_end();
                let (width, precise_bounds) = self.calculate_line_metrics(line);
                self.current_y += self.font.default_line_height() as i32;
                return Some(WrappedLine {
                    content: line,
                    width,
                    precise_bounds,
                });
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
                    let (width, precise_bounds) = self.calculate_line_metrics(result);
                    self.current_y += self.font.default_line_height() as i32;
                    return Some(WrappedLine {
                        content: result,
                        width,
                        precise_bounds,
                    });
                }
                // Force split the word
                let split_pos = if pos > 0 {
                    pos
                } else {
                    let Some(p) = self.remaining.char_indices().nth(1) else {
                        let last_char = self.remaining;
                        self.remaining = "";
                        let (width, precise_bounds) = self.calculate_line_metrics(last_char);
                        self.current_y += self.font.default_line_height() as i32;
                        return Some(WrappedLine {
                            content: last_char,
                            width,
                            precise_bounds,
                        });
                    };
                    p.0
                };
                let (result, rest) = self.remaining.split_at(split_pos);
                self.remaining = rest;
                // width - char_width is the width up to (but not including) the current char
                let result_width = width - char_width;
                let (_, precise_bounds) = if self.calculate_precise_bounds {
                    self.calculate_line_metrics(result)
                } else {
                    (result_width, None)
                };
                self.current_y += self.font.default_line_height() as i32;
                return Some(WrappedLine {
                    content: result,
                    width: result_width,
                    precise_bounds,
                });
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
            let (_, precise_bounds) = if self.calculate_precise_bounds {
                self.calculate_line_metrics(result)
            } else {
                (width, None)
            };
            self.current_y += self.font.default_line_height() as i32;
            return Some(WrappedLine {
                content: result,
                width,
                precise_bounds,
            });
        }

        // No wrap needed - return all remaining text
        let result = self.remaining.trim_end();
        self.remaining = "";
        let (width, precise_bounds) = self.calculate_line_metrics(result);
        self.current_y += self.font.default_line_height() as i32;
        Some(WrappedLine {
            content: result,
            width,
            precise_bounds,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::font::{Font, FontMetrics, FontRender};
    use crate::primitives::geometry::Rectangle;
    use crate::primitives::{Point, Size};
    use crate::surface::Surface;
    use crate::{font::CharacterBufferFont, primitives::ProposedDimension};
    use std::vec;
    use std::vec::Vec;
    // a basic font for which all characters are 1 unit wide
    static FONT: CharacterBufferFont = CharacterBufferFont;

    /// Helper function to calculate expected precise bounds for a line of text.
    /// This unions all character `rendered_size` rectangles, accounting for advance.
    /// Returns None if no characters have rendered bounds.
    fn calculate_expected_bounds(
        text: &str,
        metrics: &impl FontMetrics,
        y_offset: i32,
    ) -> Option<Rectangle> {
        let mut result: Option<Rectangle> = None;
        let mut advance = 0;

        for ch in text.chars() {
            if let Some(mut char_bounds) = metrics.rendered_size(ch) {
                char_bounds.origin += Point::new(advance as i32, y_offset);
                result = result.map_or(Some(char_bounds.clone()), |rect| {
                    Some(rect.union(&char_bounds))
                });
            }
            advance += metrics.advance(ch);
        }

        result
    }

    #[test]
    fn empty_text() {
        let metrics = &FONT.metrics();
        let wrap = super::WhitespaceWrap::new("", 10, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<&str>>(),
            Vec::<&str>::new()
        );
    }

    #[ignore = "Not sure how much I care about this behavior"]
    #[test]
    fn only_whitespace_lines_are_retained_up_to_wrapping_width() {
        let metrics = &FONT.metrics();
        let wrap = super::WhitespaceWrap::new(" ", 5, metrics, false);
        assert_eq!(wrap.map(|l| l.content).collect::<Vec<_>>(), vec![" "]);
        let wrap = super::WhitespaceWrap::new("    ", 5, metrics, false);
        assert_eq!(wrap.map(|l| l.content).collect::<Vec<_>>(), vec!["    "]);
        let wrap = super::WhitespaceWrap::new("     ", 5, metrics, false);
        assert_eq!(wrap.map(|l| l.content).collect::<Vec<_>>(), vec!["     "]);
        let wrap = super::WhitespaceWrap::new("      ", 5, metrics, false);
        assert_eq!(wrap.map(|l| l.content).collect::<Vec<_>>(), vec!["     "]);
        let wrap = super::WhitespaceWrap::new("       ", 5, metrics, false);
        assert_eq!(wrap.map(|l| l.content).collect::<Vec<_>>(), vec!["     "]);
    }

    #[ignore = "Not sure how much I care about this behavior"]
    #[test]
    fn only_whitespace_lines_are_retained_up_to_wrapping_width_after_newline() {
        let metrics = &FONT.metrics();
        let wrap = super::WhitespaceWrap::new("hello\n ", 5, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            vec!["hello", " "]
        );
        let wrap = super::WhitespaceWrap::new("hello\n    ", 5, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            vec!["hello", "    "]
        );
        let wrap = super::WhitespaceWrap::new("hello\n     ", 5, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            vec!["hello", "     "]
        );
        let wrap = super::WhitespaceWrap::new("hello\n      ", 5, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            vec!["hello", "     "]
        );
        let wrap = super::WhitespaceWrap::new("hello\n       ", 5, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            vec!["hello", "     "]
        );
    }

    #[test]
    fn single_word() {
        let metrics = &FONT.metrics();

        let wrap = super::WhitespaceWrap::new("hello", 10, metrics, false);
        assert_eq!(wrap.map(|l| l.content).collect::<Vec<_>>(), vec!["hello"]);
    }

    #[test]
    fn multiple_words_fit() {
        let metrics = &FONT.metrics();
        let wrap = super::WhitespaceWrap::new("hello world", 11, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            vec!["hello world"]
        );
    }

    #[test]
    fn multiple_words_wrap() {
        let metrics = &FONT.metrics();
        let wrap = super::WhitespaceWrap::new("hello world", 10, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            vec!["hello", "world"]
        );
    }

    #[test]
    fn leading_whitespace_is_retained() {
        let metrics = &FONT.metrics();
        let wrap = super::WhitespaceWrap::new("  hello", 10, metrics, false);
        assert_eq!(wrap.map(|l| l.content).collect::<Vec<_>>(), vec!["  hello"]);
    }

    #[test]
    fn trailing_whitespace_is_dropped_even_when_it_fits() {
        let metrics = &FONT.metrics();
        let wrap = super::WhitespaceWrap::new("hello  ", 10, metrics, false);
        assert_eq!(wrap.map(|l| l.content).collect::<Vec<_>>(), vec!["hello"]);
    }

    #[test]
    fn trailing_whitespace_is_dropped_instead_of_wrapped() {
        let metrics = &FONT.metrics();
        let wrap = super::WhitespaceWrap::new("hello  ", 6, metrics, false);
        assert_eq!(wrap.map(|l| l.content).collect::<Vec<_>>(), vec!["hello"]);
    }

    #[test]
    fn multiple_whitespace_is_dropped_when_wrapped() {
        let metrics = &FONT.metrics();
        (5..=12).for_each(|available_width| {
            let wrap = super::WhitespaceWrap::new("hello   world", available_width, metrics, false);
            assert_eq!(
                wrap.map(|l| l.content).collect::<Vec<_>>(),
                vec!["hello", "world"]
            );
        });
    }

    #[test]
    fn partial_words_are_wrapped_1() {
        let metrics = &FONT.metrics();
        let wrap = super::WhitespaceWrap::new("hello world", 1, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            vec!["h", "e", "l", "l", "o", "w", "o", "r", "l", "d"]
        );
    }

    #[test]
    fn partial_words_are_wrapped_2() {
        let metrics = &FONT.metrics();
        let wrap = super::WhitespaceWrap::new("hello world", 2, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            vec!["he", "ll", "o", "wo", "rl", "d"]
        );
    }

    #[test]
    fn partial_words_are_wrapped_3() {
        let metrics = &FONT.metrics();
        let wrap = super::WhitespaceWrap::new("hello world", 3, metrics, false);
        // @typos-ignore
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            vec!["hel", "lo", "wor", "ld"]
        );
    }

    #[test]
    fn newlines_are_always_wrapped() {
        let metrics = &FONT.metrics();
        let wrap = super::WhitespaceWrap::new("hello\nworld", 10, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            vec!["hello", "world"]
        );
    }

    #[test]
    fn multiple_consecutive_newlines_produce_empty_lines() {
        let metrics = &FONT.metrics();
        let wrap = super::WhitespaceWrap::new("hello\n\nworld", 10, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            vec!["hello", "", "world"]
        );
    }

    #[test]
    fn spaces_after_newlines_are_retained() {
        let metrics = &FONT.metrics();
        let wrap = super::WhitespaceWrap::new("hello \n world", 10, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            vec!["hello", " world"]
        );
    }

    #[test]
    fn newlines_on_wrap_boundary_do_not_produce_empty_lines() {
        let metrics = &FONT.metrics();
        let wrap = super::WhitespaceWrap::new("hello\nworld", 5, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            vec!["hello", "world"]
        );
    }

    #[test]
    fn newlines_wrap_after_forced_overflow() {
        let metrics = &FONT.metrics();
        let wrap = super::WhitespaceWrap::new("hello\nworld", 4, metrics, false);
        // @typos-ignore
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            vec!["hell", "o", "worl", "d"]
        );
    }

    #[test]
    fn unicode_wraps_correctly() {
        let metrics = &FONT.metrics();
        let wrap = super::WhitespaceWrap::new("mº🦀º🦀 🦀ºº ºº 🦀", 4, metrics, false);
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
        let wrap = super::WhitespaceWrap::new("1 2 3 4 5 6", 5, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            vec!["1 2", "3", "4", "5", "6"]
        );
    }

    #[test]
    fn compact_width_offer_never_wraps() {
        let metrics = &FONT.metrics();
        let wrap =
            super::WhitespaceWrap::new("hello world", ProposedDimension::Compact, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            vec!["hello world"]
        );
    }

    #[test]
    fn infinite_width_offer_never_wraps() {
        let metrics = &FONT.metrics();
        let wrap =
            super::WhitespaceWrap::new("hello world", ProposedDimension::Infinite, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            vec!["hello world"]
        );
    }

    #[test]
    fn compact_width_offer_only_wraps_explicit_newlines() {
        let metrics = &FONT.metrics();
        let wrap =
            super::WhitespaceWrap::new("hello\nworld", ProposedDimension::Compact, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            vec!["hello", "world"]
        );
    }

    #[test]
    fn infinite_width_offer_only_wraps_explicit_newlines() {
        let metrics = &FONT.metrics();
        let wrap =
            super::WhitespaceWrap::new("hello\nworld", ProposedDimension::Infinite, metrics, false);
        assert_eq!(
            wrap.map(|l| l.content).collect::<Vec<_>>(),
            vec!["hello", "world"]
        );
    }

    #[test]
    fn width_is_calculated_correctly() {
        let metrics = &FONT.metrics();
        let wrap = super::WhitespaceWrap::new("hello world", 10, metrics, false);
        let lines: Vec<_> = wrap.collect();
        assert_eq!(lines[0].width, 5);
        assert_eq!(lines[1].width, 5);
    }

    #[test]
    fn width_is_calculated_with_variable_width() {
        let metrics = &VariableWidthFont.metrics();
        let wrap = super::WhitespaceWrap::new("1 2 3 4 5 6", 5, metrics, false);
        let lines: Vec<_> = wrap.collect();
        assert_eq!(lines[0].width, 5); // "1 2" = 1 + 2 + 2 = 5
        assert_eq!(lines[1].width, 3); // "3" = 3
        assert_eq!(lines[2].width, 4); // "4" = 4
        assert_eq!(lines[3].width, 5); // "5" = 5
        assert_eq!(lines[4].width, 6); // "6" = 6
    }

    #[test]
    fn precise_bounds_are_not_calculated_when_disabled() {
        let metrics = &FONT.metrics();
        let wrap = super::WhitespaceWrap::new("hello world", 10, metrics, false);
        let lines: Vec<_> = wrap.collect();
        assert!(lines[0].precise_bounds.is_none());
        assert!(lines[1].precise_bounds.is_none());
    }

    #[test]
    fn precise_bounds_are_calculated_when_enabled() {
        let metrics = &FONT.metrics();
        let wrap = super::WhitespaceWrap::new("hello world", 10, metrics, true);
        let lines: Vec<_> = wrap.collect();

        // First line "hello" should have bounds matching expected calculation
        assert!(lines[0].precise_bounds.is_some());
        let expected = calculate_expected_bounds("hello", metrics, 0);
        assert_eq!(lines[0].precise_bounds, expected);

        // Second line "world" should have bounds matching expected calculation
        assert!(lines[1].precise_bounds.is_some());
        let expected = calculate_expected_bounds("world", metrics, 1);
        assert_eq!(lines[1].precise_bounds, expected);
    }

    #[test]
    fn precise_bounds_handle_empty_lines() {
        let metrics = &FONT.metrics();
        let wrap = super::WhitespaceWrap::new("hello\n\nworld", 10, metrics, true);
        let lines: Vec<_> = wrap.collect();

        let expected = calculate_expected_bounds("hello", metrics, 0);
        assert_eq!(lines[0].precise_bounds, expected);

        assert_eq!(lines[1].precise_bounds, None); // Empty line has no bounds

        let expected = calculate_expected_bounds("world", metrics, 2);
        assert_eq!(lines[2].precise_bounds, expected);
    }

    #[test]
    fn precise_bounds_with_variable_width_font() {
        let metrics = &VariableWidthFont.metrics();
        let wrap = super::WhitespaceWrap::new("1 2 3 4 5 6", 5, metrics, true);
        let lines: Vec<_> = wrap.collect();

        // Verify each line's precise bounds matches expected
        assert_eq!(
            lines[0].precise_bounds,
            calculate_expected_bounds("1 2", metrics, 0)
        );
        assert_eq!(
            lines[1].precise_bounds,
            calculate_expected_bounds("3", metrics, 1)
        );
        assert_eq!(
            lines[2].precise_bounds,
            calculate_expected_bounds("4", metrics, 2)
        );
        assert_eq!(
            lines[3].precise_bounds,
            calculate_expected_bounds("5", metrics, 3)
        );
        assert_eq!(
            lines[4].precise_bounds,
            calculate_expected_bounds("6", metrics, 4)
        );
    }

    #[test]
    fn precise_bounds_with_forced_line_break() {
        let metrics = &FONT.metrics();
        let wrap = super::WhitespaceWrap::new("hello\nworld", 4, metrics, true);
        let lines: Vec<_> = wrap.collect();

        // "hell"
        assert_eq!(
            lines[0].precise_bounds,
            calculate_expected_bounds("hell", metrics, 0)
        );
        // "o"
        assert_eq!(
            lines[1].precise_bounds,
            calculate_expected_bounds("o", metrics, 1)
        );
        // "worl"
        assert_eq!(
            lines[2].precise_bounds,
            calculate_expected_bounds("worl", metrics, 2)
        );
        // "d"
        assert_eq!(
            lines[3].precise_bounds,
            calculate_expected_bounds("d", metrics, 3)
        );
    }

    #[test]
    fn precise_bounds_multiline_height_is_correct() {
        let metrics = &FONT.metrics();
        // Two lines of text
        let wrap = super::WhitespaceWrap::new("hello world", 10, metrics, true);
        let lines: Vec<_> = wrap.collect();

        // Union the bounds manually to verify total height
        let first = lines[0].precise_bounds.clone().unwrap();
        let second = lines[1].precise_bounds.clone().unwrap();
        let combined = first.union(&second);

        // Total height should be exactly 2 * line_height (1 unit per line for this font)
        // There should be no extra line worth of space
        assert_eq!(combined.size.height, 2);
        assert_eq!(combined.origin.y, 0);
        // Bottom of bounds should be at y=2, not y=3
        assert_eq!(combined.origin.y + combined.size.height as i32, 2);
    }
}
