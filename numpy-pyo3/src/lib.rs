
use ndarray::{ArrayD, ArrayViewD};
use numpy::{IntoPyArray, PyArrayDyn};
use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use rayon::prelude::*;
use std::fs;
use std::path::PathBuf;

fn matches(word: &str, needle: &str) -> bool {
    let mut needle = needle.chars();
    for ch in word.chars().skip_while(|ch| !ch.is_alphabetic()) {
        match needle.next() {
            None => {
                return !ch.is_alphabetic();
            }
            Some(expect) => {
                if ch.to_lowercase().next() != Some(expect) {
                    return false;
                }
            }
        }
    }
    return needle.next().is_none();
}

/// Count the occurences of needle in line, case insensitive
#[pyfunction]
fn count_line(line: &str, needle: &str) -> usize {
    let mut total = 0;
    for word in line.split(' ') {
        if matches(word, needle) {
            total += 1;
        }
    }
    total
}

fn mult(x: ArrayViewD<f64>, y: ArrayViewD<f64>) -> ArrayD<f64> {
    &x + &y
}

#[pymodule]
fn _core(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(count_line))?;

    #[pyfn(m, "mult")]
    fn mult_py(py: Python, x: &PyArrayDyn<f64>, y: &PyArrayDyn<f64>) -> Py<PyArrayDyn<f64>> {
        let x = x.as_array();
        let y = y.as_array();
        mult(x, y).into_pyarray(py).to_owned()
    }

    Ok(())
}
