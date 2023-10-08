use std::env;

use pyo3::prelude::*;
use pyo3::exceptions::{PyAssertionError, PyBaseException};
use pyo3::types::PyType;

use crate::helper;

pyo3::create_exception!(reasycheck, LimitError, PyBaseException);

#[pyfunction]
pub fn check_if_in_limits(_py: Python,
                          x: f64,
                          lower_limit: Option<f64>,
                          upper_limit: Option<f64>,
                          handle_with: Option<&PyType>,
                          message: Option<&str>,
                          include_equal: Option<bool>) -> PyResult<()> {
    let is_disabled = env::var("EASYCHECK_RUN").unwrap_or("1".parse()?) == "0";

    if is_disabled {
        return Ok(());
    }

    let mut throw_error = true;
    let llimit = lower_limit.unwrap_or(f64::NEG_INFINITY);
    let ulimit = upper_limit.unwrap_or(f64::INFINITY);

    if include_equal.unwrap_or(true) {
        throw_error = !((llimit <= x) && (x <= ulimit));
    } else {
        throw_error = !((llimit < x) && (x < ulimit));
    }

    if throw_error {
        match handle_with {
            None => {
                Err(helper::check_handle_with(Some(_py.get_type::<LimitError>()), message))
            }
            Some(_) => {
                Err(helper::check_handle_with(handle_with, message))
            }
        }

    } else {
        Ok(())
    }
}

#[pyfunction]
pub fn assert_if_in_limits(_py: Python,
                           x: f64,
                           lower_limit: Option<f64>,
                           upper_limit: Option<f64>,
                           handle_with: Option<&PyType>,
                           message: Option<&str>,
                           include_equal: Option<bool>) -> PyResult<()> {
    check_if_in_limits(_py, x, lower_limit, upper_limit, Option::from(PyAssertionError::new_err(()).get_type(_py)), message, include_equal)
}