use std::sync::Arc;

use crate::machine::MachineTrait;
use pyo3::prelude::*;

// Custom python machine, could be generalized to any language
pub struct CustomMachine {
    required_method1 : Arc<PyObject>,
    required_method2 : Arc<PyObject>,
}

impl CustomMachine {
    pub fn new(required_method1 : PyObject, required_method2 : PyObject) -> Self {
        Self {
            required_method1: Arc::new(required_method1),
            required_method2: Arc::new(required_method2),
        }
    }
}

impl MachineTrait for CustomMachine {
    fn required_method1(&self, arg1: i32, arg2: String) -> bool {
        Python::with_gil(|py| {
            self.required_method1
                .call1(py, (arg1, arg2)).expect("failed function call")
                .extract(py).expect("no error handling")
        })
    }

    fn required_method2(&self, arg1: String, arg2: bool) {
        unimplemented!()
    }
}
