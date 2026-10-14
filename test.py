

import pytest
import numpy as nnp
from pylib import np  # <-- change "your_module_name" to your compiled module

a = [[1,2,3],[4,5,6],[7,8,9]]

c = nnp.array(a)
b = np.Np(a)
def test_smt():
    assert str(b) == str(c)
def test_add():
    d= [
        [1,0,0],
        [0,1,0],
        [0,0,1]
    ]
    e = np.Np(d)
    f = nnp.array(d)
    print(b+e)
    print(f+c)
    assert str(b+e)  == str(f+c)
