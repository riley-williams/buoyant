use buoyant::{
    font::TextBufferFont,
    layout::{Environment, Layout, VerticalAlignment},
    primitives::Size,
    render::{FixedTextBuffer, Render, RenderTarget},
    view::{HStack, HorizontalTextAlignment, Spacer, Text},
};

fn main() {
    let mut buffer = FixedTextBuffer::<80, 10>::default();

    let env = TestEnv {};

    let stack = HStack::three(
        Text::new("This is centered in its space", TextBufferFont::default())
            .max_lines::<0>()
            .multiline_text_alignment(HorizontalTextAlignment::Center),
        Spacer::default(),
        Text::new("This is aligned to the right", TextBufferFont::default())
            .max_lines::<0>()
            .multiline_text_alignment(HorizontalTextAlignment::Trailing),
    )
    .spacing(1)
    .alignment(VerticalAlignment::Bottom);

    println!("View size {}", std::mem::size_of_val(&stack));
    println!("Env size {}", std::mem::size_of_val(&env));
    for i in 6..80 {
        let layout = stack.layout(Size::new(i, buffer.size().height), &env);
        println!("Layout size {}", std::mem::size_of_val(&layout));
        layout.render(&mut buffer, &env);

        println!("{}", buffer);
        buffer.clear();
    }
}

struct TestEnv;
impl Environment for TestEnv {}
