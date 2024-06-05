use core::cmp::max;

use crate::{
    font::{Font, TextBufferFont},
    layout::{Environment, Layout, PreRender},
    primitives::{iint, uint, Point, Size},
    render::{Render, RenderTarget},
};

#[derive(Clone, Copy, Debug, PartialEq, Default)]
pub enum HorizontalTextAlignment {
    #[default]
    Leading,
    Center,
    Trailing,
}

impl HorizontalTextAlignment {
    pub fn align(&self, available: iint, content: iint) -> iint {
        match self {
            Self::Leading => 0,
            Self::Center => (available - content) / 2,
            Self::Trailing => available - content,
        }
    }
}

pub struct Text<'a, F, const N: usize> {
    pub text: &'a str,
    pub font: F,
    pub alignment: HorizontalTextAlignment,
}

/// The default maximum number of lines to cache
/// Generally only appilcable in no_std environments
/// Beyond this, the text layout will be recalculated
/// during rendering
const TEXT_LINE_LEN: usize = 10;

impl<F: Font> Text<'_, F, TEXT_LINE_LEN> {
    pub fn new(text: &str, font: F) -> Text<F, TEXT_LINE_LEN> {
        Text {
            text,
            font,
            alignment: HorizontalTextAlignment::default(),
        }
    }
}

impl<'a, F: Font, const N: usize> Text<'a, F, N> {
    /// Set the maximum number of lines to cache.
    /// This can be useful to reduce the memory footprint of the text view
    /// layout cache.
    /// The number of cached lines does not affect the size of the text view itself.
    pub fn max_lines<const M: usize>(self) -> Text<'a, F, M> {
        Text {
            text: self.text,
            font: self.font,
            alignment: self.alignment,
        }
    }

    pub fn multiline_text_alignment(self, alignment: HorizontalTextAlignment) -> Text<'a, F, N> {
        Text { alignment, ..self }
    }
}

// TODO: The size of this could probably be reduced with just a little bit of effort
// Maybe the view cache should be a generic so smaller caches can be used
// This occupies the vast majority of the size of the layout cache, only really for no_std support
pub struct TextLayoutCache<'a, const N: usize> {
    /// The cached lines and their lengths
    lines: [Option<(uint, &'a str)>; N],
    did_exceed_cache: bool,
    remaining: &'a str,
}

impl<'a, F: Font, const N: usize> Layout for Text<'a, F, N> {
    type Cache<'b> = TextLayoutCache<'a, N> where Self: 'b;

    fn layout(
        &self,
        offer: Size,
        _env: &dyn Environment,
    ) -> PreRender<'_, Self, TextLayoutCache<'a, N>> {
        let font_height = self.font.line_height();
        let mut line_cache = [Option::None; N];
        let mut cache_index = 0;
        let mut did_exceed_cache = false;

        let mut consumed_height = 0;

        // track the longest line
        let mut max_line_width_points = 0;

        let mut remaining_slice = self.text;
        let mut uncached_slice = &self.text[0..0];

        // layout a new line as long as there is vertical space for it, always layout at least one line
        while !remaining_slice.is_empty() && consumed_height + font_height <= offer.height {
            // find the longest line that fits horizontally without truncating mid-word, unless
            // only one word fits
            let mut whole_width_points = 0;
            // used to backtrack to the last whole word if needed
            let mut width_accumulator = 0;

            let mut char_indices = remaining_slice.char_indices();

            let mut completed_index = 0;
            // accummulate by word until the line is too long
            loop {
                if let Some((index, char)) = char_indices.next() {
                    match char {
                        '\n' => {
                            // apply any accumulated width
                            whole_width_points += width_accumulator;
                            completed_index = index + 1;
                            break;
                        }
                        ' ' => {
                            whole_width_points += width_accumulator;
                            let char_width = self.font.character_width(' ');
                            // add the space to the accumulator so it is skipped if there are no
                            // other characters on the line
                            width_accumulator = char_width;

                            completed_index = index + 1;
                        }
                        _ => {
                            let char_width = self.font.character_width(char);
                            let candidate_width =
                                whole_width_points + width_accumulator + char_width;
                            if candidate_width > offer.width {
                                // if we reached the limit before the first word, break mid word
                                if whole_width_points == 0 {
                                    if index == 0 {
                                        // if the first character is too wide, use it and break
                                        whole_width_points = char_width;
                                        completed_index = 1;
                                    } else {
                                        // otherwise drop the character and break
                                        whole_width_points = width_accumulator;
                                        completed_index = index;
                                    }
                                }
                                break;
                            } else {
                                width_accumulator += char_width;
                            }
                        }
                    }
                } else {
                    // if we reached the end of the string, apply the collected word
                    whole_width_points += width_accumulator;
                    completed_index = remaining_slice.len();
                    break;
                }
            }

            if cache_index < N {
                line_cache[cache_index] =
                    Some((whole_width_points, &remaining_slice[..completed_index]));
                cache_index += 1;
            } else {
                if !did_exceed_cache {
                    uncached_slice = remaining_slice;
                }
                did_exceed_cache = true;
            }

            consumed_height += font_height;
            remaining_slice = &remaining_slice[completed_index..];

            max_line_width_points = max(max_line_width_points, whole_width_points);
        }

        let size = Size::new(max_line_width_points, consumed_height);
        PreRender {
            source_view: self,
            layout_cache: TextLayoutCache {
                lines: line_cache,
                did_exceed_cache,
                remaining: uncached_slice,
            },
            resolved_size: size,
        }
    }
}

impl<const N: usize> Render<char>
    for PreRender<'_, Text<'_, TextBufferFont, N>, TextLayoutCache<'_, N>>
{
    fn render(&self, target: &mut impl RenderTarget<char>, _env: &impl Environment) {
        let mut consumed_height: uint = 0;
        for (width, line) in self.layout_cache.lines.iter().filter_map(|l| *l) {
            let x = self
                .source_view
                .alignment
                .align(self.resolved_size.width as iint, width as iint);

            line.chars().enumerate().for_each(|(i, c)| {
                target.draw(Point::new(x + i as iint, consumed_height as iint), c);
            });
            consumed_height += 1;
        }

        if !self.layout_cache.did_exceed_cache {
            return;
        }

        // we already know the longest line
        let mut max_line_width_points = self.resolved_size.width;

        let mut remaining_slice = self.layout_cache.remaining;

        // layout a new line as long as there is vertical space for it, always layout at least one line
        while !remaining_slice.is_empty() && consumed_height < self.resolved_size.height {
            // find the longest line that fits horizontally without truncating mid-word, unless
            // only one word fits
            let mut whole_width_points = 0;
            // used to backtrack to the last whole word if needed
            let mut width_accumulator = 0;

            let mut char_indices = remaining_slice.char_indices();

            let mut completed_index = 0;
            // accummulate by word until the line is too long
            loop {
                if let Some((index, char)) = char_indices.next() {
                    match char {
                        '\n' => {
                            // apply any accumulated width
                            whole_width_points += width_accumulator;
                            completed_index = index + 1;
                            break;
                        }
                        ' ' => {
                            whole_width_points += width_accumulator;
                            let char_width = self.source_view.font.character_width(' ');
                            // add the space to the accumulator so it is skipped if there are no
                            // other characters on the line
                            width_accumulator = char_width;

                            completed_index = index + 1;
                        }
                        _ => {
                            let char_width = self.source_view.font.character_width(char);
                            let candidate_width =
                                whole_width_points + width_accumulator + char_width;
                            if candidate_width > self.resolved_size.width {
                                // if we reached the limit before the first word, break mid word
                                if whole_width_points == 0 {
                                    if index == 0 {
                                        // if the first character is too wide, use it and break
                                        whole_width_points = char_width;
                                        completed_index = 1;
                                    } else {
                                        // otherwise drop the character and break
                                        whole_width_points = width_accumulator;
                                        completed_index = index;
                                    }
                                }
                                break;
                            } else {
                                width_accumulator += char_width;
                            }
                        }
                    }
                } else {
                    // if we reached the end of the string, apply the collected word
                    whole_width_points += width_accumulator;
                    completed_index = remaining_slice.len();
                    break;
                }
            }
            let x = self
                .source_view
                .alignment
                .align(self.resolved_size.width as iint, whole_width_points as iint);

            remaining_slice[..completed_index]
                .chars()
                .enumerate()
                .for_each(|(i, c)| {
                    target.draw(Point::new(x + i as iint, consumed_height as iint), c);
                });

            consumed_height += 1;

            remaining_slice = &remaining_slice[completed_index..];

            max_line_width_points = max(max_line_width_points, whole_width_points);
        }
    }
}
