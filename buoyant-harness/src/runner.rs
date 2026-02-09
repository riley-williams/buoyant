//! Workflow runner with parallel execution support.
//!
//! The [`WorkflowRunner`] manages multiple workflows and can execute them
//! in parallel using rayon, or run a single workflow in visual debugging mode.

use std::sync::Arc;

use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::RgbColor;
use rayon::prelude::*;

use crate::{Args, HarnessConfig};

/// A boxed workflow function type.
pub type WorkflowFn = Box<dyn Fn(HarnessConfig) -> Result<(), String> + Send + Sync>;

/// A registered workflow with its name and function.
pub struct WorkflowEntry {
    /// The name of the workflow, used for filtering and output directories.
    pub name: &'static str,
    /// The workflow function to execute.
    pub run: WorkflowFn,
}

impl std::fmt::Debug for WorkflowEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("WorkflowEntry")
            .field("name", &self.name)
            .finish_non_exhaustive()
    }
}

/// Default configuration applied to all workflows in a runner.
#[derive(Debug, Clone)]
pub struct RunnerDefaults {
    /// Default display size.
    pub size: buoyant::primitives::Size,
    /// Default foreground color.
    pub foreground: Rgb888,
    /// Default background color.
    pub background: Rgb888,
    /// Default step time in milliseconds.
    pub step_time: u64,
}

impl Default for RunnerDefaults {
    fn default() -> Self {
        Self {
            size: buoyant::primitives::Size::new(480, 320),
            foreground: Rgb888::WHITE,
            background: Rgb888::BLACK,
            step_time: 1000,
        }
    }
}

/// Manages and executes multiple workflows.
///
/// Workflows are registered with [`register`](WorkflowRunner::register) and
/// executed with [`run`](WorkflowRunner::run). In headless mode, workflows
/// run in parallel using rayon. In visual mode (`--show`), a single workflow
/// runs synchronously with a simulator window.
///
/// # Example
///
/// ```ignore
/// use buoyant_harness::{Args, WorkflowRunner, workflow};
/// use clap::Parser;
///
/// let args = Args::parse();
/// let runner = WorkflowRunner::new(args)
///     .register(my_workflow())
///     .register(other_workflow());
/// runner.run().unwrap();
/// ```
#[derive(Debug)]
pub struct WorkflowRunner {
    workflows: Vec<WorkflowEntry>,
    /// Default configuration applied to all workflows.
    pub defaults: RunnerDefaults,
    args: Arc<Args>,
}

impl WorkflowRunner {
    /// Creates a new workflow runner with the given CLI arguments.
    #[must_use]
    pub fn new(args: Args) -> Self {
        Self {
            workflows: Vec::new(),
            defaults: RunnerDefaults::default(),
            args: Arc::new(args),
        }
    }

    /// Registers a workflow entry.
    #[must_use]
    pub fn register(mut self, entry: WorkflowEntry) -> Self {
        self.workflows.push(entry);
        self
    }

    /// Returns the names of all registered workflows.
    #[must_use]
    pub fn workflow_names(&self) -> Vec<&'static str> {
        self.workflows.iter().map(|w| w.name).collect()
    }

    /// Runs all (or filtered) workflows.
    ///
    /// - If `--show <name>` is specified, runs only that workflow in visual mode.
    /// - If `--workflows <names...>` is specified, runs only those workflows.
    /// - Otherwise, runs all workflows in parallel.
    ///
    /// # Errors
    ///
    /// Returns an error string if any workflow fails or if a specified workflow
    /// is not found.
    pub fn run(&self) -> Result<(), String> {
        // Filter workflows based on CLI args
        let workflows_to_run: Vec<&WorkflowEntry> = self.args.show.as_ref().map_or_else(
            || {
                if self.args.workflows.is_empty() {
                    self.workflows.iter().collect()
                } else {
                    self.workflows
                        .iter()
                        .filter(|w| self.args.workflows.contains(&w.name.to_string()))
                        .collect()
                }
            },
            |show_name| {
                self.workflows
                    .iter()
                    .filter(|w| w.name == show_name)
                    .collect()
            },
        );

        if workflows_to_run.is_empty() {
            if let Some(ref show_name) = self.args.show {
                return Err(format!(
                    "Workflow '{}' not found. Available: {:?}",
                    show_name,
                    self.workflow_names()
                ));
            }
            if !self.args.workflows.is_empty() {
                return Err(format!(
                    "No matching workflows found. Available: {:?}",
                    self.workflow_names()
                ));
            }
            println!("No workflows registered.");
            return Ok(());
        }

        // Visual mode runs a single workflow synchronously
        if self.args.show.is_some() {
            let workflow = workflows_to_run[0];
            println!("Running workflow '{}' in visual mode...", workflow.name);
            let config = self.create_config(workflow.name);
            return (workflow.run)(config);
        }

        // Parallel execution for headless mode
        println!(
            "Running {} workflow(s) in parallel...",
            workflows_to_run.len()
        );

        let results: Vec<(&str, Result<(), String>)> = workflows_to_run
            .par_iter()
            .map(|workflow| {
                let config = self.create_config(workflow.name);
                let result = (workflow.run)(config);
                (workflow.name, result)
            })
            .collect();

        // Report results
        let mut had_errors = false;
        for (name, result) in &results {
            match result {
                Ok(()) => println!("\u{2713} Workflow '{name}' completed successfully"),
                Err(e) => {
                    eprintln!("\u{2717} Workflow '{name}' failed: {e}");
                    had_errors = true;
                }
            }
        }

        if had_errors {
            Err("Some workflows failed".to_string())
        } else {
            Ok(())
        }
    }

    /// Creates a [`HarnessConfig`] for a specific workflow.
    fn create_config(&self, workflow_name: &str) -> HarnessConfig {
        let workflow_dir = self.args.output_dir.join(workflow_name);
        let images_dir = workflow_dir.join("images");
        let state_dir = workflow_dir.join("state");

        HarnessConfig {
            name: workflow_name.to_string(),
            size: self.defaults.size,
            foreground: self.defaults.foreground,
            background: self.defaults.background,
            step_time: self.args.step_time,
            show_overlay: !self.args.no_overlay,
            visual_mode: self.args.show.is_some(),
            images_dir,
            state_dir,
        }
    }
}

/// Helper function to create a [`WorkflowEntry`].
///
/// This is the recommended way to define workflows.
///
/// # Example
///
/// ```ignore
/// use buoyant_harness::{WorkflowEntry, workflow};
///
/// pub fn my_workflow() -> WorkflowEntry {
///     workflow("my_workflow", |config| {
///         let mut h = config.init(my_view, MyState::default())?;
///         h.render("initial")?;
///         Ok(())
///     })
/// }
/// ```
pub fn workflow<F>(name: &'static str, f: F) -> WorkflowEntry
where
    F: Fn(HarnessConfig) -> Result<(), String> + Send + Sync + 'static,
{
    WorkflowEntry {
        name,
        run: Box::new(f),
    }
}
