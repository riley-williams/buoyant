use buoyant::pixel::CrosstermColorSymbol;
use buoyant::view::ViewExtensions;
use buoyant::{
    environment::DefaultEnvironment,
    font::TerminalChar,
    layout::{Layout, VerticalAlignment},
    primitives::Size,
    render::Render,
    render_target::{CrosstermRenderTarget, RenderTarget},
    style::horizontal_gradient::HorizontalGradient,
    view::{Divider, HStack, HorizontalTextAlignment, Rectangle, Spacer, Text, VStack, ZStack},
};
use crossterm::event::{read, Event};

fn main() {
    let mut target = CrosstermRenderTarget::default();

    target.enter_fullscreen();
    target.clear();
    let mut size = target.size();
    println!("Size {:?}", size);

    let env = DefaultEnvironment::new(CrosstermColorSymbol::new(' '));
    let font = TerminalChar {};
    let stack = VStack::three(
        HStack::three(
            Text::char(
                "This red text is aligned to the leading edge of its space\nThe stack however, has bottom alignment.",
                &font,
            )
                .multiline_text_alignment(HorizontalTextAlignment::Leading)
                .foreground_style(
                    CrosstermColorSymbol::new(' ')
                        .with_foreground(crossterm::style::Color::Red)
                ),
            Spacer::default(),
            Text::char(
                "This text is aligned to the right, with trailing multi-line text alignment",
                &font,
            )
            .multiline_text_alignment(HorizontalTextAlignment::Trailing)
            .flex_frame(Some(10), Some(35), None, None, None, None),
        )
        .spacing(1)
        .alignment(VerticalAlignment::Bottom),
        Divider::default(),
        VStack::three(
            ZStack::two(
                Rectangle
                    .foreground_style(HorizontalGradient::new(
                        CrosstermColorSymbol::new('#')
                            .with_foreground(crossterm::style::Color::Rgb { r: 0, g: 255, b: 0 })
                            .with_background(crossterm::style::Color::Rgb { r: 127, g: 0, b: 0 }),
                        CrosstermColorSymbol::new('@')
                            .with_foreground(crossterm::style::Color::Rgb { r: 127, g: 0, b: 255 })
                            .with_background(crossterm::style::Color::Rgb { r: 0, g: 0, b: 127 }),
                        )
                    ),
                Text::char(
                    "This is in a fixed size box",
                    &font,
                ).frame(Some(10), Some(10), None, None),
            ),
            Text::char(
                "This is several lines of text.\nEach line is centered in the available space.\n The rectangle fills all the remaining verical space and align the content within it.\n2 points of padding are around this text",
                &font,
            )
                .multiline_text_alignment(HorizontalTextAlignment::Center)
                .padding(2),
            Divider::default()
            .foreground_style(CrosstermColorSymbol::new(' ').with_foreground(crossterm::style::Color::DarkYellow))
        ),
    );

    println!("View size {}", std::mem::size_of_val(&stack));
    println!("Env size {}", std::mem::size_of_val(&env));

    let layout = stack.layout(target.size(), &env);
    stack.render(&mut target, &layout, &env);

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
                target.clear();
                size = Size::new(width, height);
                let layout = stack.layout(size, &env);
                stack.render(&mut target, &layout, &env);

                target.flush();
            }
            Event::Paste(_) => (),
        }
    }
}
