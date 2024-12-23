use buoyant::font::TerminalCharFont;
use buoyant::primitives::Point;
use buoyant::render::CharacterRender;
use buoyant::view::{CharacterRenderExtensions, LayoutExtensions};
use buoyant::{
    environment::DefaultEnvironment,
    layout::{Layout, VerticalAlignment},
    primitives::Size,
    render_target::{CharacterRenderTarget, CrosstermRenderTarget},
    view::{Divider, HStack, HorizontalTextAlignment, Rectangle, Spacer, Text, VStack, ZStack},
};
use crossterm::event::{read, Event};
use crossterm::style::Colors;

fn main() {
    let mut target = CrosstermRenderTarget::default();

    let blank_color = Colors {
        foreground: None,
        background: None,
    };

    target.enter_fullscreen();
    target.clear(blank_color);
    let mut size = target.size();
    println!("Size {:?}", size);

    let env = DefaultEnvironment::new(blank_color);
    let font = TerminalCharFont {};
    let stack = VStack::new((
        HStack::new((
            Text::str(
                "This red text is aligned to the leading edge of its space\nThe stack however, has bottom alignment.",
                &font,
            )
                .multiline_text_alignment(HorizontalTextAlignment::Leading)
                .foreground_color(
                    Colors { foreground: Some(crossterm::style::Color::Red), background: None },
                ),
            Spacer::default(),
            Text::str(
                "This text is aligned to the right, with trailing multi-line text alignment",
                &font,
            )
                .multiline_text_alignment(HorizontalTextAlignment::Trailing)
                .flex_frame(Some(10), Some(35), None, None, None, None),
        ))
            .with_spacing(1)
            .with_alignment(VerticalAlignment::Bottom),
        Divider::default(),
        VStack::new((
            ZStack::two(
                Rectangle
                    .foreground_color(
                        Colors {
                            foreground: Some(crossterm::style::Color::Rgb { r: 0, g: 255, b: 0 }),
                            background: Some(crossterm::style::Color::Rgb { r: 127, g: 0, b: 0 })
                        }
                    ),
                Text::str(
                    "This is in a fixed size box",
                    &font,
                )
                    .frame(Some(10), Some(10), None, None),
            ),
            Text::str(
                "This is several lines of text.\nEach line is centered in the available space.\n The rectangle fills all the remaining verical space and aligns the content within it.\n2 points of padding are around this text",
                &font,
            )
                .multiline_text_alignment(HorizontalTextAlignment::Center)
                .foreground_color(
                    Colors { foreground:Some(crossterm::style::Color::Rgb { r: 255, g: 0, b: 255 }), background: None }
                )
                .padding(2),
            Divider::default()
                .foreground_color(Colors { foreground: Some(crossterm::style::Color::DarkYellow), background: None })
        )),
    ));

    println!("View size {}", std::mem::size_of_val(&stack));
    println!("Env size {}", std::mem::size_of_val(&env));

    let layout = stack.layout(target.size(), &env);
    stack.render(&mut target, &layout, Point::zero(), &env);

    target.flush();

    loop {
        // `read()` blocks until an `Event` is available
        match read().unwrap() {
            Event::FocusGained => (),
            Event::FocusLost => (),
            Event::Key(event) => {
                if event.code == crossterm::event::KeyCode::Char('q') {
                    break;
                }
            }
            Event::Mouse(_) => (),
            Event::Resize(width, height) => {
                target.clear(blank_color);
                size = Size::new(width, height);
                let layout = stack.layout(size, &env);
                stack.render(&mut target, &layout, Point::zero(), &env);

                target.flush();
            }
            Event::Paste(_) => (),
        }
    }
}
