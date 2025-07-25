use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn find_samples(source: &Path) -> Vec<PathBuf> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_find_samples_basic() {
        let dir = tempdir().unwrap();
        fs::File::create(dir.path().join("sample1.wav")).unwrap();
        fs::File::create(dir.path().join("sample2.mp3")).unwrap();
        fs::create_dir(dir.path().join("subdir")).unwrap();
        fs::File::create(dir.path().join("subdir/sample3.wav")).unwrap();

        let samples = find_samples(dir.path());
        assert_eq!(samples.len(), 2);
        assert!(samples.contains(&dir.path().join("sample1.wav")));
        assert!(samples.contains(&dir.path().join("subdir/sample3.wav")));
        assert!(!samples.contains(&dir.path().join("sample2.mp3")));
    }

    #[test]
    fn test_find_samples_empty_dir() {
        let dir = tempdir().unwrap();
        let samples = find_samples(dir.path());
        assert!(samples.is_empty());
    }

    #[test]
    fn test_find_samples_no_wav_files() {
        let dir = tempdir().unwrap();
        fs::File::create(dir.path().join("sample1.mp3")).unwrap();
        fs::File::create(dir.path().join("sample2.aiff")).unwrap();

        let samples = find_samples(dir.path());
        assert!(samples.is_empty());
    }

    #[test]
    fn test_find_samples_nested_dirs() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("a/b/c")).unwrap();
        fs::File::create(dir.path().join("a/sample_a.wav")).unwrap();
        fs::File::create(dir.path().join("a/b/sample_b.wav")).unwrap();
        fs::File::create(dir.path().join("a/b/c/sample_c.wav")).unwrap();

        let samples = find_samples(dir.path());
        assert_eq!(samples.len(), 3);
        assert!(samples.contains(&dir.path().join("a/sample_a.wav")));
        assert!(samples.contains(&dir.path().join("a/b/sample_b.wav")));
        assert!(samples.contains(&dir.path().join("a/b/c/sample_c.wav")));
    }
}