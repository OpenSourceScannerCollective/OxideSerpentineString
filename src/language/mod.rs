pub(crate) mod javascript;
pub(crate) mod python;
pub(crate) mod json;
pub(crate) mod toml;

use std::collections::HashMap;
use std::str::FromStr;
use pyo3::{pyclass, pyfunction};
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

// fn get_parser_for_lang(lang: ProgrammingLanguage) -> Parser {
//
//     let lang_parser = match lang {
//         ProgrammingLanguage::JavaScript => javascript::JavaScriptParser,
//         ProgrammingLanguage::Python => python::PythonParser,
//         ProgrammingLanguage::Json => json::JsonParser,
//     };
//
//     Ok(lang_parser)
// }

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