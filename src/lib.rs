mod patterns;
mod language;

use pyo3::prelude::*;

#[pymodule]
#[pyo3(name="oxide_serpentine_string")]
fn oxide_serpentine_string(_py: Python, m: &PyModule) -> PyResult<()> {

    // Parsing
    m.add_class::<language::ParseMatch>()?;
    m.add_class::<language::ParseMatchType>()?;
    m.add_class::<language::MatchPos>()?;
    m.add_class::<language::ProgrammingLanguage>()?;
    m.add_function(wrap_pyfunction!(language::parse_with_lang_enum, m)?)?;
    m.add_function(wrap_pyfunction!(language::parse_with_lang_str, m)?)?;
    m.add_function(wrap_pyfunction!(language::lang_from_str, m)?)?;
    m.add_function(wrap_pyfunction!(language::lang_to_str, m)?)?;
    m.add_function(wrap_pyfunction!(language::kind_to_str, m)?)?;
    m.add_function(wrap_pyfunction!(language::parse, m)?)?;

    // Regular Expressions
    m.add_class::<patterns::RegexMatch>()?;
    m.add_class::<patterns::RegexMatchCollection>()?;
    m.add_function(wrap_pyfunction!(patterns::py_do_regex, m)?)?;
    m.add_function(wrap_pyfunction!(language::detect_lang_str, m)?)?;
    m.add_function(wrap_pyfunction!(language::detect_lang_file, m)?)?;

    Ok(())
}
