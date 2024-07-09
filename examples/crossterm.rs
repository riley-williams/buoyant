use buoyant::view::View;
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
use rgb::RGB8;

fn main() {
    let mut target = CrosstermRenderTarget::default();

    target.enter_fullscreen();
    target.clear();
    let mut size = target.size();
    println!("Size {:?}", size);

    let env = DefaultEnvironment;
    let font = TerminalChar {};
    let stack = VStack::three(
        HStack::three(
            Text::char(
                "This red text is aligned to the leading edge of its space\nThe stack however, has bottom alignment.",
                &font,
            )
                .multiline_text_alignment(HorizontalTextAlignment::Leading).foreground_style(RGB8::new(255, 0, 0)),
            Spacer::default(),
            Text::char(
                "This text is aligned to the right, with trailing multi-line text alignment",
                &font,
            )
            .multiline_text_alignment(HorizontalTextAlignment::Trailing),
        )
        .spacing(1)
        .alignment(VerticalAlignment::Bottom),
        Divider::default(),
        VStack::three(
            ZStack::two(
                Rectangle::new(0)
                    .foreground_style(HorizontalGradient::new(
                        RGB8::new(0, 255, 0),
                        RGB8::new(0, 0, 255)
                        )),
                Text::char(
                    "This is a fancy rectangle",
                    &font,
                ),
            ),
            Text::char(
                "This is several lines of text.\nEach line is centered in the available space.\n The rectangle fills all the remaining verical space and align the content within it.\n2 points of padding are around this text",
                &font,
            )
                .multiline_text_alignment(HorizontalTextAlignment::Center)
                .padding(2),
            Divider::default(),
        ),
    );

    println!("View size {}", std::mem::size_of_val(&stack));
    println!("Env size {}", std::mem::size_of_val(&env));
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
                target.flush();
                size = Size::new(width, height);
                let layout = stack.layout(size, &env);
                stack.render(&mut target, &layout, &env);

                target.flush();
            }
            Event::Paste(_) => (),
        }
    }
}
