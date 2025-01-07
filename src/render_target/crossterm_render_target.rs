// use crossterm::{
//     cursor, execute,
//     style::{self, Stylize},
//     terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
//     ExecutableCommand as _, QueueableCommand,
// };
// use embedded_graphics::prelude::DrawTarget;
//
// #[cfg(feature = "std")]
// use std::io::{stdout, Stdout, Write};
//
// use crate::{font::CharacterBufferFont, primitives::Size};
//
// use super::RenderTarget;
//
// pub struct CrosstermRenderTarget {
//     stdout: Stdout,
// }
//
// impl CrosstermRenderTarget {
//     pub fn enter_fullscreen(&mut self) {
//         execute!(self.stdout, EnterAlternateScreen).unwrap();
//     }
//
//     pub fn exit_fullscreen(&mut self) {
//         execute!(self.stdout, LeaveAlternateScreen).unwrap();
//     }
//
//     pub fn flush(&mut self) {
//         self.stdout.flush().unwrap();
//     }
//
//     pub fn clear(&mut self) {
//         _ = self
//             .stdout
//             .execute(terminal::Clear(terminal::ClearType::All))
//             .unwrap();
//     }
// }
//
// impl Default for CrosstermRenderTarget {
//     fn default() -> Self {
//         Self { stdout: stdout() }
//     }
// }
//
// impl Drop for CrosstermRenderTarget {
//     fn drop(&mut self) {
//         self.flush();
//         execute!(self.stdout, LeaveAlternateScreen).unwrap();
//     }
// }
//
// impl DrawTarget for CrosstermRenderTarget {
//     type Color = crossterm::style::Colors;
//
//     fn size(&self) -> Size {
//         crossterm::terminal::size()
//             .map(|(w, h)| Size::new(w, h))
//             .unwrap_or_default()
//     }
//
//     fn draw(&mut self, point: crate::primitives::Point, color: crossterm::style::Colors) {
//         let mut styled_char = ' '.stylize();
//         if let Some(foreground) = color.foreground {
//             styled_char = styled_char.with(foreground);
//         }
//         if let Some(background) = color.background {
//             styled_char = styled_char.on(background);
//         }
//         self.stdout
//             .queue(cursor::MoveTo(point.x as u16, point.y as u16))
//             .unwrap()
//             .queue(style::PrintStyledContent(styled_char))
//             .unwrap();
//     }
// }
