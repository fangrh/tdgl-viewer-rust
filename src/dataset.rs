use std::path::Path;

#[derive(Debug)]
pub struct JeDataset {
    pub je_value: f64,
    pub run_id: String,
    pub solver: String,
    pub completed_steps: usize,
}

impl JeDataset {
    pub fn load(path: &Path) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        // For now, load only metadata (attrs) to keep it simple
        // Full dataset loading will be added when integrating with viewer rendering
        let file = hdf5::File::open(path)?;
        let je_value: f64 = file.attr("Je")?.read_scalar()?;

        // Read string attributes as fixed-size arrays then convert to String
        let run_id_bytes: [u8; 64] = file.attr("run_id")?.read_scalar()?;
        let run_id = String::from_utf8(run_id_bytes.to_vec())
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?
            .trim_end_matches('\0')
            .to_string();

        let solver_bytes: [u8; 64] = file.attr("solver")?.read_scalar()?;
        let solver = String::from_utf8(solver_bytes.to_vec())
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?
            .trim_end_matches('\0')
            .to_string();

        let completed_steps: usize = file.attr("completed_steps")?.read_scalar()?;
        Ok(Self { je_value, run_id, solver, completed_steps })
    }
}
