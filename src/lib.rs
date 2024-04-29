mod resolver;
use pyo3::prelude::*;
use std::collections::HashSet;

#[pymodule]
fn sum_subset(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(resolve_sum_of_subset, m)?)?;
    Ok(())
}

#[pyfunction]
fn resolve_sum_of_subset(
    py: Python,
    value: Vec<HashSet<u32>>,
    weight: Vec<u32>,
    calc_distance: PyObject,
) -> PyResult<Vec<HashSet<usize>>> {
    let f = |x: u32, y: u32| -> u32 {
        calc_distance
            .call(py, (x, y), None)
            .unwrap()
            .extract(py)
            .unwrap()
    };
    let r: Vec<HashSet<usize>> = resolver::resolve_sum_of_subset(value, weight, f)
        .into_iter()
        .map(|node| node.visited_indices)
        .collect::<Vec<_>>();
    Ok(r)
}
