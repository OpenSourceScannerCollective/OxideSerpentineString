use std::collections::HashMap;
use pyo3::{pyfunction, PyResult, Python, ToPyObject};
use pyo3::types::IntoPyDict;
use pyo3::PyObject;
use {
    once_cell::sync::Lazy,
    regex::Regex,
};


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

// TODO:    1. move to a vector of hashmaps to allow for multiple instances of the same key
//          2. provide meta-data for the match including positional information
pub fn do_regex(str_input: &str) -> HashMap<String, String> {

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

#[pyfunction]
#[pyo3(name = "do_regex")]
pub fn py_do_regex(py: Python<'_>, str_input: &str) -> PyResult<PyObject> {
    Ok(do_regex(str_input).into_py_dict(py).to_object(py))
}