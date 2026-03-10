# Copilot Instructions

Buoyant is a Rust library for writing and rendering SwiftUI-like views for embedded systems and constrained devices. It targets `no_std` environments and draws to `embedded-graphics` `DrawTarget`s (e.g., LCD displays), terminals via crossterm, and a software simulator.

## Build, Test, and Lint

```bash
# Build
cargo build --all-features

# Full test suite
cargo test --all --all-features

# Single test
cargo test test_name

# Single test with output
cargo test test_name -- --nocapture

# Tests in a specific module/file
cargo test --test vstack

# Lint
cargo clippy --all --all-features -- -D warnings

# Format check
rustfmt **/*.rs --check --edition 2024

# Format fix
rustfmt **/*.rs --edition 2024

# Feature powerset check
cargo hack check --feature-powerset --no-dev-deps

# Documentation book tests
bash scripts/test-book.sh

# Coverage
cargo llvm-cov --all-features --package buoyant --lcov

# Semver compatibility
cargo semver-checks
```

## Architecture

The library is structured around a **three-phase pipeline**: layout → render tree → draw.

**Core trait hierarchy:**
- `ViewMarker` — marker trait
- `ViewLayout<Captures>` — provides `layout()`, `build_state()`, and associated `State`, `Renderables`, `FocusTree` types
- `View<Color, Captures>` — combines `ViewLayout` where `Renderables: Render<Color>`
- `Render<Color>` — produced by layout, implements `render()` and `render_animated()`

**Key modules:**
- `src/view/` — SwiftUI-like view components (`VStack`, `HStack`, `ZStack`, `Text`, `Button`, `ForEach`, `ScrollView`, `ViewThatFits`, `MatchView`, etc.)
- `src/view/modifier.rs` — View modifiers (`.frame()`, `.padding()`, `.opacity()`, `.foreground_color()`, etc.)
- `src/render/` — Render tree node types that implement `Render<Color>` after layout
- `src/app/` — `App` lifecycle, the `Harness` trait, `Tracked` state, render tree management
- `src/primitives/` — Geometry types (`Point`, `Size`, `Dimensions`, shapes)
- `src/layout.rs` — `Alignment`, `LayoutDirection`, `ProposedDimensions`, `ResolvedLayout`
- `src/animation.rs` — `Animation` and `Curve` types
- `src/font/` — Font abstractions (`u8g2`, `rusttype`, character buffer)
- `src/render_target/` — `RenderTarget`, `Brush`, `Surface` abstractions
- `src/transition.rs` — View transition animations

**Color is a generic type parameter** throughout the entire stack. `Rgb565`/`Rgb888` are used for embedded displays; `char` is used for terminal/test rendering.

**`Captures` is a type parameter** on `ViewLayout`/`View` that represents external mutable state (closures capture it). It defaults to `()` for static views.

**Workspace crates:**
- `buoyant` — the main library
- `buoyant-harness` — test/dev harness with screenshot capture, state serialization, CLI runner
- `buoyant-examples/battery` — example application

## Key Conventions

**View component structure:** Each view component (e.g., `VStack`) owns its children and provides `ViewLayout` impl with three associated types (`State`, `Renderables`, `FocusTree`). `layout()` returns a `ResolvedLayout<Self::Renderables>` containing both resolved geometry and the render tree.

**`ProposedDimensions`:** Layout is driven by proposals. A parent proposes `ProposedDimensions` (each axis is `Exact(i32)` or `Infinite`) to children, which return a `ResolvedLayout` with `resolved_size: Dimensions`.

**`render/collections.rs`:** Contains `OneOf2` through `OneOf10` union types used when a view can produce one of several render node types (e.g., conditional views, `MatchView`).

**`Interpolate` trait:** All animated types implement `Interpolate`. Animations blend between two render trees using this trait.

**Feature flags:**
- `embedded-graphics` — enables embedded-graphics draw target support
- `rusttype-fonts` — enables TrueType font rendering
- `crossterm` — enables terminal rendering
- `simulator` — enables software simulation via embedded-graphics-simulator

**Workspace lints (enforced globally):**
- `unsafe_code` — `forbid`
- `missing_debug_implementations` — `deny`
- Clippy pedantic with selective `allow`/`deny` overrides per crate

**Testing patterns:**
- Tests live in `tests/` organized by component (e.g., `tests/vstack.rs`, `tests/animation.rs`)
- Shared helpers and macros are in `tests/common/`
- `assert_str_grid_eq!` — macro for visual grid comparisons (terminal rendering)
- `DefaultEnvironment::non_animated()` — standard test environment
- Typical test flow: construct view → `build_state()` → `layout()` → assert on `resolved_size` or render output
- The `buoyant-harness` crate provides `Harness` trait with `send()`, `focus_forward()`, `next()`, `select()` etc. for integration tests
