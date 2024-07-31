use core::panic;

use ndarray::prelude::*;
use numpy::ndarray::ArrayView1;

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - 

pub fn compute_sdtw(x : ArrayView1<'_, f64>, y : ArrayView1<'_, f64>, gamma : f64) -> f64 {
    let x_len = x.len();
    let y_len = y.len();

    if x_len != y_len {
        panic!("x and y must have the same length. x length = {}, y length = {}", x_len, y_len)
    }

    let distance_matrix = compute_distance_matrix(x, y, gamma);

    return distance_matrix[[x_len -1, y_len -1]];
}

pub fn compute_distance_matrix(x : ArrayView1<'_, f64>, y : ArrayView1<'_, f64>, gamma : f64) -> Array2<f64> {
    
    let gamma_for_computation = if gamma <= 0. {
        println!("gamma must be non negative. Current value is {}. Set the value to 1", gamma);
        1.
    } else {
        gamma
    };

    // print_type_of(&x);

    // Create distance matrix. Even if I do this initial check for now I conside x and y of the
    // same dimension.
    // TODO implement version with x and y of different length
    let mut distance_matrix = if x.len() >= y.len() {
        Array::<f64, _>::zeros((x.len(), y.len()).f())
    } else {
        Array::<f64, _>::zeros((y.len(), x.len()).f())
    };
    
    // Compute element in position (0, 0)
    distance_matrix[[0, 0]] = l_norm(x[0], y[0], 2);

    let n_diagonals = x.len() + y.len() - 1;
    
    // The value of the soft-dtw are compute in diagonals, starting from the upper left corner and
    // proceding toward the lower right corner.
    // The firs diagonal has only 1 element, i.e. the element in posizion (0, 0)
    // The second diagonal has 2 elements, i.e. the elements in position (0, 1) and (1, 0) 
    let mut n_elements_diagonal = 1;

    // Compute the shift (used to compute the correct indices if you are in the lower right
    // section of the matrix)
    let mut shift = 0;
    
    // println!("n_diagonals = {}, n_diagonals / 2 = {}", n_diagonals, n_diagonals / 2);
    // Compute soft-dtw for each element of the matrix
    for n_1 in 1..n_diagonals{ // Iterate through the diagonals
        // Compute the number of element in the diagonals
        // TODO modify when add arrays of different length
        let computation_in_the_upper_left_section : bool;
        if n_1 <= n_diagonals / 2 { // Upper left of the distance matrix
            n_elements_diagonal += 1;
            computation_in_the_upper_left_section = true;
        } else { // Lower right of the distance matrix
            n_elements_diagonal -= 1;
            computation_in_the_upper_left_section = false;
        }
        
        // Compute the shift for the lower right section of the matrix
        shift = if computation_in_the_upper_left_section {
            0
        } else {
            shift + 1
        };
        
        // Print for debug
        // println!("n_1 = {} ({}), n_elements_diagonal = {}, upper_left_section = {}, shift = {}", n_1, n_1 + 1, n_elements_diagonal, computation_in_the_upper_left_section, shift);

        // Indices
        // let idx_array = Array::<f64, _>::range(0., n_elements_diagonal as f64, 1.);
        // idx 1 is used for the row of the matrix
        // idx 2 is used for the column of the matrix
        for n_2 in 0..n_elements_diagonal {
            // Set indices
            let idx_1 = n_2 + shift;
            let idx_2 = n_elements_diagonal - 1 - n_2 + shift;

            // Print for debug
            // println!("\tidx_1 = {}, idx_2 = {}", idx_1, idx_2);

            // Get the value above the one I want to compute
            let left_value = if idx_1 as i64 - 1 < 0 {
                i16::MAX as f64
            } else {
                distance_matrix[[idx_1 - 1, idx_2]]
            };
            
            // Get the value above the one I want to compute
            let upper_value = if idx_2 as i64 - 1 < 0 {
                i16::MAX as f64
            } else {
                distance_matrix[[idx_1, idx_2 - 1]]
            };

            // Get the value above and on the left (i.e. in diagonal) respect the one I want to compute
            let upper_left_value = if idx_1 as i64 - 1 < 0 || idx_2 as i64 -1 < 0 {
                i16::MAX as f64
            } else {
                distance_matrix[[idx_1 - 1, idx_2 - 1]]
            };
            
            // Compute the dtw value for the current cell
            let l_norm_value = l_norm(x[idx_1], y[idx_2], 2);
            let soft_min_values = compute_soft_min_optimized(upper_value, upper_left_value, left_value, gamma_for_computation);
            distance_matrix[[idx_1, idx_2]] = l_norm_value + soft_min_values;

            // if l_norm_value.is_infinite() || soft_min_values.is_infinite() {
            //     println!("{:?}", distance_matrix);
            //     println!("l_norm_value      = {}", l_norm_value);
            //     println!("soft_min_values   = {}", soft_min_values);
            //     println!("upper_value       = {}", upper_value);
            //     println!("upper_left_value  = {}", upper_left_value);
            //     println!("left_value        = {}", left_value);
            //     println!("idx_1 = {}, idx_2 = {}", idx_1, idx_2);
            //
            //     return distance_matrix
            // }
        }
    }

    return distance_matrix 
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - 
// Mathematical funtions

fn l_norm(x : f64, y : f64, n : usize) -> f64 {
    return f64::powf(num::pow(x - y, n), 1 as f64 / n as f64)
}

/// "Optimized" version of the soft min for the soft-dtw computation.
/// Due to the structure of the distance matrix to compute a new value you only need to know
/// the values above, on the left and on the left diagonal above the current value.
/// Calling this 3 value x, y and z, and the new value soft_min then this function compute
/// soft_min = -gamma * ln(exp(-x/gamma) + exp(-y/gamma) + exp(-z/gamma))
/// where gamme is a non negative hyperparameter
fn compute_soft_min_optimized(upper_value : f64, upper_left_value : f64, left_value : f64, gamma : f64) -> f64{
    let e_uv = (-upper_value/gamma).exp();
    let e_ulv = (-upper_left_value/gamma).exp();
    let e_lv = (-left_value/gamma).exp();

    // println!("e_uv  = {}", e_uv);
    // println!("e_ulv = {}", e_ulv);
    // println!("e_lv  = {}\n", e_lv);
    //
    // println!("uv  = {}", upper_value);
    // println!("ulv = {}", upper_left_value);
    // println!("lv  = {}", left_value);
    // println!("- - - - -- - -");

    if e_uv == 0. && e_ulv == 0. && e_lv == 0. {
        return min_of_three_values(upper_value, upper_left_value, left_value)
    } else {
        return  -gamma * (e_uv + e_ulv + e_lv).ln()
    }

}

fn compute_soft_min_general(values_array : &[f64], gamma : f64) -> f64 {
    let mut soft_min = 0.0;
    for &value in values_array.iter(){
        soft_min += (-(value / gamma)).exp();
    }

    return -gamma * soft_min.ln()
}

fn min_of_three_values(v_1 : f64, v_2 : f64, v_3 : f64) -> f64{
    if v_1 <= v_2 {
        if v_1 <=v_3 { return v_1 }
        else { return v_3}
    } else {
        if v_2 <=v_3 { return v_2 }
        else { return v_3}
    }
}

// - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - 
//
pub fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
