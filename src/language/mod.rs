pub(crate) mod javascript;
pub(crate) mod python;
pub(crate) mod json;
pub(crate) mod toml;

use std::str::FromStr;
use pest::Span;
use pyo3::{pyclass, pyfunction, PyResult, Python, PyObject, IntoPy};
use strum_macros::EnumString;
use crate::patterns::{do_regex, RegexMatchCollection};

#[pyclass(get_all)]
#[derive(Clone, Copy, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum ProgrammingLanguage {
    Python,
    JavaScript,
    Json,
    Toml,
}

// TODO: make this a function of ProgrammingLanguage class
#[pyfunction]
#[allow(dead_code)]
pub fn lang_from_str(str_input: &str) -> ProgrammingLanguage {
    return ProgrammingLanguage::from_str(str_input).unwrap();
}

#[pyclass(get_all)]
#[derive(Clone)]
pub struct ParseMatch {
    pub kind: String,
    pub value: String,
    pub raw: String,
    pub char: MatchPos,
    pub line: MatchPos,
    pub matches: Vec<RegexMatchCollection>
}

impl ParseMatch {
    fn from(rule_str: &str, value_str: &str, raw_str: &str, inner_span: Span) -> ParseMatch {
        let p_match = ParseMatch {
            kind: rule_str.to_string(),
            value: value_str.to_string(),
            raw: raw_str.to_string(),
            char: MatchPos {
                start: inner_span.start_pos().pos(),
                end: inner_span.end_pos().pos()
            },
            line: MatchPos {
                start: inner_span.start_pos().line_col().0,
                end: inner_span.end_pos().line_col().0
            },
            matches: do_regex(value_str).into()
        };

        return p_match;
    }
}

#[pyclass(get_all)]
#[derive(Clone)]
pub struct MatchPos {
    pub start: usize,
    pub end: usize
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