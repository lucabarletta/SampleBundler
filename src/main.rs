mod config;
mod sample_finder;
mod categorizer;
mod copier;
mod tree_printer;
mod discoverer;
mod utils;

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use std::io;

#[derive(Parser)]
#[command(name = "Sample Organizer", version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Organize {
        #[arg(short, long)]
        source: PathBuf,
        #[arg(short, long)]
        dest: PathBuf,
        #[arg(short, long, default_value = "config.toml")]
        config: PathBuf,
    },

    Tree {
        #[arg(short, long)]
        source: PathBuf,

        #[arg(long)]
        folders_only: bool,

        /// List only unique matched categories (from config)
        #[arg(long)]
        list_categories: bool,

        /// Discover filename similarity patterns in folders
        #[arg(long)]
        run_discover: bool,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Command::Organize {
            source,
            dest,
            config,
        } => {
            // TODO
            let config = match config::load_config(config) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Error loading config: {}", e);
                    return;
                }
            };
            let samples = sample_finder::find_samples(source);
            let mut copied_count = 0;
            let mut uncategorized_count = 0;

            for sample in &samples {
                if let Some(category) = categorizer::categorize_sample(sample, &config) {
                    if let Err(e) = copier::copy_to_dest(sample, dest, &category) {
                        eprintln!("Error copying file: {}", e);
                    }
                    copied_count += 1;
                } else {
                    uncategorized_count += 1;
                }
            }
            println!("-");
            println!("Organization complete.");
            println!("Copied {} files.", copied_count);
            println!("{} files were not categorized.", uncategorized_count);
        }

        Command::Tree {
            source,
            folders_only,
            list_categories,
            run_discover,
        } => {
            if *run_discover {
                discoverer::discover_patterns(&mut std::io::stdout(), source).unwrap();
            } else if *list_categories {
                let config = match config::load_config(&PathBuf::from("config.toml")) {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("Error loading config: {}", e);
                        return;
                    }
                };

                let samples = sample_finder::find_samples(source);
                let mut category_counts: std::collections::HashMap<String, usize> = std::collections::HashMap::new();

                for sample in samples {
                    if let Some(category) = categorizer::categorize_sample(&sample, &config) {
                        *category_counts.entry(category).or_insert(0) += 1;
                    }
                }

                if category_counts.is_empty() {
                    println!("No matching samples found.");
                } else {
                    println!("Matched sample categories:");
                    let mut sorted: Vec<_> = category_counts.into_iter().collect();
                    sorted.sort_by(|a, b| a.0.cmp(&b.0));

                    for (cat, count) in sorted {
                        println!("- {}: {}", cat, count);
                    }
                }
            } else {
                println!("{}", source.display());
                tree_printer::print_tree(&mut io::stdout(), source, "".into(), *folders_only).unwrap();
            }
        }
    }
}
