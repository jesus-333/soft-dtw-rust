use ndarray::prelude::*;
use num::complex::ComplexFloat;
use numpy::ndarray::{ArrayViewD};

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - 

pub fn compute_1d(x : ArrayViewD<'_, f32>, y : ArrayViewD<'_, f32>, gamma : f32) -> f32 {
    let result = 1.;
    
    let gamma_for_computation = if gamma <= 0. {
        println!("gamma must be non negative. Current value is {}. Set the value to 1", gamma);
        1.
    } else {
        gamma
    };

    print_type_of(&x);

    // Create distance matrix. Even if I do this initial check for now I conside x and y of the
    // same dimension.
    // TODO implement version with x and y of different length
    let distance_matrix = if x.len() >= y.len() {
        Array::<f32, _>::zeros((x.len(), y.len()).f())
    } else {
        Array::<f32, _>::zeros((y.len(), x.len()).f())
    };
    println!("{:?}", distance_matrix);

    let n_diagonals = x.len() + y.len() - 1;
    println!("{}, {}, {}", n_diagonals, x.len(), y.len());
    
    // The value of the soft-dtw are compute in diagonals, starting from the upper left corner and
    // proceding toward the lower right corner.
    // The firs diagonal has only 1 element, i.e. the element in posizion (0, 0)
    // The second diagonal has 2 elements, i.e. the elements in position (0, 1) and (1, 0) 
    let mut n_elements_diagonal = 1;
    for n_1 in 0..n_diagonals{ // Iterate through the diagonals
        println!("\t{n_1}, {n_elements_diagonal}");
        // Compute the number of element in the diagonals
        // TODO modify when add arrays of different length
        if n_1 <= n_diagonals / 2 { // Upper left of the distance matrix
            n_elements_diagonal += 1;
        } else { // Lower right of the distance matrix
            n_elements_diagonal -= 1;

        }

        // Indices
        // let idx_array = Array::<f32, _>::range(0., n_elements_diagonal as f32, 1.);
        // idx 1 is used for the row of the matrix
        // idx 2 is used for the column of the matrix
        for idx_1 in 0..n_elements_diagonal {
            let idx_2 = n_elements_diagonal - 1 - idx_1;
            
            // Get the value above the one I want to compute
            let upper_value = if idx_2 - 1 < 0 {
                f32::MAX
            } else {
                1.
            };
        }
    }

    return result
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - 

fn compute_distance_matrix_value() -> f32 {
    return 1.
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - 
// Mathematical funtions

fn l_norm(x : f32, y : f32, n : usize) -> f32 {
    return f32::powf(num::pow(x - y, n), 1 as f32 / n as f32)
}

/// Optimized version of the soft min for the soft-dtw computation
/// Due to the structure of the distance matrix to compute a new value you only need to know
/// the values above, on the left and on the left diagonal above the current value.
/// Calling this 3 value x, y and z, and the new value soft_min then this function compute
/// soft_min = -gamma * ln(exp(-x/gamma) + exp(-y/gamma) + exp(-z/gamma))
/// where gamme is a non negative hyperparameter
fn compute_soft_min_optimized(upper_value : f32, upper_left_value : f32, left_value : f32, gamma : f32) -> f32{
    
    return  -gamma * ((-upper_value/gamma).exp() + (-upper_left_value/gamma).exp() + (-left_value/gamma).exp()).ln()

}

fn compute_soft_min_general(values_array : &[f32], gamma : f32) -> f32 {
    let mut soft_min = 0.0;
    for &value in values_array.iter(){
        soft_min += (-(value / gamma)).exp();
    }

    return -gamma * soft_min.ln()
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - 
//
pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
