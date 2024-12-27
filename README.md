# Buoyant

<!--toc:start-->
- [Buoyant](#buoyant)
  - [Capabilities](#capabilities)
  - [Layout and rendering](#layout-and-rendering)
  - [Available render targets](#available-render-targets)
  - [Roadmap](#roadmap)
  - [Usage notes](#usage-notes)
  - [License](#license)
  - [Contribution](#contribution)
<!--toc:end-->

This is a library for writing and rendering SwiftUI-like layouts in Rust,
primarily intended for use on embedded systems.

## Capabilities

- Embedded / no_std support:
  - Zero heap allocation
  - Minimal memory footprint
- Support for both character-based and pixel-based rendering and layout
- Ability to support a variety of render devices (terminal,
  framebuffer, SPI display, ...)

## Layout and rendering

Layout code is shared across all pixel types. Views produce a layout
which can then be rendered to a render target, calling the rendering code
specific to the render target's pixel type. This allows creating support for
arbitrary render backends.

## Available render targets

- `embedded-graphics` displays, with "native" fonts and colors.
- `TextBuffer`: A basic fixed-size `char` buffer. Does not respect graphemes.
  This is primarily useful for testing and debugging.
- `CrossTerm`: Renders colored character-pixels to a terminal using
  the `crossterm` crate.

## Roadmap

Right now, core components exist to build and render a wide variety of
basic static views. In the current state, usability far exceeds manual
layout using embedded-graphics primitives directly.

These are the currently planned features:

### Distinct View and Widget trees

- State management
  - Layout reuse
  - Animation

- Interactivity
  - click/tap routing
  - focus management + keyboard input

### Rendering

- Canvas view for arbitrary path/shape/raster drawing
  - The rendering implementation exclusively targets embedded-graphics,
    but migrating everything to a canvas interface would enable reusing
    the rendering logic for other backends.
- Shape stroke/fill
- Embedded SPI displays with built-in fonts
- Alpha blending
  - Rendering is currently write-only, enabling framebufferless rendering

### Text

- Unicode breaking character support for better text wrapping on
  less resource-constrained devices.

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
