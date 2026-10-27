use pyo3::pymodule;

use pyo3::prelude::*;

#[pymodule]
pub mod np {
    use pyo3::prelude::*;
    use pyo3::exceptions::PyTypeError;
    use pyo3::{pyclass, BoundObject, IntoPyObjectExt};

    use pyo3::types::PyList;

    #[pyclass(skip_from_py_object)]
    #[derive(Clone, Eq, PartialEq)]
    pub struct Np {
        arr: Vec<i32>,
        nd: usize,
        shape: Vec<usize>
    }
    #[pymethods]
    impl Np {
        #[new]
        pub fn new(array: &Bound<'_, PyList>) -> PyResult<Self> {
            if array.len() == 0 {
                return Err(PyErr::new::<PyTypeError, _>("Array must have at least one element"));
            }
            let nd = Self::get_dim(array);
            let shape = Self::compute_shape(array);
            let flat = Self::flatten(array)?;
            Ok(Np {
                arr: flat,
                nd,
                shape
            })
        }
        pub fn __str__(&self) -> String {
            // NumPy pads every element to the width of the longest one
            let w = self.arr.iter().map(|v| v.to_string().len()).max().unwrap_or(1);
            self.fmt(0, 0, w)
        }

        pub fn __repr__(&self) -> String {
            self.__str__()
        }
        pub fn __add__(&self, other: &Np) -> PyResult<Self> {
            if self.shape != other.shape {
                return Err(PyErr::new::<PyTypeError, _>("mismatched shape"));
            }
            let mut n: Vec<i32> = Vec::new();
            for i in 0..self.arr.len() {
                n.push(self.arr[i] + other.arr[i]);
            }
            Ok(Self {
                arr: n,
                nd: self.nd,
                shape: self.shape.clone()
            })
        }
        pub fn __mul__(&self, other: &Np) -> PyResult<Self> {
            if self.shape != other.shape {
                return Err(PyErr::new::<PyTypeError, _>("mismatched shape"));
            }
            let mut n: Vec<i32> = Vec::new();
            for i in 0..self.arr.len() {
                n.push(self.arr[i] * other.arr[i])
            }
            Ok(Self {
                arr: n,
                nd: self.nd,
                shape: self.shape.clone()
            })
        }
        /// currently only for 1d or 2d arrays
        pub fn __matmul__(&self, other: &Np, py:Python<'_>) -> PyResult<Py<PyAny>> {
            if self.nd > 2 || other.nd > 2 {
                return Err(PyErr::new::<PyTypeError, _>("Too many dimensions"));
            }

            if self.nd == 1 && other.nd == 1 {
                return (self.matmul_1x1(other).into_py_any(py));
            }
            if self.shape[0] == other.shape[1]{
                return self.matmul_o(other).into_py_any(py);
            }
            Err(PyErr::new::<PyTypeError, _>("matrix multiplication failed"))





        }
    }

    impl Np {
        fn compute_shape(array: &Bound<'_, PyList>) -> Vec<usize> {
            let mut shape: Vec<usize> = Vec::new();
            let mut curr = array.clone().into_any();
            while let Ok(list) = curr.cast::<PyList>() {
                shape.push(list.len());
                match list.iter().next() {
                    Some(first) => curr = first,
                    None => break
                }
            }
            shape
        }
        fn get_dim(array: &Bound<'_, PyList>) -> usize {
            //* getting the number of dimensions of the list
            let mut nd = 0;
            let mut curr = array.clone().into_any();
            while let Ok(list) = curr.cast::<PyList>() {
                nd += 1;
                match list.iter().next() {
                    Some(first) => curr = first,
                    None => break
                }
            }
            nd
        }


        fn flatten_into(obj: &Bound<'_, PyAny>, out: &mut Vec<i32>) -> PyResult<()> {
            match obj.cast::<PyList>() {          // `downcast` on PyO3 < 0.27
                Ok(list) => {
                    for item in list.iter() {
                        Self::flatten_into(&item, out)?;
                    }
                    Ok(())
                }
                Err(_) => {
                    out.push(obj.extract::<i32>()?);
                    Ok(())
                }
            }
        }

        fn flatten(list: &Bound<'_, PyList>) -> PyResult<Vec<i32>> {
            let mut out = Vec::new();
            Self::flatten_into(list.as_any(), &mut out)?;
            Ok(out)
        }
        fn fmt(&self, dim: usize, start: usize, w: usize) -> String {
            let n = self.shape[dim];
            let stride: usize = self.shape[dim + 1..].iter().product();
            let last = dim + 1 == self.shape.len();

            let parts: Vec<String> = (0..n)
                .map(|i| {
                    let off = start + i * stride;
                    if last {
                        format!("{:>w$}", self.arr[off], w = w)
                    } else {
                        self.fmt(dim + 1, off, w)
                    }
                })
                .collect();

            if last {
                format!("[{}]", parts.join(" "))
            } else {
                // one newline per remaining dimension, indented by depth
                let sep = format!(
                    "{}{}",
                    "\n".repeat(self.shape.len() - dim - 1),
                    " ".repeat(dim + 1)
                );
                format!("[{}]", parts.join(&sep))
            }
        }
        fn matmul_1x1(&self,other: &Np) -> i32 {
            let mut sum = 0;
            for i in 0..self.arr.len() {
                sum += self.arr[i] * other.arr[i];
            }
            sum

        }

        fn matmul_o(&self,other: &Np)  {


        }

    }

}

