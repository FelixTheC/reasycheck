use std::any::Any;
use std::env;
use pyo3::{AsPyPointer, IntoPy, pyfunction, PyObject, PyResult, Python, PyTypeInfo};
use pyo3::exceptions::PyTypeError;
use pyo3::ffi::PyObject_IsInstance;
use pyo3::types::{PyList, PySet, PyType};
use crate::helper;


#[pyfunction]
#[pyo3(signature = (item, expected_type, /, handle_with=None, message=""))]
pub unsafe fn check_type(_py: Python, item: PyObject, expected_type: PyObject, handle_with: Option<&PyType>, message: Option<&str>) -> PyResult<()> {
    let is_disabled = env::var("EASYCHECK_RUN").unwrap_or("1".parse()?) == "0";

    if is_disabled {
        return Ok(());
    }

    if PyObject_IsInstance(expected_type.as_ptr(), PyList::type_object(_py).as_ptr()).is_positive() {
        let list_obj: &PyList = expected_type.downcast(_py).unwrap();

        for obj in list_obj {
            let res = PyObject_IsInstance(item.as_ptr(), obj.as_ptr()).is_positive();
            if res {
                return Ok(());
            }
        }
    } else if PyObject_IsInstance(expected_type.as_ptr(), PySet::type_object(_py).as_ptr()).is_positive() {
        let set_obj: &PySet = expected_type.downcast(_py).unwrap();

        for obj in set_obj {
            let res = PyObject_IsInstance(item.as_ptr(), obj.as_ptr()).is_positive();
            if res {
                return Ok(());
            }
        }
    } else {
        if PyObject_IsInstance(item.as_ptr(), expected_type.as_ptr()).is_positive() {
            return Ok(());
        }
    }

    match handle_with {
        None => {
            match message {
                None => {Err(PyTypeError::new_err(""))}
                Some(msg) => {Err(PyTypeError::new_err(msg.to_string()))}
            }
        }
        Some(_) => {
            Err(helper::check_handle_warning_with(handle_with, message))
        }
    }
}