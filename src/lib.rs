pub mod resolver;

use pyo3::prelude::*;
use std::collections::HashSet;

macro_rules! run_resolver {
    ( $func:path, $value:expr, $weight:expr, $calc_distance:path ) => {{
        let r = match ($calc_distance) {
            Some(calc_distance) => ($func)(($value), ($weight), move |x: u32, y: u32| -> u32 {
                Python::with_gil(|py| calc_distance.call_bound(py, (x, y), None).unwrap().extract(py).unwrap())
            }),
            None => ($func)(($value), ($weight), |x: u32, y: u32| -> u32 { x * x + y * y + 2 * x * y }),
        };
        let r: Vec<HashSet<usize>> = r.into_iter().map(|node| node.visited_indices).collect();
        Ok(r)
    }};
}

#[pyfunction]
fn resolve_sum_of_subset(_py: Python, value: Vec<HashSet<u32>>, weight: Vec<u32>, calc_distance: Option<PyObject>) -> PyResult<Vec<HashSet<usize>>> {
    run_resolver!(resolver::resolve_sum_of_subset, value, weight, calc_distance)
}

#[pyfunction]
fn resolve_sum_of_subset_rec(_py: Python, value: Vec<HashSet<u32>>, weight: Vec<u32>, calc_distance: Option<PyObject>) -> PyResult<Vec<HashSet<usize>>> {
    run_resolver!(resolver::resolve_sum_of_subset_rec, value, weight, calc_distance)
}

#[pyfunction]
fn resolve_sum_of_subset_rec_spawn(_py: Python, value: Vec<HashSet<u32>>, weight: Vec<u32>, calc_distance: Option<PyObject>) -> PyResult<Vec<HashSet<usize>>> {
    run_resolver!(resolver::resolve_sum_of_subset_rec_spawn, value, weight, calc_distance)
}

#[pyfunction]
fn resolve_sum_of_subset_rec_rayon(_py: Python, value: Vec<HashSet<u32>>, weight: Vec<u32>, calc_distance: Option<PyObject>) -> PyResult<Vec<HashSet<usize>>> {
    run_resolver!(resolver::resolve_sum_of_subset_rec_rayon, value, weight, calc_distance)
}

#[pymodule]
fn sum_subset(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(resolve_sum_of_subset, m)?)?;
    m.add_function(wrap_pyfunction!(resolve_sum_of_subset_rec, m)?)?;
    m.add_function(wrap_pyfunction!(resolve_sum_of_subset_rec_spawn, m)?)?;
    m.add_function(wrap_pyfunction!(resolve_sum_of_subset_rec_rayon, m)?)?;
    Ok(())
}
