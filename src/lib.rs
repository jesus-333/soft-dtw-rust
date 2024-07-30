use pyo3::prelude::*;
use ndarray::prelude::*;

use numpy::{PyReadonlyArrayDyn, PyArrayDyn};

mod sdtw_v1;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - 

/// A Python module implemented in Rust.
#[pymodule]
fn soft_dtw_rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(compute_sdtw_1d, m)?)?;
    Ok(())
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - 
// Function in the python module

// #[pyfunction] // NOT WORK
fn get_distance_matrix(x : PyReadonlyArrayDyn<f64>, y : PyReadonlyArrayDyn<f64>, gamma : Option<f64>) -> PyResult<Array2<f64>> {
    // sdtw_v1::print_type_of(&x);

    Ok(sdtw_v1::compute_distance_matrix(x.as_array(), y.as_array(), gamma.unwrap_or(1.)))
}

#[pyfunction]
fn compute_sdtw_1d(x : PyReadonlyArrayDyn<f64>, y : PyReadonlyArrayDyn<f64>, gamma : Option<f64>) -> PyResult<f64> {
    // sdtw_v1::print_type_of(&x);
    Ok(sdtw_v1::compute_sdtw(x.as_array(), y.as_array(), gamma.unwrap_or(1.)))
}
