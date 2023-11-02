use std::collections::HashMap;
use line_numbers::LinePositions;
use pyo3::{IntoPy, pyclass, pyfunction, PyResult, Python};
use pyo3::PyObject;
use regex::Match;
use {
    once_cell::sync::Lazy,
    regex::Regex,
};
use crate::language::{MatchPos, MatchSpan};


// TODO:    1. procedural macro to create patterns at compile-time
//          2. ability to supply patterns at run-time
#[allow(dead_code)]
pub fn get_patterns() -> HashMap<&'static str, &'static Lazy<Regex>> {

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

#[pyclass(get_all)]
#[derive(Default, Debug, Clone)]
pub struct RegexMatchCollection {
    pub kind: String,
    pub source: String,
    pub matches: Vec<RegexMatch>
}

#[pyclass(get_all)]
#[derive(Default, Debug, Clone)]
pub struct RegexMatch {
    pub value: String,
    pub position: MatchPos,
    pub source_pos: MatchPos
}

pub fn do_regex(str_input: &str, source_pos: Option<MatchPos>) -> Vec<RegexMatchCollection> {

    let mut re_results: Vec<RegexMatchCollection> = Vec::new();
    let source_position = match source_pos {
        None => MatchPos::default(),
        _=> source_pos.clone().unwrap()
    };

    let source = str_input;
    for (key, pattern) in get_patterns() {

        let mut re_matches:Vec<RegexMatch> = Vec::new();
        let mut cap_match: Match;
        let line_positions = LinePositions::from(str_input);

        for cap in pattern.captures_iter(str_input) {
            cap_match = cap.get(0).unwrap();
            re_matches.push(RegexMatch {
                value: cap_match.as_str().to_string(),
                position: MatchPos {
                    char: MatchSpan {
                        start: cap_match.start(),
                        end: cap_match.end()
                    },
                    line: MatchSpan {
                        start: line_positions.from_offset(cap_match.start()).as_usize() + 1,
                        end: line_positions.from_offset(cap_match.end()).as_usize() + 1
                    }
                },
                source_pos: MatchPos {
                    char: MatchSpan {
                        start: cap_match.start() + source_position.char.start,
                        end: cap_match.end() + source_position.char.end
                    },
                    line: MatchSpan {
                        start: line_positions.from_offset(cap_match.start()).as_usize() + source_position.line.start,
                        end: line_positions.from_offset(cap_match.end()).as_usize() + source_position.line.end
                    }
                },
            });
        }

        if re_matches.len() > 0 {
            re_results.push(RegexMatchCollection {
                kind: key.to_string(),
                source: source.to_string(),
                matches: re_matches,
            });
        }
    }

    return re_results.into();
}

#[pyfunction]
#[pyo3(name = "do_regex")]
pub fn py_do_regex(py: Python<'_>, str_input: &str, source_pos: Option<MatchPos>) -> PyResult<PyObject> {
    Ok(do_regex(str_input, source_pos).into_py(py))
}