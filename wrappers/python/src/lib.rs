use pyo3::prelude::*;
//TODO: find a better exception to use here
use pyo3::exceptions::PyBaseException;
use ::jawohl as jw;


/// Export complete_json to Python.
#[pyfunction]
fn complete_json(input: &str) -> PyResult<String> {
    match jw::complete_json(input) {
        Ok(s) => Ok(s),
        Err(e) => Err(PyErr::new::<PyBaseException, _>(format!("{}", e))),
    }
} 

/// Export get_closing_string_for_partial_json to Python.
#[pyfunction]
fn get_closing_string_for_partial_json(input: &str) -> PyResult<String> {
    match jw::get_closing_string_for_partial_json(input) {
        Ok(s) => Ok(s),
        Err(e) => Err(PyErr::new::<PyBaseException, _>(format!("{}", e))),
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn jawohl(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(complete_json, m)?)?;
    m.add_function(wrap_pyfunction!(get_closing_string_for_partial_json, m)?)?;
    Ok(())
}