use std::fs;
use std::path::Path;

pub fn copy_to_dest(sample_path: &Path, dest_root: &Path, category: &str) -> Result<(), String> {
    let file_name = match sample_path.file_name() {
        Some(name) => name,
        None => return Err("Filename not found.".to_string()),
    };

    let dest_dir = dest_root.join(category);
    if !dest_dir.exists() {
        fs::create_dir_all(&dest_dir).map_err(|e| format!("Failed to create folder {:?}: {}", dest_dir, e))?;
    }

    let dest_file = dest_dir.join(file_name);
    if dest_file.exists() {
        println!(
            "Skipping {:?}, file already exists in category '{}'",
            sample_path, category
        );
        return Ok(());
    }

    fs::copy(sample_path, &dest_file)
        .map_err(|e| format!("Failed to copy {:?}: {}", sample_path, e))?;
    println!("Copied {:?} to {}", sample_path, category);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_copy_to_dest_success() {
        let src_dir = tempdir().unwrap();
        let dest_root = tempdir().unwrap();
        let sample_path = src_dir.path().join("test_sample.wav");
        fs::File::create(&sample_path).unwrap().write_all(b"test content").unwrap();

        let category = "test_category";
        copy_to_dest(&sample_path, dest_root.path(), category).unwrap();

        let expected_dest_file = dest_root.path().join(category).join("test_sample.wav");
        assert!(expected_dest_file.exists());
        assert_eq!(fs::read(&expected_dest_file).unwrap(), b"test content");
    }

    #[test]
    fn test_copy_to_dest_directory_creation() {
        let src_dir = tempdir().unwrap();
        let dest_root = tempdir().unwrap();
        let sample_path = src_dir.path().join("test_sample.wav");
        fs::File::create(&sample_path).unwrap();

        let category = "new_category";
        copy_to_dest(&sample_path, dest_root.path(), category).unwrap();

        let expected_dest_dir = dest_root.path().join(category);
        assert!(expected_dest_dir.is_dir());
    }

    #[test]
    fn test_copy_to_dest_file_already_exists() {
        let src_dir = tempdir().unwrap();
        let dest_root = tempdir().unwrap();
        let sample_path = src_dir.path().join("test_sample.wav");
        fs::File::create(&sample_path).unwrap().write_all(b"original content").unwrap();

        let category = "existing_category";
        let dest_dir = dest_root.path().join(category);
        fs::create_dir_all(&dest_dir).unwrap();
        fs::File::create(dest_dir.join("test_sample.wav")).unwrap().write_all(b"existing content").unwrap();

        copy_to_dest(&sample_path, dest_root.path(), category).unwrap();

        let expected_dest_file = dest_root.path().join(category).join("test_sample.wav");
        assert!(expected_dest_file.exists());
        assert_eq!(fs::read(&expected_dest_file).unwrap(), b"existing content");
    }

    #[test]
    fn test_copy_to_dest_no_filename() {
        let dest_root = tempdir().unwrap();
        let sample_path = Path::new("/");

        let result = copy_to_dest(&sample_path, dest_root.path(), "some_category");
        assert!(result.is_err());
        let err_msg = result.unwrap_err();
        assert_eq!(err_msg, "Filename not found.");
        assert!(!dest_root.path().join("some_category").exists());
    }
}
