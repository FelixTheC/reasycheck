mod if_condition;
mod if_not_condition;
mod helper;
mod limit_condition;
mod length_condition;
mod is_close_condition;

use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn reasycheck(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(if_condition::check_if, m)?)?;
    m.add_function(wrap_pyfunction!(if_condition::assert_if, m)?)?;
    m.add_function(wrap_pyfunction!(if_not_condition::check_if_not, m)?)?;
    m.add_function(wrap_pyfunction!(if_not_condition::assert_if_not, m)?)?;
    m.add_function(wrap_pyfunction!(limit_condition::check_if_in_limits, m)?)?;
    m.add_function(wrap_pyfunction!(limit_condition::assert_if_in_limits, m)?)?;
    m.add_function(wrap_pyfunction!(length_condition::check_length, m)?)?;
    m.add_function(wrap_pyfunction!(length_condition::assert_check_length, m)?)?;
    m.add_function(wrap_pyfunction!(is_close_condition::check_if_isclose, m)?)?;
    m.add_function(wrap_pyfunction!(is_close_condition::assert_if_isclose, m)?)?;
    m.add("LimitError", _py.get_type::<limit_condition::LimitError>())?;
    m.add("LengthError", _py.get_type::<length_condition::LengthError>())?;
    m.add("NotCloseEnoughError", _py.get_type::<is_close_condition::NotCloseEnoughError>())?;
    Ok(())
}