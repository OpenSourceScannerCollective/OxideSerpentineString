pub(crate) mod javascript;
pub(crate) mod python;
pub(crate) mod json;
pub(crate) mod toml;

use std::collections::HashMap;
use std::str::FromStr;
use pyo3::{pyclass, pyfunction, PyResult, Python, PyObject, IntoPy};
use strum_macros::EnumString;

#[pyclass(get_all)]
#[derive(Clone, Copy, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum ProgrammingLanguage {
    Python,
    JavaScript,
    Json,
    Toml,
}

#[pyfunction]
#[allow(dead_code)]
pub fn lang_from_str(str_input: &str) -> ProgrammingLanguage {
    return ProgrammingLanguage::from_str(str_input).unwrap();
}

#[pyclass(get_all)]
#[derive(Clone)]
pub struct ParseMatch {
    kind: String,
    value: String,
    raw: String,
    char: MatchPos,
    line: MatchPos,
    matches: HashMap<String, String>
}

#[pyclass(get_all)]
#[derive(Clone)]
pub struct MatchPos {
    start: usize,
    end: usize
}

#[pyfunction]
pub fn parse_with_enum(py: Python<'_>, str_input: &str, lang: ProgrammingLanguage) -> PyResult<PyObject> {

    let tokens = match lang {
        ProgrammingLanguage::JavaScript => javascript::parse(str_input),
        ProgrammingLanguage::Python => python::parse(str_input),
        ProgrammingLanguage::Json => json::parse(str_input),
        ProgrammingLanguage::Toml => toml::parse(str_input),
    };

    return Ok(tokens.into_py(py));
}

#[pyfunction]
pub fn parse(py: Python<'_>, str_input: &str, str_lang: &str) -> PyResult<PyObject> {
    let lang: ProgrammingLanguage = match ProgrammingLanguage::from_str(str_lang) {
        Ok(data) => data,
        Err(err) => panic!("Problem parsing string: {:?}", err),
    };
    return parse_with_enum(py, str_input, lang);
}