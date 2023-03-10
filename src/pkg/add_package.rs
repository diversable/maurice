use jlrs::prelude::*;

pub fn add_one_package(julia: &mut Julia, package: &str) {
    println!("\nAdding \"{}\" to installed packages\n", &package);

    let package = package.to_string();
    let add = julia
        .scope(|mut frame| {
            let jl_module_main = Module::main(&mut frame);

            let pkg = JuliaString::new(&mut frame, package);

            unsafe {
                jl_module_main
                    // the submodule doesn't have to be rooted because it's never reloaded.
                    .submodule(&mut frame, "Maurice")?
                    .submodule(&mut frame, "PkgAPI")?
                    // the same holds true for the function: the module is never reloaded so it's globally rooted
                    .function(&mut frame, "add_package")?
                    //
                    // CALLING A FUNCTION
                    //
                    // Call the function with the two arguments it takes
                    // .call2(&mut frame, dim, iters)
                    //
                    // .call0(&mut frame)
                    //
                    .call1(&mut frame, pkg.as_value())
                    //
                    // If you don't want to use the exception, it can be converted to a `JlrsError`
                    // In this case the error message will contain the message that calling `display` in Julia would show
                    .into_jlrs_result()?
                    // The function that was called returns a `Cint`, which can be unboxed as `i32`
                    // .unbox()::<i32>()
                    //
                    // unbox a function return type from Julia which is `Nothing`
                    // .unbox::<Nothing>()
                    .unbox::<String>()
            }
        })
        .expect("Result is an error");

    println!("Result: {:?}", add.expect("add package via Julia failed"));
}
