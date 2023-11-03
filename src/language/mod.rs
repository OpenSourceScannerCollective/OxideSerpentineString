pub(crate) mod javascript;
pub(crate) mod python;
pub(crate) mod json;
pub(crate) mod toml;
pub(crate) mod csv;

use std::path::Path;
use std::str::FromStr;
use pest::Span;
use pyo3::{pyclass, pyfunction, PyResult, Python, PyObject, IntoPy};
use strum_macros::{Display, EnumString};
use crate::patterns::{do_regex, RegexMatchCollection};

extern crate syntect;
use syntect::parsing::{SyntaxReference, SyntaxSet};
use syntect::util::LinesWithEndings;

#[pyclass(get_all)]
#[derive(Clone, Copy, EnumString, Display )]
#[strum(ascii_case_insensitive)]
pub enum ProgrammingLanguage {
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

#[pyclass(get_all)]
#[derive(Default, Clone)]
pub struct ParseMatch {
    pub kind: ParseMatchType,
    pub value: String,
    pub raw: String,
    pub position: MatchPos,
    pub matches: Vec<RegexMatchCollection>
}

impl ParseMatch {
    fn from(rule_str: ParseMatchType, value_str: &str, raw_str: &str, inner_span: Span) -> ParseMatch {
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
        let p_match = ParseMatch {
            kind: rule_str,
            value: value_str.to_string(),
            raw: raw_str.to_string(),
            position: source_pos.clone(),
            matches: do_regex(value_str, Some(source_pos)).into()
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
pub fn parse_with_enum(py: Python<'_>, str_input: &str, lang: ProgrammingLanguage) -> PyResult<PyObject> {

    let detect_lang = detect_language(str_input, None);

    println!("Detected language: {:?}", detect_lang);

    let tokens = match lang {
        ProgrammingLanguage::JavaScript => javascript::parse(str_input),
        ProgrammingLanguage::Python => python::parse(str_input),
        ProgrammingLanguage::Json => json::parse(str_input),
        ProgrammingLanguage::Toml => toml::parse(str_input),
        ProgrammingLanguage::Csv => csv::parse(str_input),
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

#[pyfunction]
pub fn detect_language(input_str: &str, file_path: Option<&str>) -> Option<String> {

    let syntax_set = SyntaxSet::load_defaults_newlines();
    let mut syntax_ref: Option<&SyntaxReference>;

    if file_path.is_some() {
        let path: &Path = Path::new(file_path.unwrap());
        let file_ext = path.extension().and_then(|x| x.to_str()).unwrap_or("");
        syntax_ref = syntax_set.find_syntax_by_extension(file_ext);
        if syntax_ref.is_some() {
            return Some(syntax_ref.unwrap().name.to_string());
        }
    }

    for line in LinesWithEndings::from(input_str) {
        syntax_ref = syntax_set.find_syntax_by_first_line(line);
        if !syntax_ref.is_none() {
            return Some(syntax_ref.unwrap().name.to_string());
        }
    }

    // If no language is detected, return plaintext
    Some(syntax_set.find_syntax_plain_text().name.to_string())
}