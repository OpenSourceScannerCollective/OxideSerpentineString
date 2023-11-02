use std::collections::HashMap;
use pyo3::{IntoPy, pyclass, pyfunction, PyResult, Python};
use pyo3::PyObject;
use {
    once_cell::sync::Lazy,
    regex::Regex,
};
use crate::language::MatchPos;


// TODO:    1. supply patterns from a config file at compile-time
//          2. supply patterns from a config file at run-time
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
#[derive(Clone)]
pub struct RegexMatch {
    pub kind: String,
    pub value: String,
    pub position: MatchPos
}

// TODO:    1. m̶o̶v̶e̶ ̶t̶o̶ ̶a̶ ̶v̶e̶c̶t̶o̶r̶ ̶o̶f̶ ̶h̶a̶s̶h̶m̶a̶p̶s̶ ̶t̶o̶ ̶a̶l̶l̶o̶w̶ ̶f̶o̶r̶ ̶m̶u̶l̶t̶i̶p̶l̶e̶ ̶i̶n̶s̶t̶a̶n̶c̶e̶s̶ ̶o̶f̶ ̶t̶h̶e̶ ̶s̶a̶m̶e̶ ̶k̶e̶y̶
//          2. provide meta-data for the match including positional information
pub fn do_regex(str_input: &str) -> HashMap<String, Vec<RegexMatch>> {

    let re_patterns: HashMap<&'static str, &'static Lazy<Regex>> = get_patterns();
    let mut re_captures:HashMap<String, Vec<RegexMatch>> = HashMap::new();

    for (key, pattern) in re_patterns {
        for cap in pattern.captures_iter(str_input) {

            let cap_match = cap.get(0).unwrap();
            let re_match = RegexMatch {
                kind: key.to_string(),
                value: cap_match.as_str().to_string(),
                position: MatchPos {
                    start: cap_match.start(),
                    end: cap_match.end()
                }
            };
            re_captures.entry(key.to_string()).or_insert(Vec::new()).push(re_match);
        }
    }

    return re_captures.into();
}

#[pyfunction]
#[pyo3(name = "do_regex")]
pub fn py_do_regex(py: Python<'_>, str_input: &str) -> PyResult<PyObject> {
    // Ok(do_regex(str_input).into_py_dict(py).to_object(py))
    Ok(do_regex(str_input).into_py(py))
}