use pyo3::prelude::*;
use ndarray::prelude::*;

use numpy::{PyArray2, PyArrayDyn, PyReadonlyArrayDyn};

mod sdtw_v1;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - 

/// A Python module implemented in Rust.
#[pymodule]
fn soft_dtw_rust(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(compute_sdtw_1d, m)?)?;
    m.add_function(wrap_pyfunction!(compute_sdtw_2d, m)?)?;
    Ok(())
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - 
// Function in the python module

#[pyfunction]
/// Compute the sdtw between the signal x and the signal y. Both signal must be 1D signal.
fn compute_sdtw_1d(x : PyReadonlyArrayDyn<f64>, y : PyReadonlyArrayDyn<f64>, gamma : Option<f64>) -> PyResult<f64> {
    // sdtw_v1::print_type_of(&x);
    Ok(sdtw_v1::compute_sdtw(x.as_array().slice(s![..]), y.as_array().slice(s![..]), gamma.unwrap_or(1.)))
}

#[pyfunction]
/// Apply the sdtw at two 2D matrices. The matrix must be in the form n_channels x n_time_samples
/// The algorithm compute the sdtw row by row (i.e. the first row of x is compared with the first
/// row of y and so on).
/// The final output is the sum of the sdtw along the channels.
/// If the parameter average_along_channels is passed with value true the final output will be
/// averaged by the number of channels. If the parameter is not passed by default is considered
/// with value false.
fn compute_sdtw_2d(x : PyReadonlyArrayDyn<f64>, y : PyReadonlyArrayDyn<f64>, gamma : Option<f64>, average_along_channels : Option<bool>) -> PyResult<f64> {
    // Convert variable
    let x = x.as_array();
    let y = y.as_array();
    let average_along_channels = average_along_channels.unwrap_or(false);

    let mut output : f64 = 0.;

    for i in 0..x.shape()[0] {
        output += sdtw_v1::compute_sdtw(x.slice(s![i, ..]), y.slice(s![i, ..]), gamma.unwrap_or(1.))
    }
    
    if average_along_channels {
        Ok(output / x.shape()[0] as f64)
    } else {
        Ok(output)
    }
}

// #[pyfunction] // NOT WORK
// fn get_distance_matrix_1d(x : PyReadonlyArrayDyn<f64>, y : PyReadonlyArrayDyn<f64>, gamma : Option<f64>) -> PyResult<f64> {
//     // sdtw_v1::print_type_of(&x);
//     let distance_matrix = sdtw_v1::compute_distance_matrix(x.as_array().slice(s![..]), y.as_array().slice(s![..]), gamma.unwrap_or(1.));
//     let pyarray = Python::with_gil(|py| {
//         PyArray2::from_array_bound(py, &distance_matrix)
//     });
//
//     Ok(1.)
// }

// fn convert_to_pyarray(x : ArrayView2<'_, f64>) -> PyArray2<f64>{
//
//     let a = Python::with_gil(|py| {
//         PyArray2::from_array_bound(py, &x)
//     });
//
//     return a
// }
