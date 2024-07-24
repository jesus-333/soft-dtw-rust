use ndarray::prelude::*;
use numpy::ndarray::{ArrayViewD};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - 

pub fn compute_1d(x : ArrayViewD<'_, f32>, y : ArrayViewD<'_, f32>) -> f32 {
    let result = 1.;

    print_type_of(&x);

    // Create distance matrix. Even if I do this initial check for now I conside x and y of the
    // same dimension.
    // TODO implement version with x and y of different length
    let distance_matrix = if x.len() >= y.len() {
        Array::<f32, _>::zeros((x.len(), y.len()).f());
    } else {
        Array::<f32, _>::zeros((y.len(), x.len()).f());
    };

    let n_diagonals = x.len() + y.len() - 1;
    println!("{}, {}, {}", n_diagonals, x.len(), y.len());
    
    let mut n_elements_diagonal = 1;
    for n_1 in 1..n_diagonals{ // Iterate through the diagonals
        
        // Compute the number of element in the diagonals
        // TODO modify when add arrays of different length
        if n_1 <= n_diagonals / 2 { // Upper left of the distance matrix
            n_elements_diagonal += 1;
        } else { // Lower right of the distance matrix
            n_elements_diagonal -= 1;
        }
        
        // Indices
        let idx = Array::<i32, _>::range(0., n_elements_diagonal as f32, 1.);
        for n_2 in 0..n_elements_diagonal {

        }
    }

    return result
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - 


fn l_norm(x : f32, y : f32, n : usize) -> f32 {
    return f32::powf(num::pow(x - y, n), 1 as f32 / n as f32);
}

pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
