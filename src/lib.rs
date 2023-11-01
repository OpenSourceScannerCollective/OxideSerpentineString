mod patterns;
mod language;

use pyo3::prelude::*;
use std::str::FromStr;
use crate::language::{MatchPos, ParseMatch, ProgrammingLanguage};

#[pyfunction]
fn parse_with_enum(py: Python<'_>, str_input: &str, lang: language::ProgrammingLanguage) -> PyResult<PyObject> {

    let tokens = match lang {
        ProgrammingLanguage::JavaScript => language::javascript::parse(str_input),
        ProgrammingLanguage::Python => language::python::parse(str_input),
        ProgrammingLanguage::Json => language::json::parse(str_input),
        ProgrammingLanguage::Toml => language::toml::parse(str_input),
    };

    Ok(tokens.into_py(py))
}

#[pyfunction]
fn parse(py: Python<'_>, str_input: &str, str_lang: &str) -> PyResult<PyObject> {
    let lang: ProgrammingLanguage = match ProgrammingLanguage::from_str(str_lang) {
        Ok(data) => data,
        Err(err) => panic!("Problem parsing string: {:?}", err),
    };
    return parse_with_enum(py, str_input, lang);
}

// A Python module implemented in Rust.
#[pymodule]
fn string_extract(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ParseMatch>()?;
    m.add_class::<MatchPos>()?;
    m.add_class::<ProgrammingLanguage>()?;
    m.add_function(wrap_pyfunction!(parse_with_enum, m)?)?;
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    m.add_function(wrap_pyfunction!(patterns::py_do_regex, m)?)?;
    Ok(())
}
