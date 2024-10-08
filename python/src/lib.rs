use pyo3::exceptions::{PyIndexError, PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::*;
use sesdiff::{
    shortest_edit_script, shortest_edit_script_suffix, EditInstruction, EditScript, Mode,
};

#[pyclass]
#[pyo3(name = "Mode")]
#[derive(Clone, PartialEq, Default)]
struct PyMode(Mode);

#[pymethods]
impl PyMode {
    #[classattr]
    const NORMAL: PyMode = PyMode(Mode::Normal);

    #[classattr]
    const SUFFIX: PyMode = PyMode(Mode::Suffix);

    #[classattr]
    const PREFIX: PyMode = PyMode(Mode::Prefix);

    #[classattr]
    const INFIX: PyMode = PyMode(Mode::Infix);

    fn __richcmp__(&self, other: PyRef<Self>, op: CompareOp) -> Py<PyAny> {
        let py = other.py();
        match op {
            CompareOp::Eq => (*self == *other).into_py(py),
            CompareOp::Ne => (*self != *other).into_py(py),
            _ => py.NotImplemented(),
        }
    }
}

#[pyclass]
#[pyo3(name = "EditScript")]
#[derive(Clone, PartialEq, Default)]
struct PyEditScript {
    inner: EditScript<String>,
}

#[pymethods]
impl PyEditScript {
    fn mode(&self) -> PyMode {
        PyMode(self.inner.mode)
    }

    fn distance(&self) -> u32 {
        self.inner.distance
    }

    fn __len__(&self) -> usize {
        self.inner.instructions.len()
    }

    fn __getitem__(&self, index: isize) -> PyResult<(char, String)> {
        if let Some(instruction) = self.inner.instructions.get(&index) {
            match instruction {
                EditInstruction::Identify(s) => Ok(('=', s)),
                EditInstruction::Insertion(s) => Ok(('+', s)),
                EditInstruction::Deletion(s) => Ok(('-', s)),
            }
        } else {
            Err(PyIndexError::new_err("Index out of range for EditScript"))
        }
    }
}

#[pymethods]
#[pyfunction]
#[pyo3(name = "shortest_edit_script")]
fn shortest_edit_script_py(
    source: &str,
    target: &str,
    mode: PyMode,
    prefix: bool,
    generic: bool,
    allow_substitutions: bool,
) -> PyResult<PyEditScript> {
    if mode.0 == Mode::Suffix {
        ::sesdiff::shortest_edit_script_suffix()
    } else {
        ::sesdiff::shortest_edit_script_suffix()
    }
}

#[pymodule]
fn sesdiff(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class(wrap_pyclass!(PyMode, m));
    m.add_class(wrap_pyclass!(PyEditScript, m));
    m.add_function(wrap_pyfunction!(shortest_edit_script_py, m)?);
    Ok(())
}
