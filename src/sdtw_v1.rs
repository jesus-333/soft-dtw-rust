use ndarray::prelude::*;
use numpy::ndarray::{ArrayViewD, ArrayViewMutD};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - 

pub fn compute_1d(x : ArrayViewD<'_, f32>, y : ArrayViewD<'_, f32>) -> f32 {
    let result = 1.;

    print_type_of(&x);

    // let distance_matrix = Array::zeros([x.len(), y.len()]);
    let distance_matrix = Array::<f32, _>::zeros((x.len(), y.len()).f());

    let n_diagonals = x.len() + y.len();

    // print!("{:?}\n", distance_matrix.dim());
    // print!("{:?}\n", distance_matrix.shape());
    // print!("{}\n", distance_matrix.shape()[0]);

    return result
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - 


fn l_norm(x : f32, y : f32, n : usize) -> f32 {
    return f32::powf(num::pow(x - y, n), 1 as f32 / n as f32);
}

pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
