#![allow(unused_variables)]
// #![allow(unused_imports)]

use pyo3::prelude::*;
use pyo3::{types::PyDict, wrap_pyfunction};

/// A hyper-fast Cache written in Rust
#[pymodule]
fn rscache(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    m.add_wrapped(wrap_pyfunction!(load))?;
    m.add_wrapped(wrap_pyfunction!(sum_as_string))?;
    // m.add_wrapped(wrap_pyfunction!(loads))?;
    // m.add_wrapped(wrap_pyfunction!(dump))?;
    // m.add_wrapped(wrap_pyfunction!(dumps))?;

    Ok(())
}

#[pyfunction]
pub fn load(py: Python, fp: PyObject, kwargs: Option<&PyDict>) -> PyResult<()> {
    Ok(())
}

/// Formats the sum of two numbers as string.
#[pyfunction]
pub fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}
