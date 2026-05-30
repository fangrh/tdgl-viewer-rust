//! tdgl-viewer-rust: Auto-discovery viewer for per-Je TDGL HDF5 files.
//!
//! Provides both a Rust CLI and Python bindings via PyO3.

use pyo3::prelude::*;

pub mod dataset;
pub mod iv_curve;
pub mod manifest;
pub mod scanner;
pub mod timeline;

#[pymodule]
fn tdgl_viewer_rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<scanner::PyScanner>()?;
    m.add_class::<iv_curve::PyIVCurve>()?;
    Ok(())
}
