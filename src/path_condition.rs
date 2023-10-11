use panic::catch_unwind;
use std::any::Any;
use std::{env, panic};
use std::fmt::Debug;
use std::ops::Deref;
use std::path::Path;

use pyo3::{AsPyPointer, FromPyPointer, Py, PyAny, PyErr, pyfunction, PyObject, PyResult, Python, PyTypeInfo, ToPyObject};
use pyo3::exceptions::{PyAssertionError, PyFileNotFoundError, PyRuntimeError, PyTypeError, PyValueError};
use pyo3::ffi::{PyGen_New, PyIter_Next, PyObject_IsInstance};
use pyo3::types::{PyList, PyModule, PyString, PyTuple, PyType};
use crate::helper;

/// Check if a path or paths exist.
///
/// If it does not, either raise (or return) an exception or issue (or return)
/// a warning.
///
/// Args:
///     paths (str, pathlib.Path, abc.Sequence[str or pathlib.Path]): path or paths
///         to validate
///     handle_with (type): type of exception or warning to be raised/returned
///     message (str): a text to use as the exception/warning message.
///         Defaults to None, which means using no message for built-in
///         exceptions/warnings, and the docstrings of the exception/warning
///         class as a message for custom exceptions.
///     execution_mode (str): defines what happens if not all the paths exist
///         May take one of the following values:
///             - 'raise': exception/warning will be raised
///             - 'return': function will return information about the errors
///
/// Returns:
///     None, if execution_mode is 'raise' and check succeeded
///     A tuple, if execution_mode is 'return'. The tuple has two elements:
///         - an instance of the type provided by the handle_with parameter
///         - a list of the non-existing paths
///
/// Raises:
///     Exception of the type provided by the handle_with parameter,
///     FileNotFoundError by default (unless handle_with is a warning).
///
/// >>> import os
/// >>> check_if_paths_exist('Q:/Op/Oop/')
/// Traceback (most recent call last):
///     ...
/// FileNotFoundError
/// >>> check_if_paths_exist(os.listdir()[0])
/// >>> check_if_paths_exist(Path(os.listdir()[0]))
/// >>> check_if_paths_exist(os.listdir())
///
/// >>> check_if_paths_exist('Q:/Op/Oop', execution_mode='return')
/// (FileNotFoundError(), ['Q:/Op/Oop'])
/// >>> check_if_paths_exist(os.listdir()[0], execution_mode='return')
/// (None, [])
/// >>> check_if_paths_exist(os.listdir(), execution_mode='return')
/// (None, [])
///
/// To issue a warning, do the following (we'll catch the warning):
/// >>> with warnings.catch_warnings(record=True) as w:
/// ...     check_if_paths_exist('Q:/Op/Oop', handle_with=Warning)
/// >>> check_if_paths_exist('Q:/Op/Oop',
/// ...    execution_mode='return',
/// ...    handle_with=Warning)
/// (Warning(), ['Q:/Op/Oop'])
/// >>> check_if_paths_exist('Q:/Op/Oop',
/// ...    execution_mode='return',
/// ...    handle_with=Warning,
/// ...    message='Attempt to use a non-existing path')
/// (Warning('Attempt to use a non-existing path'), ['Q:/Op/Oop'])
#[pyfunction]
#[pyo3(signature = (paths, /, handle_with=None, message="", *, execution_mode="raise"))]
pub unsafe fn check_if_paths_exist(_py: Python,
                                   paths: PyObject,
                                   handle_with: Option<&PyType>,
                                   message: Option<&str>,
                                   execution_mode: Option<&str>) -> Result<Option<Py<PyAny>>, PyErr> {

    let is_disabled = env::var("EASYCHECK_RUN").unwrap_or("1".parse().unwrap()) == "0";

    if is_disabled {
        return Ok(None);
    }

    match execution_mode {
        None => {}
        Some(mode) => {
            if mode != "raise" && mode != "return" {
                return Err(PyValueError::new_err("execution_mode can only be `raise` or `return`"));
            }
        }
    }

    if PyObject_IsInstance(paths.as_ptr(), PyString::type_object(_py).as_ptr()).is_positive() {
        return check_single_path(_py, paths, handle_with, message, execution_mode);
    }

    let pathlib_mod = PyModule::import(_py, "pathlib").unwrap();
    let py_path_obj = pathlib_mod.call_method1("__getattribute__", PyTuple::new(_py, [PyString::new(_py, "Path")])).unwrap();

    if PyObject_IsInstance(paths.as_ptr(), py_path_obj.as_ptr()).is_positive() {
        return check_single_path(_py, paths, handle_with, message, execution_mode);
    }

    if PyObject_IsInstance(paths.as_ptr(),
                           PyTuple::new(_py, [PyList::type_object(_py), PyTuple::type_object(_py)]).as_ptr()).is_positive() {
        let obj_list: &PyList = paths.downcast(_py).unwrap();
        let ex_mode = execution_mode.unwrap();

        for obj in obj_list {
            match check_single_path(_py, PyObject::from(obj), handle_with, message, execution_mode) {
                Ok(succ) => {
                    if ex_mode == "return" {
                        match succ {
                            None => {}
                            Some(r_obj) => {
                                let tuple_obj: &PyTuple = r_obj.downcast(_py).unwrap();
                                // if the first element is not `None` it means it must be an error
                                if !tuple_obj[0].is_none() {
                                    return Ok(Some(r_obj));
                                }
                            }
                        }
                    }
                }
                Err(err) => {
                    return Err(err);
                }
            }
        }
        return return_success(_py, ex_mode);
    }

    let abc_mod = PyModule::import(_py, "collections.abc").unwrap();
    let py_iterable_obj = abc_mod.call_method1("__getattribute__", PyTuple::new(_py, [PyString::new(_py, "Iterable")])).unwrap();

    if PyObject_IsInstance(paths.as_ptr(),
                           py_iterable_obj.as_ptr()).is_positive() {
        let ex_mode = execution_mode.unwrap();

        while true {
            let mut_obj = PyIter_Next(paths.as_ptr());

            match mut_obj.as_ref() {
                None => {
                    break;
                }
                Some(_) => {
                    let py_obj = PyObject::from_borrowed_ptr(_py, mut_obj);
                    match check_single_path(_py, py_obj, handle_with, message, execution_mode) {
                        Ok(succ) => {
                            if ex_mode == "return" {
                                match succ {
                                    None => {}
                                    Some(r_obj) => {
                                        let tuple_obj: &PyTuple = r_obj.downcast(_py).unwrap();
                                        // if the first element is not `None` it means it must be an error
                                        if !tuple_obj[0].is_none() {
                                            return Ok(Some(r_obj));
                                        }
                                    }
                                }
                            }
                        }
                        Err(err) => {
                            return Err(err);
                        }
                    }
                }
            }
        }
        return return_success(_py, ex_mode);
    }

    return Err(PyTypeError::new_err("Argument paths must be string"));
}

fn return_success(_py: Python, ex_mode: &str) -> Result<Option<Py<PyAny>>, PyErr> {
    match ex_mode {
        "raise" => {
            return Ok(None);
        }
        "return" => {
            let empty_list: pyo3::PyObject = PyList::empty(_py).to_object(_py);
            let elements = vec![None, Some(empty_list)];
            let result: &PyAny = PyTuple::new(_py, elements);

            return Ok(Some(Py::from(result)));
        }
        _ => { return Err(PyRuntimeError::new_err("Something went wrong.")); }
    }
}

fn check_single_path(_py: Python,
                     paths: PyObject,
                     handle_with: Option<&PyType>,
                     message: Option<&str>,
                     execution_mode: Option<&str>) -> Result<Option<Py<PyAny>>, PyErr> {
    let ex_mode = execution_mode.unwrap();

    if Path::new(paths.to_string().as_str()).exists() {
        match ex_mode {
            "raise" => {
                return Ok(None);
            }
            "return" => {
                let empty_list: pyo3::PyObject = PyList::empty(_py).to_object(_py);
                let elements = vec![None, Some(empty_list)];
                let result: &PyAny = PyTuple::new(_py, elements);

                return Ok(Some(Py::from(result)));
            }
            _ => { return Err(PyRuntimeError::new_err("Something went wrong.")); }
        }
    } else {
        match ex_mode {
            "raise" => {
                match handle_with {
                    None => {
                        let mut err_msg: String = "".to_string();
                        match message {
                            None => {
                                err_msg = format!("{} is not a valid path", paths.to_string().as_str());
                            }
                            Some(msg) => {
                                err_msg = msg.to_string();
                            }
                        }
                        return Err(PyFileNotFoundError::new_err(err_msg));
                    }
                    Some(_) => {
                        return Err(helper::check_handle_warning_with(handle_with, message));
                    }
                }
            }
            "return" => {
                let mut py_exception = PyFileNotFoundError::new_err("").to_object(_py);
                match handle_with {
                    None => {}
                    Some(py_ex) => {
                        py_exception = py_ex.to_object(_py);
                    }
                }
                let py_list: pyo3::PyObject = PyList::new(_py, [paths]).to_object(_py);
                let elements = vec![py_exception, py_list];
                let result: &PyAny = PyTuple::new(_py, elements);

                return Ok(Some(Py::from(result)));
            }
            _ => { return Err(PyRuntimeError::new_err("Something went wrong.")); }
        }
    }
}

#[pyfunction]
pub unsafe fn assert_paths(_py: Python, paths: PyObject) -> Result<Option<Py<PyAny>>, PyErr> {
    return check_if_paths_exist(_py,
                                paths,
                                Option::from(PyAssertionError::new_err(()).get_type(_py)),
                                Option::from(""),
                                Option::from("raise"));
}