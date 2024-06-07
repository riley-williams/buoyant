use buoyant::{
    font::TextBufferFont,
    layout::{Environment, Layout, VerticalAlignment},
    primitives::{Point, Size},
    render::{ClippingRenderProxy, FixedTextBuffer, Render, RenderTarget},
    view::{Divider, HStack, HorizontalTextAlignment, Spacer, Text, VStack},
};

fn main() {
    let mut buffer = FixedTextBuffer::<30, 20>::default();

    let env = TestEnv {};

    let stack = VStack::three(
        Divider::default(),
        VStack::three(
            HStack::three(
                Text::new("This is centered in its space", TextBufferFont::default())
                    .max_lines::<0>()
                    .multiline_text_alignment(HorizontalTextAlignment::Center),
                Spacer::default(),
                Text::new("This is aligned to the right", TextBufferFont::default())
                    .max_lines::<0>()
                    .multiline_text_alignment(HorizontalTextAlignment::Trailing),
            )
            .spacing(1)
            .alignment(VerticalAlignment::Bottom),
            Divider::default(),
            Text::new("This is a single line of text", TextBufferFont::default())
                .max_lines::<0>()
                .multiline_text_alignment(HorizontalTextAlignment::Center),
        )
        .spacing(1),
        Divider::default(),
    );

    println!("View size {}", std::mem::size_of_val(&stack));
    println!("Env size {}", std::mem::size_of_val(&env));
    for i in 3..20 {
        let size = Size::new(10, i);
        let layout = stack.layout(size, &env);
        println!("Layout size {}", std::mem::size_of_val(&layout));
        layout.render(&mut buffer, &env);

        let proxy = ClippingRenderProxy::new(&mut buffer, Point::default(), size);
        println!("{}", buffer);
        buffer.clear();
    }
}

struct TestEnv;
impl Environment for TestEnv {}
