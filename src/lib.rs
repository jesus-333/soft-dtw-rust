use pyo3::prelude::*;

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

#[pyfunction]
fn compute_sdtw_1d(x : PyReadonlyArrayDyn<f32>, y : PyReadonlyArrayDyn<f32>) -> PyResult<f32> {
    sdtw_v1::print_type_of(&x);
    Ok(sdtw_v1::compute_1d(x.as_array(), y.as_array()))
}
