# pylib

A small NumPy-style array library for Python, written in Rust with [PyO3](https://pyo3.rs).

> ⚠️ **Work in progress.** This is a passion project I'm building for fun and to learn. Expect missing features, rough edges, and breaking changes. It is not production-ready, so please don't depend on it for anything important.

## What is this?

`pylib` is an experiment in writing a NumPy-like library from scratch. The core is implemented in Rust and compiled into a native Python extension module, so it can be imported from Python like any other package.

The goal isn't to replace NumPy. It's to understand how array libraries work under the hood, and to have fun with Rust and Python interop along the way.

## Current features

- Create a 2D array from nested Python lists
- Print arrays
- Element-wise addition with `+`

More is on the way (see the roadmap below).

## Usage

```python
from pylib import np

a = np.Np([[1, 2, 3],
           [4, 5, 6],
           [7, 8, 9]])

b = np.Np([[1, 0, 0],
           [0, 1, 0],
           [0, 0, 1]])

print(a + b)
```

## Building from source

You'll need [Rust](https://www.rust-lang.org/tools/install) and Python 3.

```bash
git clone https://github.com/blazermega/pylib.git
cd pylib

# build and install the extension into your current Python environment
pip install maturin
maturin develop
```

## Running the tests

The tests compare `pylib`'s output against NumPy, so you'll need `pytest` and `numpy` installed.

```bash
pip install pytest numpy
pytest test.py
```

## Roadmap

- [ ] More operators (subtraction, multiplication, division)
- [ ] Matrix multiplication
- [ ] Indexing and slicing
- [ ] Shape and dtype support
- [ ] Better error handling
- [ ] Documentation and benchmarks

## Contributing

This is mainly a personal learning project, but issues, ideas, and suggestions are welcome. Just keep in mind that things may change quickly.

