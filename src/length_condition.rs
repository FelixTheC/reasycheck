use std::any::{Any, TypeId};
use pyo3::prelude::*;
use pyo3::exceptions::{PyAttributeError, PyBaseException, PyTypeError};
use pyo3::ffi::PyTypeObject;
use pyo3::PyTypeInfo;
use pyo3::types::{PyList, PyType};

use crate::helper;

pyo3::create_exception!(mymodule, LengthError, PyBaseException);

fn raise_length_error_if(_py: Python, handle_with: Option<&PyType>, message: Option<&str>) -> Result<(), PyErr> {
    match handle_with {
        None => { Err(helper::check_handle_with(Some(_py.get_type::<LengthError>()), message)) }
        Some(_) => { Err(helper::check_handle_with(handle_with, message)) }
    }
}

#[pyfunction]
pub fn check_length(_py: Python,
                    item: &PyAny,
                    expected_length: &PyAny,
                    handle_with: Option<&PyType>,
                    message: Option<&str>,
                    operator: Option<PyObject>,
                    assign_length_to_others: Option<bool>) -> PyResult<()> {

    match item.call_method0("__len__") {
        Ok(result) => {
            match operator {
                None => {
                    if result.eq(expected_length).unwrap_or(false) {
                        Ok(())
                    } else {
                        raise_length_error_if(_py, handle_with, message)
                    }
                }
                Some(op) => {
                    if !op.getattr(_py, "__call__").is_ok() {
                        let val: &PyAny = op.downcast(_py)?;
                        Err(PyTypeError::new_err(format!("'{}' object is not callable", val.get_type().name().unwrap_or(""))))
                    } else {
                        match op.call_method(_py, "__call__", (result, expected_length), None) {
                            Ok(_) => {Ok(())}
                            Err(_) => {
                                raise_length_error_if(_py, handle_with, message)
                            }
                        }
                    }

                }
            }
        }
        Err(_) => {
            if assign_length_to_others.unwrap_or(false) {
                let pylist = PyList::new(_py, &[item]);
                if pylist.call_method0("__len__").unwrap().eq(expected_length).unwrap_or(false) {
                    Ok(())
                } else {
                    raise_length_error_if(_py, handle_with, message)
                }
            } else {
                Err(PyAttributeError::new_err(item.to_string() + " does not support __len__."))
            }
        }
    }

}
