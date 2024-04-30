mod resolver;
use pyo3::prelude::*;
use std::collections::HashSet;

#[pymodule]
fn sum_subset(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(resolve_sum_of_subset, m)?)?;
    m.add_function(wrap_pyfunction!(resolve_sum_of_subset_rec, m)?)?;
    m.add_function(wrap_pyfunction!(resolve_sum_of_subset_rec_spawn, m)?)?;
    Ok(())
}

#[pyfunction]
fn resolve_sum_of_subset(
    py: Python,
    value: Vec<HashSet<u32>>,
    weight: Vec<u32>,
    calc_distance: Option<PyObject>,
) -> PyResult<Vec<HashSet<usize>>> {
    let r = if let Some(calc_distance) = calc_distance {
        resolver::resolve_sum_of_subset(&value, &weight, |x: u32, y: u32| -> u32 {
            calc_distance
                .call_bound(py, (x, y), None)
                .unwrap()
                .extract(py)
                .unwrap()
        })
    } else {
        resolver::resolve_sum_of_subset(&value, &weight, |x: u32, y: u32| -> u32 {
            x * x + y * y + 2 * x * y
        })
    };
    let r: Vec<HashSet<usize>> = r
        .into_iter()
        .map(|node| node.visited_indices)
        .collect::<Vec<_>>();
    Ok(r)
}

#[pyfunction]
fn resolve_sum_of_subset_rec(
    py: Python,
    value: Vec<HashSet<u32>>,
    weight: Vec<u32>,
    calc_distance: Option<PyObject>,
) -> PyResult<Vec<HashSet<usize>>> {
    let r = if let Some(calc_distance) = calc_distance {
        resolver::resolve_sum_of_subset_rec(&value, &weight, |x: u32, y: u32| -> u32 {
            calc_distance
                .call_bound(py, (x, y), None)
                .unwrap()
                .extract(py)
                .unwrap()
        })
    } else {
        resolver::resolve_sum_of_subset_rec(&value, &weight, |x: u32, y: u32| -> u32 {
            x * x + y * y + 2 * x * y
        })
    };
    let r: Vec<HashSet<usize>> = r
        .into_iter()
        .map(|node| node.visited_indices)
        .collect::<Vec<_>>();
    Ok(r)
}

#[pyfunction]
fn resolve_sum_of_subset_rec_spawn(
    py: Python,
    value: Vec<HashSet<u32>>,
    weight: Vec<u32>,
    calc_distance: Option<PyObject>,
) -> PyResult<Vec<HashSet<usize>>> {
    let r = if let Some(calc_distance) = calc_distance {
        resolver::resolve_sum_of_subset_rec_spawn(value, weight, move |x: u32, y: u32| -> u32 {
            Python::with_gil(|py| {
                calc_distance
                    .call_bound(py, (x, y), None)
                    .unwrap()
                    .extract(py)
                    .unwrap()
            })
        })
    } else {
        resolver::resolve_sum_of_subset_rec_spawn(value, weight, |x: u32, y: u32| -> u32 {
            x * x + y * y + 2 * x * y
        })
    };
    let r: Vec<HashSet<usize>> = r
        .into_iter()
        .map(|node| node.visited_indices)
        .collect::<Vec<_>>();
    Ok(r)
}
