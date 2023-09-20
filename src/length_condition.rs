use std::env;

use pyo3::prelude::*;
use pyo3::exceptions::{PyBaseException, PyValueError};
use pyo3::ffi::lenfunc;
use pyo3::PyTypeInfo;
use pyo3::types::PyType;

use crate::helper;

pyo3::create_exception!(mymodule, LengthError, PyBaseException);

#[pyfunction]
pub fn check_length(_py: Python,
                    item: &PyAny,
                    expected_length: &PyAny,
                    handle_with: Option<&PyType>,
                    message: Option<&str>,
                    operator: Option<PyObject>,
                    assign_length_to_others: Option<bool>) -> PyResult<()> {
    
    if item.hasattr("__len__").is_ok() {
        match item.call_method0("__len__") {
            Ok(result) => {
                match operator {
                    None => {
                        if result.eq(expected_length).is_ok() {
                            Ok(())
                        } else {
                            Err(helper::check_handle_with(Some(_py.get_type::<LengthError>()), message))
                        }
                    }
                    Some(op) => {
                        match op.call_method(_py, "__call__", (result, expected_length), None) {
                            Ok(_) => {Ok(())}
                            Err(_) => {
                                Err(helper::check_handle_with(Some(_py.get_type::<LengthError>()), message))
                            }
                        }
                    }
                }
            }
            Err(_) => {
                Ok(())
            }
        }
    } else {
        Err(PyValueError::new_err("example"))
    }

}
