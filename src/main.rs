use clap::{Parser, Subcommand};
use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Parser)]
#[command(name = "Sample Organizer", version)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Organize samples by regex pattern into folders
    Organize {
        #[arg(short, long)]
        source: PathBuf,
        #[arg(short, long)]
        dest: PathBuf,
        #[arg(short, long, default_value = "config.toml")]
        config: PathBuf,
    },

    /// Print folder structure of the source directory
    Tree {
        #[arg(short, long)]
        source: PathBuf,

        /// Only show folders, omit files
        #[arg(long)]
        folders_only: bool,
    },
}

#[derive(Debug, Deserialize)]
struct Config {
    patterns: HashMap<String, Vec<String>>,
}

fn load_config(path: &Path) -> Config {
    let content = fs::read_to_string(path).expect("Failed to read config file");
    toml::from_str(&content).expect("Failed to parse config")
}

fn find_samples(source: &Path) -> Vec<PathBuf> {
    WalkDir::new(source)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().map(|ext| ext.eq_ignore_ascii_case("wav")).unwrap_or(false))
        .map(|e| e.path().to_path_buf())
        .collect()
}

fn categorize_sample(path: &Path, config: &Config) -> Option<String> {
    let filename = path.file_name()?.to_str()?.to_lowercase();

    for (category, patterns) in &config.patterns {
        for pattern in patterns {
            if let Ok(re) = Regex::new(pattern) {
                if re.is_match(&filename) {
                    return Some(category.clone());
                }
            }
        }
    }

    None
}

fn copy_to_dest(sample_path: &Path, dest_root: &Path, category: &str) {
    let dest_dir = dest_root.join(category);
    if let Err(e) = fs::create_dir_all(&dest_dir) {
        eprintln!("Failed to create folder {:?}: {}", dest_dir, e);
        return;
    }

    if let Some(file_name) = sample_path.file_name() {
        let dest_file = dest_dir.join(file_name);
        if let Err(e) = fs::copy(sample_path, &dest_file) {
            eprintln!("Failed to copy {:?}: {}", sample_path, e);
        }
    }
}

fn print_tree(dir: &Path, indent: String, folders_only: bool) {
    if let Ok(entries) = fs::read_dir(dir) {
        let mut entries = entries.flatten().collect::<Vec<_>>();
        entries.sort_by_key(|e| e.path());

        for (i, entry) in entries.iter().enumerate() {
            let path = entry.path();
            let is_dir = path.is_dir();
            let is_last = i == entries.len() - 1;

            if folders_only && !is_dir {
                continue;
            }

            let prefix = if is_last { "└── " } else { "├── " };
            println!("{}{}{}", indent, prefix, path.file_name().unwrap().to_string_lossy());

            if is_dir {
                let new_indent = indent.clone() + if is_last { "    " } else { "│   " };
                print_tree(&path, new_indent, folders_only);
            }
        }
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Command::Organize {
            source,
            dest,
            config,
        } => {
            let config = load_config(config);
            let samples = find_samples(source);

            for sample in samples {
                if let Some(category) = categorize_sample(&sample, &config) {
                    copy_to_dest(&sample, dest, &category);
                    println!("Copied {:?} to {}", sample, category);
                }
            }
        }

        Command::Tree {
            source,
            folders_only,
        } => {
            println!("{}", source.display());
            print_tree(source, "".into(), *folders_only);
        }
    }
}
