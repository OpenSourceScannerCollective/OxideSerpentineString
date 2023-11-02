mod patterns;
mod language;

use pyo3::prelude::*;

#[pymodule]
fn oxide_serpentine_string(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<language::ParseMatch>()?;
    m.add_class::<language::ParseMatchType>()?;
    m.add_class::<language::MatchPos>()?;
    m.add_class::<language::ProgrammingLanguage>()?;
    m.add_function(wrap_pyfunction!(language::parse_with_enum, m)?)?;
    m.add_function(wrap_pyfunction!(language::parse, m)?)?;
    m.add_class::<patterns::RegexMatch>()?;
    m.add_class::<patterns::RegexMatchCollection>()?;
    m.add_function(wrap_pyfunction!(patterns::py_do_regex, m)?)?;
    Ok(())
}
