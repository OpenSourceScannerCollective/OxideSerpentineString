use pyo3::prelude::*;
use pyo3::types::PyList;
use pest::Parser;
use std::collections::HashMap;
use pest::iterators::Pairs;
use {
    once_cell::sync::Lazy,
    regex::Regex,
};
use strum_macros::EnumString;
use std::str::FromStr;

#[pyclass(get_all)]
#[derive(Clone, Copy, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum Language {
    Python,
    Java,
    JavaScript
}

#[pyfunction]
#[allow(dead_code)]
fn lang_from_str(str_input: &str) -> Language {
    return Language::from_str(str_input).unwrap();
}

mod javascript {
    use pest_derive::Parser;

    #[derive(Parser)]
    #[grammar = "JavaScript.pest"]
    pub struct Parser;
}

#[pyclass(get_all)]
#[derive(Clone)]
struct ParseMatch {
    kind: String,
    value: String,
    raw: String,
    char: MatchPos,
    line: MatchPos,
    matches: HashMap<String, String>
}

#[pyclass(get_all)]
#[derive(Clone)]
struct MatchPos {
    start: usize,
    end: usize
}

#[pyfunction]
fn parse_with_enum(py: Python<'_>, str_input: &str, lang: Language) -> PyResult<PyObject> {

    let mut tokens:Vec<PyObject> = Vec::new();

    let pairs: Pairs<javascript::Rule> = match lang {
        Language::JavaScript => javascript::Parser::parse(javascript::Rule::PROGRAM, &str_input).unwrap_or_else(|e| panic!("{}", e)),
        _=> javascript::Parser::parse(javascript::Rule::PROGRAM, &str_input).unwrap_or_else(|e| panic!("{}", e))
    };

    for pair in pairs {
        for inner_pair in pair.into_inner() {

            let rule_str:&str = match inner_pair.as_rule() {
                javascript::Rule::COMMENTS => "comment",
                javascript::Rule::STRING => "string",
                _=> continue,
            };

            let mut match_contents:&str = "";
            for nested_pair in inner_pair.clone().into_inner() {
                match nested_pair.as_rule() {
                    javascript::Rule::sl_str_text |
                    javascript::Rule::ml_str_text |
                    javascript::Rule::sl_comment_text |
                    javascript::Rule::ml_comment_text => {
                        match_contents = nested_pair.as_str();
                        break;
                    },
                    _ => {
                        continue;
                    },
                }
            }

            let p_match = ParseMatch {
                kind: rule_str.to_string(),
                value: match_contents.to_string(),
                raw: inner_pair.as_str().to_string(),
                char: MatchPos {
                    start: inner_pair.as_span().start_pos().pos(),
                    end: inner_pair.as_span().end_pos().pos()
                },
                line: MatchPos {
                    start: inner_pair.as_span().start_pos().line_col().0,
                    end: inner_pair.as_span().end_pos().line_col().0
                },
                matches: do_regex(py,  inner_pair.as_str()).into()
            };

            tokens.push( p_match.into_py(py));
        }
    }

    let list: &PyList = PyList::new(py, tokens);
    Ok(list.into())
}

#[pyfunction]
fn parse(py: Python<'_>, str_input: &str, str_lang: &str) -> PyResult<PyObject> {
    let lang: Language = match Language::from_str(str_lang) {
        Ok(data) => data,
        Err(err) => panic!("Problem parsing string: {:?}", err),
    };
    return parse_with_enum(py, str_input, lang);
}

#[allow(dead_code)]
fn get_patterns() -> HashMap<&'static str, &'static Lazy<Regex>> {

    /*
        Lazy load compiled regular expressions
     */

    static RSA_PRIVATE_KEY: Lazy<Regex> = Lazy::new(|| Regex::new(r"(-----BEGIN RSA PRIVATE KEY-----)").unwrap());
    static SSH_DSA_PRIVATE_KEY: Lazy<Regex> = Lazy::new(|| Regex::new(r"(-----BEGIN DSA PRIVATE KEY-----)").unwrap());
    static EC_PRIVATE_KEY: Lazy<Regex> = Lazy::new(|| Regex::new(r"(-----BEGIN EC PRIVATE KEY-----)").unwrap());
    static PGP_PRIVATE_KEY: Lazy<Regex> = Lazy::new(|| Regex::new(r"(-----BEGIN PGP PRIVATE KEY BLOCK-----)").unwrap());
    static GOOGLE_API_KEY: Lazy<Regex> = Lazy::new(|| Regex::new(r"(AIza[0-9A-Za-z\\-_]{35})").unwrap());
    static GOOGLE_OAUTH_TOKEN: Lazy<Regex> = Lazy::new(|| Regex::new(r"(ya29\\.[0-9A-Za-z\\-_]+)").unwrap());
    static GOOGLE_SERVICE_ACCOUNT: Lazy<Regex> = Lazy::new(|| Regex::new("(\"type\": \"service_account\")").unwrap());

    let re_patterns:HashMap<&'static str, &'static Lazy<Regex>> = HashMap::from([
        ("RSA private key", &RSA_PRIVATE_KEY),
        ("SSH (DSA) private key", &SSH_DSA_PRIVATE_KEY),
        ("SSH (EC) private key", &EC_PRIVATE_KEY),
        ("PGP private key block", &PGP_PRIVATE_KEY),
        ("Google API Key", &GOOGLE_API_KEY),
        ("Google OAuth Access Token", &GOOGLE_OAUTH_TOKEN),
        ("Google (GCP) Service-account", &GOOGLE_SERVICE_ACCOUNT),
    ]);

    return re_patterns;
}

fn do_regex(_py: Python<'_>, str_input: &str) -> HashMap<String, String> {

    let re_patterns: HashMap<&'static str, &'static Lazy<Regex>> = get_patterns();
    let mut re_captures:HashMap<String, String> = HashMap::new();

    for (key, pattern) in re_patterns {
        for (cap, [_group1]) in pattern.captures_iter(str_input).map(|c| c.extract()) {
            re_captures.insert(key.to_string(), cap.to_string());
            re_captures.insert(key.to_string(), cap.to_string());
        }
    }

    return re_captures.into();
}

/// A Python module implemented in Rust.
#[pymodule]
fn string_extract(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ParseMatch>()?;
    m.add_class::<MatchPos>()?;
    m.add_class::<Language>()?;
    m.add_function(wrap_pyfunction!(parse_with_enum, m)?)?;
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    Ok(())
}
