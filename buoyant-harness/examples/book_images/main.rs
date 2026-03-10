mod views;

use std::path::Path;

use buoyant_harness::{Args, Parser, WorkflowEntry, WorkflowRunner, workflow};
use embedded_graphics::pixelcolor::Rgb888;
use embedded_graphics::prelude::{RgbColor, WebColors};

macro_rules! book_image {
    ($name:expr, $book_path:expr, $view_fn:expr) => {
        (
            workflow($name, |mut config| {
                config.show_overlay = false;
                let mut h = config.init($view_fn, ())?;
                h.render("screenshot")?;
                Ok(())
            }),
            $book_path,
        )
    };
    ($name:expr, $book_path:expr, $view_fn:expr, bg: $bg:expr) => {
        (
            workflow($name, |mut config| {
                config.show_overlay = false;
                config.background = $bg;
                let mut h = config.init($view_fn, ())?;
                h.render("screenshot")?;
                Ok(())
            }),
            $book_path,
        )
    };
}

fn main() {
    let args = Args::parse();
    let output_dir = args.output_dir.clone();

    let images: Vec<(WorkflowEntry, &'static str)> = vec![
        // quickstart
        book_image!(
            "hello-world",
            "images/hello-world.png",
            |(): &()| views::quickstart::hello_view(),
            bg: Rgb888::BLACK
        ),
        // building-views/stacks
        book_image!("hstack", "building-views/images/hstack.png", |(): &()| {
            views::stacks::hstack()
        }),
        book_image!("zstack", "building-views/images/zstack.png", |(): &()| {
            views::stacks::zstack()
        }),
        book_image!(
            "mixed-stacks",
            "building-views/images/mixed-stacks.png",
            |(): &()| views::stacks::mixed_stacks()
        ),
        // building-views/stack-spacing
        book_image!(
            "vstack-spacing",
            "building-views/images/vstack-spacing.png",
            |(): &()| views::spacing::vstack_spacing()
        ),
        book_image!(
            "spacing-hierarchy",
            "building-views/images/spacing-hierarchy.png",
            |(): &()| views::spacing::spacing_hierarchy()
        ),
        book_image!(
            "spacing-hierarchy-oops",
            "building-views/images/spacing-hierarchy-oops.png",
            |(): &()| views::spacing::spacing_hierarchy_oops()
        ),
        // building-views/separating-views
        book_image!("spacer", "building-views/images/spacer.png", |(): &()| {
            views::separating_views::spacer()
        }),
        // building-views/alignment
        book_image!(
            "hstack-vertical-alignment",
            "building-views/images/hstack-vertical-alignment.png",
            |(): &()| views::alignment::hstack_vertical_alignment()
        ),
        // building-views/mixed-alignment
        book_image!(
            "split-alignment",
            "building-views/images/split-alignment.png",
            |(): &()| views::mixed_alignment::split_alignment()
        ),
        // building-views/conditional-views
        book_image!(
            "if-redacted",
            "building-views/images/if-redacted.png",
            |(): &()| views::conditional_views::if_redacted()
        ),
        book_image!("match-view", "building-views/images/match.png", |(): &(
        )| {
            views::conditional_views::match_view_example()
        }),
        // building-views/collections
        book_image!("foreach", "building-views/images/foreach.png", |(): &()| {
            views::collections::foreach()
        }),
        // building-views/fonts
        book_image!(
            "monospace-fonts",
            "building-views/fonts/images/monospace-fonts.png",
            |(): &()| views::fonts::monospace()
        ),
        book_image!(
            "u8g2-fonts",
            "building-views/fonts/images/u8g2-fonts.png",
            |(): &()| views::fonts::u8g2()
        ),
    ];

    let mut mappings: Vec<(&str, &str)> = Vec::new();
    let mut runner = WorkflowRunner::new(args);
    runner.defaults.background = Rgb888::CSS_DARK_SLATE_GRAY;

    for (entry, book_path) in images {
        mappings.push((entry.name, book_path));
        runner = runner.register(entry);
    }

    println!("Book Image Generator");
    println!("====================");
    println!("Registered {} book images", mappings.len());
    println!();

    runner.run().unwrap();

    println!();
    copy_to_book(&output_dir, &mappings);
}

fn copy_to_book(output_dir: &Path, mappings: &[(&str, &str)]) {
    let book_src = Path::new(env!("CARGO_MANIFEST_DIR")).join("../docs/book/src");

    println!("Copying images to book source...");
    let mut copied = 0;
    for (workflow_name, book_path) in mappings {
        let src = output_dir
            .join(workflow_name)
            .join("images")
            .join("001_screenshot.png");

        if !src.exists() {
            continue;
        }

        let target = book_src.join(book_path);
        if let Some(parent) = target.parent() {
            std::fs::create_dir_all(parent).unwrap_or_else(|e| {
                eprintln!("Failed to create directory {}: {e}", parent.display());
            });
        }
        if let Err(e) = std::fs::copy(&src, &target) {
            eprintln!(
                "Failed to copy {} -> {}: {e}",
                src.display(),
                target.display()
            );
            continue;
        }
        println!("  {workflow_name} -> {book_path}");
        copied += 1;
    }
    println!("Copied {copied}/{} images", mappings.len());
}
