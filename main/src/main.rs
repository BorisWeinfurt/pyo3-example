mod custom_machine;
mod machine;

use machine::{StandardMachine, MachineTrait, METHODS};
use custom_machine::CustomMachine;
use pyo3::prelude::*;
use std::fs;

#[pyfunction]
fn inspect_module_functions(py: Python, module: &PyModule) -> PyResult<()> {
    let dir = py.import("builtins")?.getattr("dir")?;
    let inspect = py.import("inspect")?;
    let signature = inspect.getattr("signature")?;

    let module_functions: Vec<String> = dir.call1((module,))?.extract()?;
    
    println!("Rust trait methods:");
    for method in METHODS {
        println!("{}", method);
    }

    print!("\n\nDeclared python functions");
    for func_name in module_functions {
        let func = module.getattr(&func_name)?;
        if let Ok(sig) = signature.call1((func,)) {
            println!("{}: {}", func_name, sig);
        }
    }
    // You could check whether the python function signatures match the trait signatures here
    Ok(())
}

fn main() {
    pyo3::prepare_freethreaded_python();
    let context = fs::read_to_string("main/src/myCustomImplementation.py")
        .expect("Should have been able to read file");
    let mut custom_python_machine: Option<CustomMachine> = None;

    // Acquire python interpreter lock
    Python::with_gil(|py| {

        // Load python code
        let module = PyModule::from_code(
            py,
            &context,
            "myCustomImlementation.py",
            "myCustomImlementation",
        )
        .expect("Shoudl have been able to load code");

        // Compare methods existence/signatures
        if inspect_module_functions(py, module).is_ok() {
            custom_python_machine = Some(CustomMachine::new(
                module
                    .getattr("required_method1")
                    .expect("no error handling")
                    .into(),
                module.getattr("required_method2").expect("no error handling").into(),
            ));
        } else {
            panic!("Custom implementation doesnt match machine trait signature");
        }
    });

    // At a much later time disconnected from the creation you can now use both
    // types of machines interchangably
    let custom_python_machine = custom_python_machine.unwrap();
    let regular_machine = StandardMachine::new();

    println!("\n\nMachine methods are being called");

    custom_python_machine.required_method1(15, "arg2".to_string());
    regular_machine.required_method1(15, "arg2".to_string());
}
