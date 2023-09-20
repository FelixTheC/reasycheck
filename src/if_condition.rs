use std::env;
use pyo3::callback::IntoPyCallbackOutput;

use pyo3::prelude::*;
use pyo3::PyTypeInfo;
use pyo3::types::{PyBool, PyType};

use crate::helper;

#[pyfunction]
pub fn check_if(_py: Python, condition: Py<PyBool>, handle_with: Option<&PyType>, message: Option<&str>) -> PyResult<()> {
    let is_disabled = env::var("EASYCHECK_RUN").unwrap_or("1".parse()?) == "0";

    if is_disabled {
        return Ok(());
    }

    match condition.is_true(_py) {
        Ok(val) => {
            if !val {
                Err(helper::check_handle_warning_with(handle_with, message))
            } else { Ok(()) }
        }
        Err(_) => {
            println!("Error");
            Ok(())
        }
    }
}

#[pyfunction]
pub fn assert_if(_py: Python, condition: Py<PyBool>, handle_with: Option<&PyType>, message: Option<&str>) -> PyResult<()> {
    check_if(_py, condition, handle_with, message)
}

