# buoyant-harness

CLI test harness and workflow runner for [buoyant](https://github.com/riley-williams/buoyant) UI applications.

`buoyant-harness` provides a framework for automated UI testing, screenshot capture, state serialization, and visual debugging of buoyant views — ideal for agent-driven development, regression testing, and interactive prototyping.

## Features

- **`TestHarness`** — Wraps the core `App` type with screenshot capture (PNG), state serialization (JSON), focus overlay rendering, and an optional live simulator window for visual debugging.
- **`WorkflowRunner`** — Manages and executes multiple named workflows, with parallel execution via rayon and CLI-based filtering.
- **`Harness` trait** — A shared trait (defined in the core `buoyant` crate) providing default implementations for focus navigation (`next`, `previous`, `select`, `blur`, `focus_forward`, `focus_backward`, `tap`, etc.) so custom harness implementations get all convenience methods for free.
- **CLI argument parsing** — Built-in `clap`-based `Args` type for controlling output directories, step timing, workflow selection, overlay toggling, and visual debugging mode.

## Quick Start

Add `buoyant-harness` as a dependency:

```toml
[dependencies]
buoyant-harness = { path = "../buoyant-harness" }
```

Define views, create workflows, and run them:

```rust
use buoyant_harness::{Args, Harness, Parser, WorkflowRunner, workflow, WorkflowEntry};

fn main() {
    let args = Args::parse();
    let runner = WorkflowRunner::new(args)
        .register(my_workflow());
    runner.run().unwrap();
}

fn my_workflow() -> WorkflowEntry {
    workflow("my_workflow", |config| {
        let mut h = config.init(my_view, MyState::default())?;

        h.render("initial")?;       // screenshot + state JSON
        h.focus_forward();           // acquire focus
        h.next();                    // move to next element
        h.select();                  // activate focused element
        h.render("after_select")?;   // capture new state

        Ok(())
    })
}
```

## CLI Usage

```sh
# Run all workflows in parallel (default)
cargo run --example agentic

# Run a specific workflow in visual debugging mode (opens a window)
cargo run --example agentic -- --show demo

# Run only specific workflows
cargo run --example agentic -- --workflows counter toggle

# Custom output directory
cargo run --example agentic -- -o ./my_output

# Custom step timing (milliseconds between steps)
cargo run --example agentic -- --step-time 500

# Disable focus overlay on screenshots
cargo run --example agentic -- --no-overlay
```

## Architecture

### `App` (core buoyant crate)

The `App<V, S>` type manages the full view/render tree lifecycle — view function, state, source/target render trees for animated transitions, focus state, and event handling. It deliberately does **not** own a render target, supporting both simple full-framebuffer rendering and advanced sliced/DMA schemes used in embedded displays.

### `Harness` trait (core buoyant crate)

A trait with only two required methods (`send` and `send_with_group`) and default implementations for all navigation convenience methods. `App` implements this trait, and so does `TestHarness`.

### `TestHarness` (this crate)

Wraps `App` via `Deref`/`DerefMut` and adds:

- **`render(step_name)`** — Renders the current view to a PNG screenshot and serializes state to JSON, with optional focus overlay. In visual mode, displays animated transitions in a simulator window.
- **Focus overlay** — Draws a yellow stroke around the currently focused element for visual debugging.
- **Output organization** — Each workflow gets its own directory with `images/` and `state/` subdirectories, files prefixed with step numbers for correct sorting.

### `WorkflowRunner`

Registers named workflows and executes them:

- **Parallel mode** (default) — All workflows run simultaneously via rayon.
- **Visual mode** (`--show <name>`) — A single workflow runs synchronously with a live simulator window showing animated transitions.
- **Filtering** (`--workflows <names...>`) — Run only specific workflows by name.

## Output Structure

```
workflow_output/
├── demo/
│   ├── images/
│   │   ├── 001_initial.png
│   │   ├── 002_focused.png
│   │   └── ...
│   └── state/
│       ├── 001_initial.json
│       ├── 002_focused.json
│       └── ...
├── counter/
│   ├── images/
│   └── state/
└── toggle/
    ├── images/
    └── state/
```

## License

Licensed under either of [Apache License, Version 2.0](../LICENSE-APACHE) or [MIT License](../LICENSE-MIT) at your option.

