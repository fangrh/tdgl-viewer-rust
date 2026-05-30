use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Manifest {
    pub run_id: String,
    pub created_at: String,
    pub scan: ScanInfo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanInfo {
    #[serde(rename = "type")]
    pub scan_type: String,
    pub Je_values: Vec<f64>,
    pub completed: Vec<f64>,
    pub current_Je: f64,
    pub status: String,
}

impl Manifest {
    pub fn load(run_dir: &Path) -> Result<Self, Box<dyn std::error::Error>> {
        let manifest_path = run_dir.join("manifest.json");
        let content = std::fs::read_to_string(&manifest_path)?;
        let manifest: Manifest = serde_json::from_str(&content)?;
        Ok(manifest)
    }

    pub fn pending(&self) -> Vec<f64> {
        self.scan.Je_values.iter()
            .filter(|je| !self.scan.completed.iter().any(|v| (v - **je).abs() < 1.0))
            .copied()
            .collect()
    }

    pub fn is_completed(&self, je: f64) -> bool {
        self.scan.completed.iter().any(|v| (v - je).abs() < 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_parse_manifest() {
        let dir = tempfile::tempdir().unwrap();
        let json = r#"{
            "run_id": "test_001",
            "created_at": "2026-05-30T10:00:00Z",
            "scan": {
                "type": "current_sweep",
                "Je_values": [1.0e6, 2.0e6, 3.0e6],
                "completed": [1.0e6],
                "current_Je": 2.0e6,
                "status": "running"
            }
        }"#;
        fs::write(dir.path().join("manifest.json"), json).unwrap();
        let m = Manifest::load(dir.path()).unwrap();
        assert_eq!(m.run_id, "test_001");
        assert_eq!(m.scan.completed, vec![1.0e6]);
        assert_eq!(m.pending(), vec![2.0e6, 3.0e6]);
        assert!(m.is_completed(1.0e6));
        assert!(!m.is_completed(2.0e6));
    }
}
