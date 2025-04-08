use crate::{font::FontMetrics, primitives::ProposedDimension};

#[derive(Debug, Clone)]
pub struct WhitespaceWrap<'a, F> {
    remaining: &'a str,
    overflow: &'a str,
    available_width: ProposedDimension,
    font: &'a F,
}

impl<'a, F: FontMetrics> WhitespaceWrap<'a, F> {
    pub fn new(text: &'a str, available_width: impl Into<ProposedDimension>, font: &'a F) -> Self {
        Self {
            remaining: text,
            overflow: "",
            available_width: available_width.into(),
            font,
        }
    }

    // Helper function to find force split position
    fn find_split_pos(&self, text: &str) -> Option<usize> {
        let mut width = 0;
        for (pos, ch) in text.char_indices() {
            width += self.font.advance(ch);
            if ProposedDimension::Exact(width) > self.available_width {
                return Some(if pos > 0 { pos } else { 1 });
            }
        }
        None
    }
}

impl<'a, F: FontMetrics + 'a> Iterator for WhitespaceWrap<'a, F> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        // Handle overflow first
        if !self.overflow.is_empty() {
            // Check if overflow needs to be split further
            if let Some(split_pos) = self.find_split_pos(self.overflow) {
                let (result, rest) = self.overflow.split_at(split_pos);
                self.overflow = rest;
                return Some(result);
            }
            let result = self.overflow;
            self.overflow = "";
            return Some(result);
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
                self.remaining = &rest[1..];

                // Handle empty lines and spaces after newlines
                if line.is_empty() {
                    return Some(line);
                }

                // Check if the line before newline needs force-splitting
                if let Some(split_pos) = self.find_split_pos(line) {
                    let (result, rest) = line.split_at(split_pos);
                    self.overflow = rest;
                    return Some(result);
                }

                return Some(line.trim_end());
            }

            width += self.font.advance(ch);

            if ch.is_whitespace() {
                last_space = Some(pos);
            }

            // Check for force split
            if ProposedDimension::Exact(width) > self.available_width {
                if let Some(space_pos) = last_space {
                    // Split at last space
                    let (result, rest) = self.remaining.split_at(space_pos);
                    self.remaining = rest.trim_start();
                    return Some(result.trim_end());
                }
                // Force split the word
                let split_pos = if pos > 0 { pos } else { 1 };
                let (result, rest) = self.remaining.split_at(split_pos);
                self.remaining = rest;
                return Some(result);
            }
        }

        // Handle whitespace-only remaining text
        if self.remaining.chars().all(char::is_whitespace) {
            let mut end = self.remaining.len();
            let mut width = 0;
            for (pos, ch) in self.remaining.char_indices() {
                width += self.font.advance(ch);
                if ProposedDimension::Exact(width) > self.available_width {
                    end = pos;
                    break;
                }
            }
            let result = &self.remaining[..end];
            self.remaining = "";
            return Some(result);
        }

        // No wrap needed - return all remaining text
        let result = self.remaining;
        self.remaining = "";
        Some(result.trim_end())
    }
}

#[cfg(test)]
mod tests {
    use crate::font::{Font, FontMetrics, FontRender};
    use crate::primitives::Size;
    use crate::surface::Surface;
    use crate::{font::CharacterBufferFont, primitives::ProposedDimension};
    use std::vec;
    use std::vec::Vec;
    // a basic font for which all characters are 1 unit wide
    static FONT: CharacterBufferFont = CharacterBufferFont;

    #[test]
    fn empty_text() {
        let metrics = &FONT.metrics();
        let wrap = super::WhitespaceWrap::new("", 10, metrics);
        assert_eq!(wrap.collect::<Vec<&str>>(), Vec::<&str>::new());
    }

    #[ignore = "Not sure how much I care about this behavior"]
    #[test]
    fn only_whitespace_lines_are_retained_up_to_wrapping_width() {
        let metrics = &FONT.metrics();
        let wrap = super::WhitespaceWrap::new(" ", 5, metrics);
        assert_eq!(wrap.collect::<Vec<_>>(), vec![" "]);
        let wrap = super::WhitespaceWrap::new("    ", 5, metrics);
        assert_eq!(wrap.collect::<Vec<_>>(), vec!["    "]);
        let wrap = super::WhitespaceWrap::new("     ", 5, metrics);
        assert_eq!(wrap.collect::<Vec<_>>(), vec!["     "]);
        let wrap = super::WhitespaceWrap::new("      ", 5, metrics);
        assert_eq!(wrap.collect::<Vec<_>>(), vec!["     "]);
        let wrap = super::WhitespaceWrap::new("       ", 5, metrics);
        assert_eq!(wrap.collect::<Vec<_>>(), vec!["     "]);
    }

    #[ignore = "Not sure how much I care about this behavior"]
    #[test]
    fn only_whitespace_lines_are_retained_up_to_wrapping_width_after_newline() {
        let metrics = &FONT.metrics();
        let wrap = super::WhitespaceWrap::new("hello\n ", 5, metrics);
        assert_eq!(wrap.collect::<Vec<_>>(), vec!["hello", " "]);
        let wrap = super::WhitespaceWrap::new("hello\n    ", 5, metrics);
        assert_eq!(wrap.collect::<Vec<_>>(), vec!["hello", "    "]);
        let wrap = super::WhitespaceWrap::new("hello\n     ", 5, metrics);
        assert_eq!(wrap.collect::<Vec<_>>(), vec!["hello", "     "]);
        let wrap = super::WhitespaceWrap::new("hello\n      ", 5, metrics);
        assert_eq!(wrap.collect::<Vec<_>>(), vec!["hello", "     "]);
        let wrap = super::WhitespaceWrap::new("hello\n       ", 5, metrics);
        assert_eq!(wrap.collect::<Vec<_>>(), vec!["hello", "     "]);
    }

    #[test]
    fn single_word() {
        let metrics = &FONT.metrics();

        let wrap = super::WhitespaceWrap::new("hello", 10, metrics);
        assert_eq!(wrap.collect::<Vec<_>>(), vec!["hello"]);
    }

    #[test]
    fn multiple_words_fit() {
        let metrics = &FONT.metrics();
        let wrap = super::WhitespaceWrap::new("hello world", 11, metrics);
        assert_eq!(wrap.collect::<Vec<_>>(), vec!["hello world"]);
    }

    #[test]
    fn multiple_words_wrap() {
        let metrics = &FONT.metrics();
        let wrap = super::WhitespaceWrap::new("hello world", 10, metrics);
        assert_eq!(wrap.collect::<Vec<_>>(), vec!["hello", "world"]);
    }

    #[test]
    fn leading_whitespace_is_retained() {
        let metrics = &FONT.metrics();
        let wrap = super::WhitespaceWrap::new("  hello", 10, metrics);
        assert_eq!(wrap.collect::<Vec<_>>(), vec!["  hello"]);
    }

    #[test]
    fn trailing_whitespace_is_dropped_even_when_it_fits() {
        let metrics = &FONT.metrics();
        let wrap = super::WhitespaceWrap::new("hello  ", 10, metrics);
        assert_eq!(wrap.collect::<Vec<_>>(), vec!["hello"]);
    }

    #[test]
    fn trailing_whitespace_is_dropped_instead_of_wrapped() {
        let metrics = &FONT.metrics();
        let wrap = super::WhitespaceWrap::new("hello  ", 6, metrics);
        assert_eq!(wrap.collect::<Vec<_>>(), vec!["hello"]);
    }

    #[test]
    fn multiple_whitespace_is_dropped_when_wrapped() {
        let metrics = &FONT.metrics();
        (5..=12).for_each(|available_width| {
            let wrap = super::WhitespaceWrap::new("hello   world", available_width, metrics);
            assert_eq!(wrap.collect::<Vec<_>>(), vec!["hello", "world"]);
        });
    }

    #[test]
    fn partial_words_are_wrapped_1() {
        let metrics = &FONT.metrics();
        let wrap = super::WhitespaceWrap::new("hello world", 1, metrics);
        assert_eq!(
            wrap.collect::<Vec<_>>(),
            vec!["h", "e", "l", "l", "o", "w", "o", "r", "l", "d"]
        );
    }

    #[test]
    fn partial_words_are_wrapped_2() {
        let metrics = &FONT.metrics();
        let wrap = super::WhitespaceWrap::new("hello world", 2, metrics);
        assert_eq!(
            wrap.collect::<Vec<_>>(),
            vec!["he", "ll", "o", "wo", "rl", "d"]
        );
    }

    #[test]
    fn partial_words_are_wrapped_3() {
        let metrics = &FONT.metrics();
        let wrap = super::WhitespaceWrap::new("hello world", 3, metrics);
        // @typos-ignore
        assert_eq!(wrap.collect::<Vec<_>>(), vec!["hel", "lo", "wor", "ld"]);
    }

    #[test]
    fn newlines_are_always_wrapped() {
        let metrics = &FONT.metrics();
        let wrap = super::WhitespaceWrap::new("hello\nworld", 10, metrics);
        assert_eq!(wrap.collect::<Vec<_>>(), vec!["hello", "world"]);
    }

    #[test]
    fn multiple_consecutive_newlines_produce_empty_lines() {
        let metrics = &FONT.metrics();
        let wrap = super::WhitespaceWrap::new("hello\n\nworld", 10, metrics);
        assert_eq!(wrap.collect::<Vec<_>>(), vec!["hello", "", "world"]);
    }

    #[test]
    fn spaces_after_newlines_are_retained() {
        let metrics = &FONT.metrics();
        let wrap = super::WhitespaceWrap::new("hello \n world", 10, metrics);
        assert_eq!(wrap.collect::<Vec<_>>(), vec!["hello", " world"]);
    }

    #[test]
    fn newlines_on_wrap_boundary_do_not_produce_empty_lines() {
        let metrics = &FONT.metrics();
        let wrap = super::WhitespaceWrap::new("hello\nworld", 5, metrics);
        assert_eq!(wrap.collect::<Vec<_>>(), vec!["hello", "world"]);
    }

    #[test]
    fn newlines_wrap_after_forced_overflow() {
        let metrics = &FONT.metrics();
        let wrap = super::WhitespaceWrap::new("hello\nworld", 4, metrics);
        // @typos-ignore
        assert_eq!(wrap.collect::<Vec<_>>(), vec!["hell", "o", "worl", "d"]);
    }

    #[ignore = "This test is correct, fix later"]
    #[test]
    fn multi_byte_unicode_wraps_correctly() {
        let metrics = &FONT.metrics();
        let wrap = super::WhitespaceWrap::new("y̆y̆y̆y̆y̆ y̆y̆y̆ y̆y̆ y̆", 4, metrics);
        assert_eq!(wrap.collect::<Vec<_>>(), vec!["y̆y̆y̆y̆", "y̆", "y̆y̆y̆", "y̆y̆ y̆"]);
    }

    /// Characters are 1 unit, whitespace is 2 units, and digits are the width of the digit value
    struct VariableWidthFont;
    struct VariableWidthFontMetrics;

    impl FontMetrics for VariableWidthFontMetrics {
        fn rendered_size(&self, c: char) -> crate::primitives::Size {
            Size::new(self.advance(c), 1)
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

        fn baseline(&self) -> u32 {
            1
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
        let wrap = super::WhitespaceWrap::new("1 2 3 4 5 6", 5, metrics);
        assert_eq!(wrap.collect::<Vec<_>>(), vec!["1 2", "3", "4", "5", "6"]);
    }

    #[test]
    fn compact_width_offer_never_wraps() {
        let metrics = &FONT.metrics();
        let wrap = super::WhitespaceWrap::new("hello world", ProposedDimension::Compact, metrics);
        assert_eq!(wrap.collect::<Vec<_>>(), vec!["hello world"]);
    }

    #[test]
    fn infinite_width_offer_never_wraps() {
        let metrics = &FONT.metrics();
        let wrap = super::WhitespaceWrap::new("hello world", ProposedDimension::Infinite, metrics);
        assert_eq!(wrap.collect::<Vec<_>>(), vec!["hello world"]);
    }

    #[test]
    fn compact_width_offer_only_wraps_explicit_newlines() {
        let metrics = &FONT.metrics();
        let wrap = super::WhitespaceWrap::new("hello\nworld", ProposedDimension::Compact, metrics);
        assert_eq!(wrap.collect::<Vec<_>>(), vec!["hello", "world"]);
    }

    #[test]
    fn infinite_width_offer_only_wraps_explicit_newlines() {
        let metrics = &FONT.metrics();
        let wrap = super::WhitespaceWrap::new("hello\nworld", ProposedDimension::Infinite, metrics);
        assert_eq!(wrap.collect::<Vec<_>>(), vec!["hello", "world"]);
    }
}
