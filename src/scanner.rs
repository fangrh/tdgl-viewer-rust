use notify::{Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::{BTreeMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::mpsc;

/// A Je file discovered in the run directory.
#[derive(Debug, Clone)]
pub struct JeFile {
    pub path: PathBuf,
    pub je_value: f64,
}

fn je_value_from_filename(name: &str) -> Option<f64> {
    let prefix = "result_Je_";
    let suffix = ".h5";
    if name.starts_with(prefix) && name.ends_with(suffix) {
        let je_str = &name[prefix.len()..name.len() - suffix.len()];
        je_str.parse::<f64>().ok()
    } else {
        None
    }
}

fn je_to_key(je: f64) -> u64 {
    je.to_bits()
}

pub struct Scanner {
    run_dir: PathBuf,
    je_files: BTreeMap<u64, JeFile>,
}

impl Scanner {
    pub fn new(run_dir: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let mut scanner = Scanner {
            run_dir: run_dir.to_path_buf(),
            je_files: BTreeMap::new(),
        };
        scanner.scan_existing()?;
        Ok(scanner)
    }

    pub fn scan_existing(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        for entry in std::fs::read_dir(&self.run_dir)? {
            let entry = entry?;
            let name = entry.file_name().to_string_lossy().to_string();
            if let Some(je) = je_value_from_filename(&name) {
                self.je_files.insert(je_to_key(je), JeFile {
                    path: entry.path(),
                    je_value: je,
                });
            }
        }
        Ok(())
    }

    pub fn je_files_sorted(&self) -> Vec<&JeFile> {
        self.je_files.values().collect()
    }

    pub fn count(&self) -> usize {
        self.je_files.len()
    }

    pub fn je_values(&self) -> Vec<f64> {
        self.je_files.values().map(|f| f.je_value).collect()
    }

    pub fn watch<F: Fn(JeFile) + Send + 'static>(
        &self,
        callback: F,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (tx, rx) = mpsc::channel();
        let mut watcher = RecommendedWatcher::new(
            move |res: Result<Event, notify::Error>| {
                if let Ok(event) = res {
                    let _ = tx.send(event);
                }
            },
            Config::default(),
        )?;
        watcher.watch(&self.run_dir, RecursiveMode::NonRecursive)?;
        let known: HashSet<u64> = self.je_files.keys().copied().collect();
        for event in rx.iter() {
            if let EventKind::Create(_) | EventKind::Modify(_) = event.kind {
                for path in event.paths {
                    let name = path.file_name().unwrap().to_string_lossy().to_string();
                    if let Some(je) = je_value_from_filename(&name) {
                        let key = je_to_key(je);
                        if !known.contains(&key) && path.exists() {
                            callback(JeFile { path, je_value: je });
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_je_value_from_filename() {
        // Support both formats: fixed-point (010.2f) and scientific (.2e)
        assert!(je_value_from_filename("result_Je_1000000.00.h5").is_some());
        assert!(je_value_from_filename("result_Je_1.00e+06.h5").is_some());
        assert_eq!(je_value_from_filename("manifest.json"), None);
        assert_eq!(je_value_from_filename("mesh.h5"), None);
    }

    #[test]
    fn test_scan_existing() {
        let dir = tempfile::tempdir().unwrap();
        std::fs::File::create(dir.path().join("result_Je_1000000.00.h5")).unwrap();
        std::fs::File::create(dir.path().join("result_Je_2000000.00.h5")).unwrap();
        std::fs::File::create(dir.path().join("mesh.h5")).unwrap();
        let scanner = Scanner::new(dir.path()).unwrap();
        assert_eq!(scanner.count(), 2);
    }
}
