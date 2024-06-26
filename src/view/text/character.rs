use crate::{
    font::CharacterFont,
    layout::{Environment, Layout, ResolvedLayout},
    primitives::{Point, Size},
    render::Render,
    render_target::RenderTarget,
};
use core::cmp::max;

use super::{HorizontalTextAlignment, Text};

impl<'a, F: CharacterFont> Text<'a, F> {
    pub fn char(text: &'a str, font: &'a F) -> Text<'a, F> {
        Text {
            text,
            font,
            alignment: HorizontalTextAlignment::default(),
        }
    }
}

impl<'a, F: CharacterFont> Text<'a, F> {
    pub fn multiline_text_alignment(self, alignment: HorizontalTextAlignment) -> Text<'a, F> {
        Text { alignment, ..self }
    }
}

impl<'a, F: CharacterFont> Layout for Text<'a, F> {
    type Sublayout<'b> = () where Self: 'b;

    fn layout(&self, offer: Size, _env: &dyn Environment) -> ResolvedLayout<()> {
        if offer.area() == 0 {
            return ResolvedLayout {
                sublayouts: (),
                resolved_size: Size::new(0, 0),
            };
        }

        let font_height = self.font.line_height();

        let mut consumed_height = 0;

        // track the longest line
        let mut max_line_width_points = 0;

        let mut remaining_slice = self.text;

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

            consumed_height += font_height;
            remaining_slice = &remaining_slice[completed_index..];

            max_line_width_points = max(max_line_width_points, whole_width_points);
        }
        let size = Size::new(max_line_width_points, consumed_height);
        ResolvedLayout {
            sublayouts: (),
            resolved_size: size,
        }
    }
}

impl<'a, F: CharacterFont> Render<char, ()> for Text<'a, F> {
    fn render(
        &self,
        target: &mut impl RenderTarget<char>,
        layout: &ResolvedLayout<()>,
        _env: &dyn Environment,
    ) {
        if layout.resolved_size.area() == 0 {
            return;
        }
        let mut consumed_height: u16 = 0;

        let mut remaining_slice = self.text;

        // layout a new line as long as there is vertical space for it, always layout at least one line
        while !remaining_slice.is_empty() && consumed_height < layout.resolved_size.height {
            // find the longest line that fits horizontally without truncating mid-word, unless
            // only one word fits
            let mut whole_width_points = 0;
            // used to backtrack to the last whole word if needed
            let mut width_accumulator = 0;

            let mut char_indices = remaining_slice.char_indices();

            let mut completed_index = 0;
            let mut last_renderable_index = 0;
            // accummulate by word until the line is too long
            loop {
                if let Some((index, char)) = char_indices.next() {
                    match char {
                        '\n' => {
                            // apply any accumulated width
                            whole_width_points += width_accumulator;
                            completed_index = index + 1;
                            last_renderable_index = index;
                            break;
                        }
                        ' ' => {
                            whole_width_points += width_accumulator;
                            let char_width = self.font.character_width(' ');
                            // add the space to the accumulator so it is skipped if there are no
                            // other characters on the line
                            width_accumulator = char_width;

                            completed_index = index + 1;
                            last_renderable_index = index;
                        }
                        _ => {
                            let char_width = self.font.character_width(char);
                            let candidate_width =
                                whole_width_points + width_accumulator + char_width;
                            if candidate_width > layout.resolved_size.width {
                                // if we reached the limit before the first word, break mid word
                                if whole_width_points == 0 {
                                    if index == 0 {
                                        // if the first character is too wide, use it and break
                                        whole_width_points = char_width;
                                        completed_index = 1;
                                        last_renderable_index = 1;
                                    } else {
                                        // otherwise drop the character and break
                                        whole_width_points = width_accumulator;
                                        completed_index = index;
                                        last_renderable_index = index;
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
                    last_renderable_index = completed_index;
                    break;
                }
            }
            let x = self
                .alignment
                .align(layout.resolved_size.width as i16, whole_width_points as i16);

            remaining_slice[..last_renderable_index]
                .chars()
                .enumerate()
                .for_each(|(i, c)| {
                    target.draw(Point::new(x + i as i16, consumed_height as i16), c);
                });

            consumed_height += 1;

            remaining_slice = &remaining_slice[completed_index..];
        }
    }
}
