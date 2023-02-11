use jlrs::prelude::*;

/// Run tests from user's project's `test` directory; ie. run the `runtests.jl` file
pub fn run_tests(julia: &mut Julia) {
    let status = julia
        .scope(|mut frame| {
            let jl_module_main = Module::main(&mut frame);
            unsafe {
                jl_module_main
                    // the submodule doesn't have to be rooted because it's never reloaded.
                    .submodule(&mut frame, "Maurice")?
                    .submodule(&mut frame, "Test_Command")?
                    // the same holds true for the function: the module is never reloaded so it's globally rooted
                    .function(&mut frame, "run_tests")?
                    //
                    // CALLING A FUNCTION
                    // use the `call0(&mut frame)` function to call a fn with no args
                    .call0(&mut frame)
                    // Get the result of the function call
                    .into_jlrs_result()?
                    // return type is String
                    .unbox::<String>()
            }
        })
        .expect("Result is an error");

    println!("\nStatus: {:?}", status.expect("running tests failed..."));
}
