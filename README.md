# Buoyant

<!--toc:start-->

- [Buoyant](#buoyant)
  - [Goals](#goals)
  - [Layout and rendering](#layout-and-rendering)
  - [Available views](#available-views)
  - [Available render targets](#available-render-targets)
  - [Roadmap](#roadmap)
  - [Usage notes](#usage-notes)
  - [License](#license)
  - [Contribution](#contribution)
  <!--toc:end-->

This is a library for writing and rendering SwiftUI-like layouts in Rust.

## Goals

- Embedded / no_std support:
  - Zero heap allocation
  - Minimal memory footprint
  - Immediate mode, CPU rendering
- Support for both text-based and pixel-based rendering and layout
  through the use of generic traits
- Ability to support a variety of render devices (e.g., terminal,
  framebuffer, SPI display, ...)

## Layout and rendering

- **View definition:** This follows the SwiftUI model of defining the
  view hierarchy entirely withing the type system.

- **Layout:** The view object is passed through the layout engine,
  which computes the size of each view. Just like SwiftUI, the parent
  offers a size, but children are free to choose their own size. This
  produces a statically typed layout tree that mirrors the structure
  of the view definition.

- **Render:** The render target is passed to the layout, and the layout
  engine renders the view hierarchy to the target. Minor layout
  computations such as text wrapping may be repeated, depending on the
  available cache size.

## Available views

- `Text`: A view that renders text. The text is wrapped to the width
  of the parent offer, attempting to respect word boundaries. Options
  are available for multiline text alignment.

- `HStack`, `VStack`, `ZStack`: Views that stack their children horizontally or
  vertically. The size of the stack is determined by the size of the
  children. Alignment and spacing options are available.

- `Spacer`: A view that takes up all available space in the parent
  view. It is aware of the layout direction it was presented in, and
  will expand in the appropriate dimension.

- `Divider`: A view that renders a horizontal or vertical line. Like
  `Spacer`, it is aware of the layout direction it was presented in,
  and will render as a line in the appropriate dimension.

- `Padding`: A view that adds padding around its child view. This is
  equivalent to the SwiftUI `padding` modifier.

- `Rectangle`: A view that renders a rectangle. The rectangle is
  filled with the foreground color.

## Available render targets

- `TextBuffer`: A basic fixed-size `char` buffer. Does not respect graphemes.
  This is primarily useful for testing and debugging.

## Roadmap

- embedded-graphics trait implementations
- Layout reuse
- Animations
- click/tap routing

## Usage notes

It is a work in progress and should not be used in production.

At this point in time, all public API should be considered unstable,
and this library does not yet respect SemVer.

## License

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license
  ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
