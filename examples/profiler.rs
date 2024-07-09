use buoyant::{
    environment::DefaultEnvironment,
    font::TerminalChar,
    layout::{Layout, VerticalAlignment},
    primitives::Size,
    render::Render,
    render_target::{FixedTextBuffer, RenderTarget},
    view::{Divider, HStack, HorizontalTextAlignment, Spacer, Text, VStack, View},
};

fn main() {
    let mut target = FixedTextBuffer::<100, 100>::default();

    target.clear();
    let mut size = target.size();
    println!("Size {:?}", size);

    let env = DefaultEnvironment;

    let font = TerminalChar {};
    let stack = VStack::three(
    HStack::three(
        Text::char(
            "This text is centered horizontally in the middle of its space\nThe stack however, has bottom alignment.",
            &font,
                )
                .multiline_text_alignment(HorizontalTextAlignment::Center),
        Divider::default(),
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
        Spacer::default(),
        Text::char(
            "This is several lines of text.\nEach line is centered in the available space.\n Spacers are used to fill all the remaining verical space and align the content within it.\n2 points of padding are around this text",
            &font,
        )
            .multiline_text_alignment(HorizontalTextAlignment::Center)
            .padding(2),
        Divider::default(),
        ),
    );

    println!("View size {}", std::mem::size_of_val(&stack));
    println!("Env size {}", std::mem::size_of_val(&env));
    target.clear();
    for width in 1..100 {
        for height in 1..100 {
            size = Size::new(width, height);
            let layout = stack.layout(size, &env);
            stack.render(&mut target, &layout, &env);
        }
    }
}
