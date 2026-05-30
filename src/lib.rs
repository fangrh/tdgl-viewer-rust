//! tdgl-viewer-rust: Auto-discovery viewer for per-Je TDGL HDF5 files.
//!
//! Provides both a Rust CLI and Python bindings via PyO3.

use pyo3::prelude::*;

pub mod dataset;
pub mod iv_curve;
pub mod manifest;
pub mod scanner;
pub mod timeline;

/// Python-accessible scanner for watching run directories.
#[pyclass]
pub struct PyScanner {
    inner: scanner::Scanner,
}

#[pymethods]
impl PyScanner {
    #[new]
    fn new(run_dir: &str) -> PyResult<Self> {
        let inner = scanner::Scanner::new(std::path::Path::new(run_dir))
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        Ok(PyScanner { inner })
    }
    fn je_values(&self) -> Vec<f64> { self.inner.je_values() }
    fn count(&self) -> usize { self.inner.count() }
}

#[pyclass]
pub struct PyIVCurve {
    je: Vec<f64>,
    voltage: Vec<f64>,
    resistance: Vec<f64>,
}

#[pymethods]
impl PyIVCurve {
    #[staticmethod]
    fn compute(run_dir: &str) -> PyResult<Self> {
        let sc = scanner::Scanner::new(std::path::Path::new(run_dir))
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        let iv = iv_curve::IVCurve::compute(&sc)
            .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        Ok(PyIVCurve {
            je: iv.points.iter().map(|p| p.je_applied).collect(),
            voltage: iv.points.iter().map(|p| p.voltage).collect(),
            resistance: iv.points.iter().map(|p| p.resistance).collect(),
        })
    }
    fn je(&self) -> Vec<f64> { self.je.clone() }
    fn voltage(&self) -> Vec<f64> { self.voltage.clone() }
    fn resistance(&self) -> Vec<f64> { self.resistance.clone() }
}

#[pymodule]
fn tdgl_viewer_rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyScanner>()?;
    m.add_class::<PyIVCurve>()?;
    Ok(())
}
