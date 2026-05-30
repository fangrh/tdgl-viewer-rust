use crate::dataset::JeDataset;
use crate::scanner::Scanner;

pub struct Timeline {
    pub steps: Vec<(usize, f64, usize)>,  // (global_step, je_value, local_step)
    pub total_frames: usize,
}

impl Timeline {
    pub fn build(scanner: &Scanner) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let files = scanner.je_files_sorted();
        let mut steps = Vec::new();
        let mut global_step = 0;
        for je_file in files {
            let dataset = JeDataset::load(&je_file.path)?;
            for local_step in 0..dataset.completed_steps {
                steps.push((global_step, dataset.je_value, local_step));
                global_step += 1;
            }
        }
        let total_frames = global_step;
        Ok(Timeline { steps, total_frames })
    }

    pub fn je_at_frame(&self, frame: usize) -> Option<f64> {
        self.steps.get(frame).map(|(_, je, _)| *je)
    }

    pub fn local_step_at_frame(&self, frame: usize) -> Option<(f64, usize)> {
        self.steps.get(frame).map(|(_, je, local)| (*je, *local))
    }
}
