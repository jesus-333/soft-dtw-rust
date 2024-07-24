import soft_dtw_rust

import numpy as np

# - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - - -

x = np.random.rand(20)
y = np.random.rand(20)

x = np.linspace(1, 10, 10, dtype='float32')
y = np.linspace(1, 20, 10, dtype='float32')

# print("The distance between {} and {} is {}".format(x, y, soft_dtw_rust.compute_sdtw_1d(x, y)))
print("OUTPUT : {}".format(soft_dtw_rust.compute_sdtw_1d(x, y)))
