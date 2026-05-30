use rayon::prelude::*;

use crate::dataset::JeDataset;
use crate::scanner::Scanner;

#[derive(Debug, Clone)]
pub struct IVPoint {
    pub je_applied: f64,
    pub voltage: f64,
    pub resistance: f64,
}

pub struct IVCurve {
    pub points: Vec<IVPoint>,
}

impl IVCurve {
    pub fn compute(scanner: &Scanner) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let files = scanner.je_files_sorted();
        let points: Vec<IVPoint> = files
            .par_iter()
            .map(|je_file| {
                let dataset = JeDataset::load(&je_file.path)?;
                // Voltage placeholder — will be computed from electric_field data
                let voltage = 0.0;
                let resistance = if dataset.je_value.abs() > 0.0 {
                    voltage / dataset.je_value
                } else {
                    0.0
                };
                Ok(IVPoint { je_applied: dataset.je_value, voltage, resistance })
            })
            .collect::<Result<Vec<_>, Box<dyn std::error::Error + Send + Sync>>>()?;
        let mut points = points;
        points.sort_by(|a, b| a.je_applied.partial_cmp(&b.je_applied).unwrap());
        Ok(IVCurve { points })
    }

    pub fn arrays(&self) -> (Vec<f64>, Vec<f64>) {
        let je: Vec<f64> = self.points.iter().map(|p| p.je_applied).collect();
        let v: Vec<f64> = self.points.iter().map(|p| p.voltage).collect();
        (je, v)
    }
}
