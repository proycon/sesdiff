use ::sesdiff::{EditInstruction, EditScript, Mode};
use pyo3::basic::CompareOp;
use pyo3::exceptions::{PyIndexError, PyRuntimeError};
use pyo3::prelude::*;

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
#[derive(Clone, PartialEq)]
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
        let instruction: Option<&EditInstruction<String>> =
            self.inner.instructions.get(index as usize);
        if let Some(instruction) = instruction {
            match instruction {
                EditInstruction::Identity(s) => Ok(('=', s.to_string())),
                EditInstruction::Insertion(s) => Ok(('+', s.to_string())),
                EditInstruction::Deletion(s) => Ok(('-', s.to_string())),
                _ => Err(PyRuntimeError::new_err(
                    "EditInstructions with multiple items are not implemented in the python binding yet",
                )),
            }
        } else {
            Err(PyIndexError::new_err("Index out of range for EditScript"))
        }
    }

    fn __str__(&self) -> String {
        format!("{}", self.inner)
    }
}

#[pyfunction]
#[pyo3(name = "shortest_edit_script")]
#[pyo3(signature = (source, target, mode=PyMode(Mode::Normal), allow_substitutions=true))]
fn shortest_edit_script_py(
    source: &str,
    target: &str,
    mode: PyMode,
    allow_substitutions: bool,
) -> PyResult<PyEditScript> {
    let editscript = if mode.0 == Mode::Suffix {
        ::sesdiff::shortest_edit_script_suffix(source, target, false, allow_substitutions)
    } else {
        ::sesdiff::shortest_edit_script(
            source,
            target,
            mode.0 == Mode::Prefix,
            false,
            allow_substitutions,
        )
        .to_owned()
    };
    Ok(PyEditScript { inner: editscript })
}

#[pymodule]
fn sesdiff(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyMode>()?;
    m.add_class::<PyEditScript>()?;
    m.add_function(wrap_pyfunction!(shortest_edit_script_py, m)?)?;
    Ok(())
}
