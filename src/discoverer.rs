use natord::compare;
use std::collections::HashMap;
use std::collections::HashSet;
use std::path::Path;
use walkdir::WalkDir;
use crate::utils::longest_common_prefix;
use std::io::{self, Write};

pub fn discover_patterns<W: Write>(writer: &mut W, source: &Path) -> io::Result<()> {
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

    for key in sorted_keys {
        let files = temp_map.remove(&key).unwrap_or_default();
        writeln!(writer, "{}", key)?;

        let mut ungrouped = files.clone();
        let mut seen = HashSet::new();

        while !ungrouped.is_empty() {
            let base = ungrouped.remove(0);
            let mut group = vec![base.clone()];
            let mut current_lcp_for_group = base.clone();

            let mut i = 0;
            while i < ungrouped.len() {
                let candidate = &ungrouped[i];
                let potential_new_lcp = longest_common_prefix(&[current_lcp_for_group.clone(), candidate.clone()]);

                if potential_new_lcp.len() >= 4 {
                    group.push(ungrouped.remove(i));
                    current_lcp_for_group = potential_new_lcp;
                } else {
                    i += 1;
                }
            }

            let final_lcp = longest_common_prefix(&group);
            if final_lcp.is_empty() || seen.contains(&final_lcp.to_lowercase()) {
                continue;
            }

            seen.insert(final_lcp.to_lowercase());
            writeln!(
                writer,
                // TODO format printer
                "- pattern like '{}*': {} files",
                final_lcp,
                group.len()
            )?;
        }

        writeln!(writer)?;
    }
    Ok(())
}
