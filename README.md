# Soft-DTW Rust
**EXPERIMENTAL** implementation of the [Soft-DTW](https://arxiv.org/abs/1703.01541) algorithm in Rust. The project is built with [maturin](https://www.maturin.rs/) so it can be used also with Python.

Note that this is primarily an exercise to learn how to code in Rust and integrate Rust scripts in Python. I'm not pretending that this is 100% functional or correct. 
This implementation is not created to be used as a loss function. It is designed only to compute the soft-dtw distance between two signals. Also for now works only if the signals to compare have the same length. In future I could modify the algorithm to work with signals of different lengths. 
The distance between samples of the two signals is computed through the L-2 norm.

## Installation
### Method 1
Use the command ```pip install soft-dtw-rust```

### Method 2

1) Download the repository and install maturin in your python environment (it is available both in conda and pip).
2) Install the package in your python environment running the command ```maturin develop --release```.
3) Import the module ```soft_dtw_rust``` and call one of the two methods `compute_sdtw_1d` or `compute_sdtw_2d`. For both methods, you can pass the hyperparameter `gamma`. If you not pass it by default will be set to 1.

## Use the library
Import the package `soft_dtw_rust` and call the function `compute_sdtw_1d` or `compute_sdtw_2d`. The differences between 1d and 2d version is described below.

### Example with code
```python
import soft_dtw_rust
x = np.random.rand(size)
y = np.random.rand(size)
output = soft_dtw_rust.compute_sdtw_1d(x, y, gamma)
```

A small example with synthetic data can be found inside the example folder.

### Extra notes on 1d and 2d version
Note that `compute_sdtw_1d` is used to compute the difference between two 1d signals and return a single float 64 value.

Instead `compute_sdtw_2d` is used to compute the difference between two matrices of signals. Also in this case the output is a single float 64 value. 
The matrices must have the same shape and each row must contain a signal (an example could be a multi-channel EEG signal). In this case, the algorithm use the soft-dtw to compare the signal in the first row of the first matrix with the signal in the first row of the second matrix and so on. 
The output of the various comparisons is summed and used as the final output of the function. 
Optionally you could pass an extra parameter called `average_along_channels`. As the name suggests, in this case, the final output will be divided by the number of signals inside the matrices.
