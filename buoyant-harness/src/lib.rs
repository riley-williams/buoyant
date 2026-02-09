//! CLI test harness and workflow runner for buoyant UI applications.
//!
//! This crate provides a complete framework for automated UI testing,
//! screenshot capture, and workflow-based debugging of buoyant views.
//!
//! # Quick Start
//!
//! ```ignore
//! use buoyant_harness::{Args, WorkflowRunner, workflow};
//! use clap::Parser;
//!
//! fn main() {
//!     let args = Args::parse();
//!     let runner = WorkflowRunner::new(args)
//!         .register(my_workflow());
//!     runner.run().unwrap();
//! }
//!
//! fn my_workflow() -> buoyant_harness::WorkflowEntry {
//!     workflow("demo", |harness| {
//!         harness.render("initial")?;
//!         harness.next();
//!         harness.select();
//!         harness.render("after_select")?;
//!         Ok(())
//!     })
//! }
//! ```

mod config;
mod runner;
mod test_harness;

pub use buoyant::app::{App, Harness};
pub use config::HarnessConfig;
pub use runner::{RunnerDefaults, WorkflowEntry, WorkflowFn, WorkflowRunner, workflow};
pub use test_harness::TestHarness;

// Re-export clap::Parser so consumers can parse Args without depending on clap directly.
pub use clap::Parser;

use std::path::PathBuf;

/// CLI arguments for the workflow runner.
///
/// Use `Args::parse()` to parse command-line arguments, then pass them
/// to [`WorkflowRunner::new`].
#[derive(clap::Parser, Debug, Clone)]
#[command(name = "buoyant-harness")]
#[command(about = "Workflow runner for UI debugging, testing, and agent-driven development")]
pub struct Args {
    /// Run a specific workflow in visual debugging mode (displays in window).
    #[arg(long, value_name = "WORKFLOW")]
    pub show: Option<String>,

    /// Run only specific workflows (space-separated names).
    #[arg(long, num_args = 1..)]
    pub workflows: Vec<String>,

    /// Output directory for workflow results.
    #[arg(short = 'o', long, default_value = "./workflow_output")]
    pub output_dir: PathBuf,

    /// Time between steps in milliseconds.
    #[arg(long, default_value = "1000")]
    pub step_time: u64,

    /// Disable focus overlay on screenshots.
    #[arg(long)]
    pub no_overlay: bool,
}
