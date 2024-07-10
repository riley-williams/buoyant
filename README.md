# Buoyant

<!--toc:start-->

- [Buoyant](#buoyant)
  - [Goals](#goals)
  - [Layout and rendering](#layout-and-rendering)
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
- Support for both character-based and pixel-based rendering and layout
- Ability to support a variety of render devices (e.g., terminal,
  framebuffer, SPI display, ...)

## Layout and rendering

Layout code is shared across all pixel types. Views produce a layout
which can then be rendered to a render target, calling the rendering code
specific to the render target's pixel type.

## Available render targets

- `TextBuffer`: A basic fixed-size `char` buffer. Does not respect graphemes.
  This is primarily useful for testing and debugging.

- `CrossTerm`: Renders colored character-pixels to a terminal using
  the `crossterm` crate.

## Roadmap

Right now, all the core components exist to build and render static views.

These are the planned features, in order of priority:

- embedded-graphics trait implementations
- default pixel rendering for pixel-based render targets
- More robust Font support
  - embedded-graphics fonts
  - Embedded SPI displays with built-in fonts
- State management
  - Layout reuse
  - Animations
- Interactivity
  - click/tap routing
  - focus management / keyboard input

These things would be nice:

- Unicode / grapheme support for proper text handling outside embedded

## Usage notes

This project is a work in progress and should not be used in production.

At this point in time, all public API should be considered unstable,
and this library does not yet respect SemVer. Yeah I should have
started at 0.0.x. Sorry.

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
