use std::env;
use pyo3::prelude::*;
use pyo3::{Py, pyfunction, PyResult, Python};
use pyo3::types::{IntoPyDict, PyBool, PyFloat, PyType};
use pyo3::exceptions::{PyAssertionError, PyBaseException};
use crate::helper;

pyo3::create_exception!(reasycheck, NotCloseEnoughError, PyBaseException);

fn raise_not_close_enough(_py: Python, handle_with: Option<&PyType>, message: Option<&str>) -> Result<(), PyErr> {
    match handle_with {
        None => { Err(helper::check_handle_warning_with(Some(_py.get_type::<NotCloseEnoughError>()), message)) }
        Some(_) => { Err(helper::check_handle_warning_with(handle_with, message)) }
    }
}

///    Check if two floats are close in value.
///
///    The function is just a wrapper around math.isclose(), and its defaults
///    are exactly the same. Two values (x and y, both being positional-only
///    parameters) will be considered close when the difference between them
///    (either relative or absolute) is smaller than at least one of the
///    tolerances. If you do not want to use any of the two tolerances, set it
///    to 0.
///
///    Note: Before applying math.isclose(), x and y are first converted to
///    floats, so you can provide them as integers or even strings.
///
///    At least one tolerance needs to be provided (so not be zero); otherwise
///    the function will do nothing.
///
///    Unlike most easycheck functions, check_if_isclose() uses two
///    positional-only and four keyword-only arguments. So when providing one of
///    the two tolerances, you have to specify it using the argument's name. You
///    have to do the same also for handle_with and message.
///
///    Args:
///        x, y (float): two numbers to compare
///        rel_tol (float): maximum difference for being considered "close",
///            relative to the magnitude of the input values
///        abs_tol (float): maximum difference for being considered "close",
///            regardless of the magnitude of the input values
///        handle_with (type): the type of exception or warning to be raised
///        message (str): a text to use as the exception/warning message.
///            Defaults to None, which means using no message for built-in
///            exceptions/warnings, and the docstrings of the exception/warning
///            class as a message for custom exceptions.
///
///    Returns:
///        None, if check succeeded.
///
///    Raises:
///        Exception of the type provided by the handle_with parameter,
///        NotCloseEnoughError by default.
#[pyfunction]
#[pyo3(signature = (x, y, /, *, handle_with=None, message="", rel_tol=0.0000000001_f64, abs_tol=0.0_f64))]
pub fn check_if_isclose(_py: Python,
                        x: Py<PyFloat>,
                        y: Py<PyFloat>,
                        handle_with: Option<&PyType>,
                        message: Option<&str>,
                        rel_tol: Option<f64>,
                        abs_tol: Option<f64>) -> PyResult<()> {
    let is_disabled = env::var("EASYCHECK_RUN").unwrap_or("1".parse()?) == "0";

    if is_disabled {
        return Ok(());
    }

    let key_rel_tol = "rel_tol";
    let val_rel_tol = rel_tol.unwrap();
    let key_abs_tol = "abs_tol";
    let val_abs_tol = abs_tol.unwrap();

    match _py.import("math") {
        Ok(module) => {
            let kwargs = [(key_rel_tol, val_rel_tol), (key_abs_tol, val_abs_tol)].into_py_dict(_py);

            match module.call_method("isclose", (x, y), Some(kwargs)) {
                Ok(py_bool) => {
                    match py_bool.downcast::<PyBool>() {
                        Ok(res) => {
                            if res.is_true() {
                                Ok(())
                            } else {
                                raise_not_close_enough(_py, handle_with, message)
                            }
                        }
                        Err(err) => {Err(PyErr::from(err)) }
                    }
                }
                Err(err) => {Err(err)}
            }
        }
        Err(err) => {Err(err)}
    }
}

#[pyfunction]
#[pyo3(signature = (x, y, /, *, handle_with=None, message="", rel_tol=0.0000000001_f64, abs_tol=0.0_f64))]
pub fn assert_if_isclose(_py: Python,
                        x: Py<PyFloat>,
                        y: Py<PyFloat>,
                        handle_with: Option<&PyType>,
                        message: Option<&str>,
                        rel_tol: Option<f64>,
                        abs_tol: Option<f64>) -> PyResult<()> {
    check_if_isclose(_py, x, y, Option::from(PyAssertionError::new_err(()).get_type(_py)), message, rel_tol, abs_tol)
}