use natord::compare;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

pub fn print_tree<W: Write>(writer: &mut W, dir: &Path, indent: String, folders_only: bool) -> io::Result<()> {
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
            writeln!(
                writer,
                "{}{}{}",
                indent,
                prefix,
                path.file_name().unwrap().to_string_lossy()
            )?;

            if is_dir {
                let new_indent = indent.clone() + if is_last { "    " } else { "│   " };
                print_tree(writer, &path, new_indent, folders_only)?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_print_tree_basic() {
        let dir = tempdir().unwrap();
        fs::File::create(dir.path().join("file1.txt")).unwrap();
        fs::create_dir(dir.path().join("subdir")).unwrap();
        fs::File::create(dir.path().join("subdir/file2.txt")).unwrap();

        let mut buffer = Vec::new();
        print_tree(&mut buffer, dir.path(), "".to_string(), false).unwrap();
        let output = String::from_utf8(buffer).unwrap();

        let expected_output = "├── file1.txt\n└── subdir\n    └── file2.txt\n".to_string();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_print_tree_folders_only() {
        let dir = tempdir().unwrap();
        fs::File::create(dir.path().join("file1.txt")).unwrap();
        fs::create_dir(dir.path().join("subdir")).unwrap();
        fs::File::create(dir.path().join("subdir/file2.txt")).unwrap();

        let mut buffer = Vec::new();
        print_tree(&mut buffer, dir.path(), "".to_string(), true).unwrap();
        let output = String::from_utf8(buffer).unwrap();

        let expected_output = "└── subdir\n".to_string();
        assert_eq!(output, expected_output);
    }

    #[test]
    fn test_print_tree_empty_dir() {
        let dir = tempdir().unwrap();
        let mut buffer = Vec::new();
        print_tree(&mut buffer, dir.path(), "".to_string(), false).unwrap();
        let output = String::from_utf8(buffer).unwrap();
        assert_eq!(output, "");
    }
}
