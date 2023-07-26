use pyo3::prelude::*;


#[derive(Debug)]
struct Fib(u64, u64);

impl Default for Fib {
    fn default() -> Self {
        Self(0, 1)
    }
}

impl Iterator for Fib {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let r = self.0;
        (self.0, self.1) = (self.1, self.0 + self.1);
        Some(r)
    }
}

/// Gets the first n elements of a fibonacci sequence.
#[pyfunction]
fn fib(n: usize) -> PyResult<Vec<u64>> {
    let f = Fib::default();
    Ok(f.take(n).collect())
}

/// A Python module implemented in Rust.
#[pymodule]
fn fibs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fib, m)?)?;
    Ok(())
}