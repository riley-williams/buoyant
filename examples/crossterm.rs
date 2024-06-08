use buoyant::{
    font::TextBufferFont,
    layout::{Environment, Layout, VerticalAlignment},
    primitives::Size,
    render::Render,
    render_target::{CrosstermRenderTarget, RenderTarget},
    view::{Divider, HStack, HorizontalTextAlignment, Padding, Spacer, Text, VStack},
};
use crossterm::event::{read, Event};

fn main() {
    let mut target = CrosstermRenderTarget::default();

    target.enter_fullscreen();
    target.clear();
    let mut size = target.size();
    println!("Size {:?}", size);

    let env = TestEnv {};

    let stack = VStack::three(
    HStack::three(
        Text::new(
            "This text is centered horizontally in the middle of its space\nThe stack however, has bottom alignment.",
            TextBufferFont::default(),
                )
                .multiline_text_alignment(HorizontalTextAlignment::Center),
        Divider::default(),
        Text::new(
            "This text is aligned to the right, with trailing multi-line text alignment",
            TextBufferFont::default(),
                )
                .multiline_text_alignment(HorizontalTextAlignment::Trailing),
            )
            .spacing(1)
            .alignment(VerticalAlignment::Bottom),
    Divider::default(),
    VStack::three(
        Spacer::default(),
        Padding::new(2,
            Text::new(
                "This is several lines of text.\nEach line is centered in the available space.\n Spacers are used to fill all the remaining verical space and align the content within it.\n2 points of padding are around this text",
                TextBufferFont::default(),
                    )
                    .multiline_text_alignment(HorizontalTextAlignment::Center),
                ),
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

struct TestEnv;
impl Environment for TestEnv {}
