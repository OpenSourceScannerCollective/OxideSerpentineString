pub(crate) mod javascript;
pub(crate) mod python;

use std::collections::HashMap;
use std::str::FromStr;
use pyo3::{pyclass, pyfunction};
use strum_macros::EnumString;

#[pyclass(get_all)]
#[derive(Clone, Copy, EnumString)]
#[strum(ascii_case_insensitive)]
pub enum ProgrammingLanguage {
    Python,
    JavaScript
}

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

// impl ToPyObject for ParseMatch {
//     fn to_object(&self, py: pyo3::Python) -> pyo3::PyObject {
//         // let mdf = self.mdf();
//         // let date = PyDate::new(py, self.year(), mdf.month() as u8, mdf.day() as u8)
//         //     .expect("Failed to construct date");
//         // date.into()
//
//     }
// }

// impl IntoPy<pyo3::PyObject> for ParseMatch {
//     fn into_py(self, py: Python<'_>) -> PyObject {
//         ToPyObject::to_object(&self, py)
//     }
// }


#[pyclass(get_all)]
#[derive(Clone)]
pub struct MatchPos {
    start: usize,
    end: usize
}