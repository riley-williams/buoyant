# Fonts

- [**embedded-graphics monospace**](./fonts/embedded-graphics-monospace.md):
  Fixed-width fonts from the `embedded-graphics` crate, perfect for simple displays and
  consistent spacing.

- [**U8g2**](./fonts/u8g2.md):
  A rich collection of fonts ported from the U8g2 library, offering more variety in styles
  and sizes.

- [**RustType**](./fonts/rusttype.md):
  Display common font formats (.otf, .ttf) through the [`rusttype`](https://crates.io/crates/rusttype)
  crate for high-quality text rendering, support for variable sizing, and hinted antialiasing.

## Supported Features

| Font | Precise Bounds | Variable Sizing | Hinted Antialiasing |
| --------------- | --------------- | --------------- | --------------- |
| EG Monospace | - | - | - |
| U8g2 | ✅ | - | - |
| rusttype | ✅ | ✅ | ✅ |

### Precise Bounds

By default, Buoyant will lay out text such that adjacent `Text` views in `HStack`s and
`VStack`s are consistently spaced and aligned regardless of their content. This is achieved
by extending the view width by the last glyph's advance, and the height by the line spacing.

However, you may want the view's bounds to tightly fit the actual drawn pixels of the text,
such as when adding backgrounds or borders to text. In this case, you can enable precise bounds:

```rust
# extern crate buoyant;
# extern crate u8g2_fonts;
# use buoyant::view::prelude::*;
# use u8g2_fonts::{fonts, FontRenderer};
#
# static HELVETICA: FontRenderer = FontRenderer::new::<fonts::u8g2_font_helvR12_tr>();
#
Text::new("Buoyant", &HELVETICA)
    .with_precise_bounds()
# ;
```

### Variable Sizing

Fonts which have no intrinsic size, such as TrueType fonts, can be rendered at variable
sizes using the `.with_font_size(pts)` modifier:

```rust,ignore
Text::new("hello", &TTF_FONT)
   .with_font_size(22)
```

If no size is specified, a default size (12pt) will be used.

### Hinted Antialiasing

Buoyant supports hinted antialiased rendering which differs from standard antialiasing in
that a background color must be provided to allow the render target to blend edges.
The actual underlying pixels are not read.

The hint can be set several ways, including:

- Using the `.hint_background_color(color)` modifier, to manually specify a background color
  on a view subtree,
- Using the `.background_color(color, shape)` modifier, which renders a shape with the specified
  fill color behind the view, automatically setting the hint, or
- Providing a hint when initializing the render target, such as
  `EmbeddedGraphicsRenderTarget::new_hinted(&mut display, color);`.

Setting a hint is not required, and text will render without antialiasing if no hint is provided.
