use pyo3::exceptions::{PyAssertionError, PyUserWarning, PyWarning};
use pyo3::{PyErr};
use pyo3::types::PyType;

pub fn check_handle_with(handle_with: Option<&PyType>, message: Option<&str>) -> PyErr {
    match handle_with {
        None => {
            match message {
                None => {
                    PyAssertionError::new_err("")
                }
                Some(msg) => {
                    PyAssertionError::new_err(msg.to_string())
                }
            }
        }
        Some(exception) => {
            match message {
                None => {
                    PyErr::from_type(exception, "")
                }
                Some(msg) => {
                    PyErr::from_type(exception, msg.to_string())
                }
            }
        }
    }
}

pub fn check_handle_warning_with(handle_with: Option<&PyType>, message: Option<&str>) -> PyErr {
    match handle_with {
        None => {
            check_handle_with(handle_with, message)
        }
        Some(err) => {
            if err.is_exact_instance_of::<PyUserWarning>() {
                let msg = message.unwrap_or("").to_string();
                PyUserWarning::new_err(msg)
            } else if err.is_exact_instance_of::<PyWarning>() {
                let msg = message.unwrap_or("").to_string();
                PyWarning::new_err(msg)
            } else {
                check_handle_with(handle_with, message)
            }
        }
    }
}
