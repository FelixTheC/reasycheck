use std::env;
use pyo3::prelude::*;
use pyo3::{Py, pyfunction, PyResult, Python};
use pyo3::types::{IntoPyDict, PyBool, PyFloat, PyType};
use crate::helper;

pyo3::create_exception!(reasycheck, NotCloseEnoughError, PyBaseException);

fn raise_not_close_enough(_py: Python, handle_with: Option<&PyType>, message: Option<&str>) -> Result<(), PyErr> {
    match handle_with {
        None => { Err(helper::check_handle_warning_with(Some(_py.get_type::<NotCloseEnoughError>()), message)) }
        Some(_) => { Err(helper::check_handle_warning_with(handle_with, message)) }
    }
}

#[pyfunction]
pub fn check_if_isclose(_py: Python,
                        x: Py<PyFloat>,
                        y: Py<PyFloat>,
                        handle_with: Option<&PyType>,
                        message: Option<&str>,
                        rel_tol: Option<Py<PyFloat>>,
                        abs_tol: Option<Py<PyFloat>>) -> PyResult<()> {
    let is_disabled = env::var("EASYCHECK_RUN").unwrap_or("1".parse()?) == "0";

    if is_disabled {
        return Ok(());
    }

    let key_rel_tol = "rel_tol";
    let val_rel_tol = rel_tol.unwrap_or(Py::from(PyFloat::new(_py, 0.0000000001)));
    let key_abs_tol = "abs_tol";
    let val_abs_tol = abs_tol.unwrap_or(Py::from(PyFloat::new(_py, 0.0)));

    match _py.import("math") {
        Ok(module) => {
            let kwargs = [(key_rel_tol, val_rel_tol), (key_abs_tol, val_abs_tol)].into_py_dict(_py);

            match module.call_method("isclose", (x, y), Some(kwargs)) {
                Ok(_) => {Ok(())}
                Err(_) => {raise_not_close_enough(_py, handle_with, message)}
            }
        }
        Err(_) => {
            Err(helper::check_handle_warning_with(handle_with, message))
        }
    }
}

#[pyfunction]
pub fn assert_if_isclose(_py: Python,
                        x: Py<PyFloat>,
                        y: Py<PyFloat>,
                        handle_with: Option<&PyType>,
                        message: Option<&str>,
                        rel_tol: Option<Py<PyFloat>>,
                        abs_tol: Option<Py<PyFloat>>) -> PyResult<()> {
    check_if_isclose(_py, x, y, handle_with, message, rel_tol, abs_tol)
}