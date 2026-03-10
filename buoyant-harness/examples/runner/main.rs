//! # Example: Workflow Runner
//!
//! A harness for running UI workflows for debugging, testing, or agent-driven
//! development. This enables automated navigation through UI states with
//! screenshot capture and state serialization.
//!
//! ## Usage
//!
//! Run all workflows in parallel (default):
//! ```sh
//! cargo run --example agentic
//! ```
//!
//! Run a specific workflow in visual debugging mode:
//! ```sh
//! cargo run --example agentic -- --show demo
//! ```
//!
//! Run only specific workflows:
//! ```sh
//! cargo run --example agentic -- --workflows counter toggle
//! ```
//!
//! Disable focus overlay:
//! ```sh
//! cargo run --example agentic -- --no-overlay
//! ```
//!
//! Custom output directory:
//! ```sh
//! cargo run --example agentic -- -o ./my_output
//! ```
//!
//! Custom step timing (milliseconds):
//! ```sh
//! cargo run --example agentic -- --step-time 500
//! ```

mod views;
mod workflows;

use buoyant_harness::{Args, Parser, WorkflowRunner};

fn main() {
    let args = Args::parse();

    println!("Workflow Runner");
    println!("===============");
    println!("Output directory: {}", args.output_dir.display());
    println!("Step time: {}ms", args.step_time);
    println!("Focus overlay: {}", !args.no_overlay);

    if let Some(ref name) = args.show {
        println!("Visual mode: {name}");
    }
    if !args.workflows.is_empty() {
        println!("Running workflows: {:?}", args.workflows);
    }
    println!();

    // Create the workflow runner with CLI args
    let runner = WorkflowRunner::new(args)
        .register(workflows::demo_workflow())
        .register(workflows::counter_workflow())
        .register(workflows::toggle_workflow())
        .register(workflows::chart_line_workflow())
        .register(workflows::chart_bar_workflow())
        .register(workflows::chart_scatter_workflow())
        .register(workflows::chart_multi_workflow())
        .register(workflows::chart_demo_temperature())
        .register(workflows::chart_demo_sales())
        .register(workflows::chart_demo_scatter())
        .register(workflows::chart_demo_budget())
        .register(workflows::chart_demo_metrics())
        .register(workflows::chart_demo_sparklines());

    // List available workflows
    println!("Available workflows: {:?}", runner.workflow_names());
    println!();

    // Run workflows
    match runner.run() {
        Ok(()) => {
            println!();
            println!("All workflows completed successfully!");
        }
        Err(e) => {
            eprintln!();
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    }
}
