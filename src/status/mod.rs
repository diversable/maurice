use jlrs::prelude::*;

/// get the current status of the packages in the current Julia environment
pub fn status(julia: &mut Julia) {
    println!("\nGetting the status of your packages....\n",);

    let status = julia
        .scope(|mut frame| {
            // let dim = Value::new(&mut frame, 4isize);
            // let iters = Value::new(&mut frame, 1_000_000isize);

            let jl_module_main = Module::main(&mut frame);
            unsafe {
                jl_module_main
                    // the submodule doesn't have to be rooted because it's never reloaded.
                    .submodule(&mut frame, "Gaston")?
                    // the same holds true for the function: the module is never reloaded so it's globally rooted
                    .function(&mut frame, "status")?
                    //
                    // CALLING A FUNCTION
                    //
                    // Call the function with the two arguments it takes
                    // .call2(&mut frame, dim, iters)
                    .call0(&mut frame)
                    //
                    // If you don't want to use the exception, it can be converted to a `JlrsError`
                    // In this case the error message will contain the message that calling
                    // `display` in Julia would show
                    .into_jlrs_result()?
                    // The function that was called returns a `Cint`, which can be unboxed as `i32`
                    // .unbox()::<i32>()
                    //
                    // unbox a function return type from Julia which is `Nothing`
                    // .unbox::<Nothing>()
                    // change return type to String
                    .unbox::<String>()
            }
        })
        .expect("Result is an error");

    println!("\nStatus: {:?}", status.unwrap());
}
