pub(crate) mod javascript;
pub(crate) mod python;
pub(crate) mod json;
pub(crate) mod toml;
pub(crate) mod csv;

use std::path::Path;
use std::str::FromStr;
use pest::Span;
use pyo3::{pyclass, pyfunction, PyResult, Python, PyObject, IntoPy};
use pyo3::exceptions::PyValueError;
use strum_macros::{Display, EnumString};
use crate::patterns::{do_regex, RegexMatchCollection};
use snailquote::unescape as snail_unescape;

#[pyclass(get_all)]
#[derive(Default, Clone, Copy, EnumString, Display )]
#[strum(ascii_case_insensitive)]
pub enum ProgrammingLanguage {
    #[default]
    Unknown,
    Python,
    JavaScript,
    Json,
    Toml,
    Csv,
}

// TODO: make this a function of ProgrammingLanguage class
#[pyfunction]
#[allow(dead_code)]
pub fn lang_from_str(str_input: &str) -> ProgrammingLanguage {
    return ProgrammingLanguage::from_str(str_input).unwrap();
}

#[pyfunction]
#[allow(dead_code)]
pub fn lang_to_str(lang: ProgrammingLanguage) -> PyResult<String> {
    return Ok(lang.to_string());
}

#[pyfunction]
#[allow(dead_code)]
pub fn kind_to_str(kind: ParseMatchType) -> PyResult<String> {
    return Ok(kind.to_string());
}

#[pyclass(get_all)]
#[derive(Default, Clone)]
pub struct ParseMatch {
    pub kind: ParseMatchType,
    pub language: ProgrammingLanguage,
    pub value: String,
    pub raw: String,
    pub position: MatchPos,
    pub matches: Vec<RegexMatchCollection>
}

impl ParseMatch {
    fn from(rule_str: ParseMatchType, lang: ProgrammingLanguage, value_str: &str, raw_str: &str, inner_span: Span) -> ParseMatch {
        let source_pos = MatchPos {
            char: MatchSpan {
                start: inner_span.start_pos().pos(),
                end: inner_span.end_pos().pos()
            },
            line: MatchSpan {
                start: inner_span.start_pos().line_col().0,
                end: inner_span.end_pos().line_col().0
            },
        };

        let str_value = snail_unescape(value_str).unwrap();

        let p_match = ParseMatch {
            kind: rule_str,
            language: lang,
            value: str_value.to_owned(),
            raw: raw_str.to_string(),
            position: source_pos.clone(),
            matches: do_regex(str_value.as_str(), Some(source_pos), Option::Some(true)).into()
        };

        return p_match;
    }
}

#[pyclass(get_all)]
#[derive(Default, Debug, Clone)]
pub struct MatchSpan {
    pub start: usize,
    pub end: usize
}

#[pyclass(get_all)]
#[derive(Default, Debug, Clone)]
pub struct MatchPos {
    pub char: MatchSpan,
    pub line: MatchSpan
}

#[pyclass(get_all)]
#[derive(Default, Clone, Copy, EnumString, Display)]
#[strum(ascii_case_insensitive)]
pub enum ParseMatchType {
    #[default]
    Unknown,
    StringLiteral,
    Comment
}

#[pyfunction]
pub fn parse_with_lang_enum(py: Python<'_>, str_input: &str, lang: ProgrammingLanguage) -> PyResult<PyObject> {

    let tokens = match lang {
        ProgrammingLanguage::JavaScript => javascript::parse(str_input),
        ProgrammingLanguage::Python => python::parse(str_input),
        ProgrammingLanguage::Json => json::parse(str_input),
        ProgrammingLanguage::Toml => toml::parse(str_input),
        ProgrammingLanguage::Csv => csv::parse(str_input),

        _ => panic!("Invalid programming language: Unknown"),
    };

    return Ok(tokens.into_py(py));
}

#[pyfunction]
pub fn parse_with_lang_str(py: Python<'_>, str_input: &str, str_lang: &str) -> PyResult<PyObject> {
    let lang: ProgrammingLanguage = match ProgrammingLanguage::from_str(str_lang) {
        Ok(data) => data,
        Err(err) => panic!("Problem parsing string: {:?}", err),
    };
    return parse_with_lang_enum(py, str_input, lang);
}

#[pyfunction]
pub fn parse(py: Python<'_>, str_input: &str, file_path: &str) -> PyResult<PyObject> {

    let str_lang = detect_lang_str(str_input, file_path);

    if str_lang.is_none() {
        return Err(PyValueError::new_err("Unable to detect language"))
    }

    let lang: ProgrammingLanguage = match ProgrammingLanguage::from_str(str_lang.unwrap().as_str()) {
        Ok(data) => data,
        Err(err) => panic!("Problem parsing string: {:?}", err),
    };
    return parse_with_lang_enum(py, str_input, lang);
}

#[pyfunction]
#[pyo3(name = "detect_lang")]
pub fn detect_lang_str(input_str: &str, file_path: &str) -> Option<String> {

    let lang_res = hyperpolyglot::detect_with_str(Path::new(file_path), input_str);

    if lang_res.is_err() {
        return Some("Unknown".to_string());
    }

    let detect = lang_res.unwrap();
    if detect.is_none() {
        return Some("Unknown".to_string());
    }

    Some(detect.unwrap().language().to_string())
}

#[pyfunction]
pub fn detect_lang_file(file_path: &str) -> Option<String> {

    let lang_res = hyperpolyglot::detect(file_path.as_ref());

    if lang_res.is_err() {
        return Some("Unknown".to_string());
    }

    let detect = lang_res.unwrap();
    if detect.is_none() {
        return Some("Unknown".to_string());
    }

    Some(detect.unwrap().language().to_string())
}