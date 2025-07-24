use clap::{Parser, Subcommand};
use natord::compare;
use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use std::collections::HashSet;
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

#[derive(Debug, Deserialize)]
struct Config {
    patterns: HashMap<String, Vec<String>>,
}

fn load_config(path: &Path) -> Result<Config, String> {
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    toml::from_str(&content).map_err(|e| e.to_string())
}

fn find_samples(source: &Path) -> Vec<PathBuf> {
    WalkDir::new(source)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| {
            e.path()
                .extension()
                .map(|ext| ext.eq_ignore_ascii_case("wav"))
                .unwrap_or(false)
        })
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
    if !dest_dir.exists() {
        if let Err(e) = fs::create_dir_all(&dest_dir) {
            eprintln!("Failed to create folder {:?}: {}", dest_dir, e);
            return;
        }
    }

    if let Some(file_name) = sample_path.file_name() {
        let dest_file = dest_dir.join(file_name);
        if dest_file.exists() {
            println!(
                "Skipping {:?}, file already exists in category '{}'",
                sample_path, category
            );
            return;
        }

        match fs::copy(sample_path, &dest_file) {
            Ok(_) => println!("Copied {:?} to {}", sample_path, category),
            Err(e) => eprintln!("Failed to copy {:?}: {}", sample_path, e),
        }
    }
}

fn print_tree(dir: &Path, indent: String, folders_only: bool) {
    if let Ok(entries) = fs::read_dir(dir) {
        let mut entries = entries.flatten().collect::<Vec<_>>();
        entries.sort_by(|a, b| compare(&a.path().to_string_lossy(), &b.path().to_string_lossy()));

        for (i, entry) in entries.iter().enumerate() {
            let path = entry.path();
            let is_dir = path.is_dir();
            let is_last = i == entries.len() - 1;

            if folders_only && !is_dir {
                continue;
            }

            let prefix = if is_last { "└── " } else { "├── " };
            println!(
                "{}{}{}",
                indent,
                prefix,
                path.file_name().unwrap().to_string_lossy()
            );

            if is_dir {
                let new_indent = indent.clone() + if is_last { "    " } else { "│   " };
                print_tree(&path, new_indent, folders_only);
            }
        }
    }
}

fn extract_token_prefix(group: &[String], re: &Regex) -> String {
    let best_prefix = longest_common_prefix(group);

    if let Some(mat) = re.find(&best_prefix) {
        best_prefix[mat.start()..mat.end()].to_string()
    } else {
        best_prefix
    }
}

fn discover_patterns(source: &Path) {
    let mut temp_map: HashMap<String, Vec<String>> = HashMap::new();

    for entry in WalkDir::new(source).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            if let Some(ext) = path.extension() {
                if ext.eq_ignore_ascii_case("wav") {
                    if let Some(file_name) = path.file_stem().and_then(|s| s.to_str()) {
                        let folder = path
                            .parent()
                            .unwrap_or_else(|| Path::new("."))
                            .display()
                            .to_string();
                        temp_map
                            .entry(folder)
                            .or_default()
                            .push(file_name.to_string());
                    }
                }
            }
        }
    }

    let mut sorted_keys: Vec<_> = temp_map.keys().cloned().collect();
    sorted_keys.sort_by(|a, b| compare(a, b));

    let prefix_re = Regex::new(r"^[A-Za-z]+(?:[_\- ]?[A-Za-z]+)*").unwrap();

    for key in sorted_keys {
        let files = temp_map.remove(&key).unwrap_or_default();
        println!("{}", key);

        let mut ungrouped = files.clone();
        let mut seen = HashSet::new();

        while !ungrouped.is_empty() {
            let base = ungrouped.remove(0);
            let mut group = vec![base.clone()];

            let mut i = 0;
            while i < ungrouped.len() {
                let candidate = &ungrouped[i];
                let prefix = longest_common_prefix(&[base.clone(), candidate.clone()]);
                if candidate.starts_with(&prefix) && prefix.len() >= 4 {
                    group.push(ungrouped.remove(i));
                } else {
                    i += 1;
                }
            }

            let refined_prefix = extract_token_prefix(&group, &prefix_re);
            if refined_prefix.is_empty() || seen.contains(&refined_prefix.to_lowercase()) {
                continue;
            }

            seen.insert(refined_prefix.to_lowercase());
            println!(
                "- pattern like '{}*': {} files",
                refined_prefix,
                group.len()
            );
        }

        println!();
    }
}

fn longest_common_prefix(strings: &[String]) -> String {
    if strings.is_empty() {
        return "".to_string();
    }

    let mut prefix = strings[0].clone();
    for s in strings.iter().skip(1) {
        while !s.starts_with(&prefix) {
            if prefix.is_empty() {
                return "".to_string();
            }
            prefix.pop();
        }
    }
    prefix
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Command::Organize {
            source,
            dest,
            config,
        } => {
            let config = match load_config(config) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Error loading config: {}", e);
                    return;
                }
            };
            let samples = find_samples(source);
            let mut copied_count = 0;
            let mut uncategorized_count = 0;

            for sample in &samples {
                if let Some(category) = categorize_sample(sample, &config) {
                    copy_to_dest(sample, dest, &category);
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
                discover_patterns(source);
            } else if *list_categories {
                let config = match load_config(&PathBuf::from("config.toml")) {
                    Ok(c) => c,
                    Err(e) => {
                        eprintln!("Error loading config: {}", e);
                        return;
                    }
                };

                let samples = find_samples(source);
                let mut category_counts: HashMap<String, usize> = HashMap::new();

                for sample in samples {
                    if let Some(category) = categorize_sample(&sample, &config) {
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
                print_tree(source, "".into(), *folders_only);
            }
        }
    }
}
