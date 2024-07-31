"""
How to use soft_dtw_rust module
"""

import soft_dtw_rust

import numpy as np
import time

# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

size_list = [10, 100, 1000]
n_ch = 22

gamma = .1

# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -
print("TEST 1D")

for i in range(len(size_list)):
    size = size_list[i]

    x = np.random.rand(size)
    y = np.random.rand(size)

    start = time.time()
    output = soft_dtw_rust.compute_sdtw_1d(x, y, gamma)
    end = time.time()

    print("Test with {} samples".format(size))
    print("\tTime = {:.8f}".format(end - start))
    print("\tOUTPUT : {}".format(output))

# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

print("TEST 2D")

for i in range(len(size_list)):
    size = size_list[i]

    x = np.random.rand(n_ch, size)
    y = np.random.rand(n_ch, size)

    start = time.time()
    output = soft_dtw_rust.compute_sdtw_2d(x, y, gamma)
    end = time.time()

    print("Test with {} samples".format(size))
    print("\tTime = {:.8f}".format(end - start))
    print("\tOUTPUT : {}".format(output))
