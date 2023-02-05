use jlrs::prelude::*;

pub fn compile_app(julia: &mut Julia, source_code_path: &str, target_directory_path: &str) {
    println!(
        "Compiling \"{}\" to the '{}' directory\n",
        &source_code_path, &target_directory_path
    );

    let source_code_path = source_code_path.to_string();
    let target_directory_path = target_directory_path.to_string();

    let app = julia
        .scope(|mut frame| {
            let jl_module_main = Module::main(&mut frame);

            let source_code_path = JuliaString::new(&mut frame, source_code_path);
            let target_directory_path = JuliaString::new(&mut frame, target_directory_path);

            unsafe {
                jl_module_main
                    // the submodule doesn't have to be rooted because it's never reloaded.
                    .submodule(&mut frame, "Gaston")?
                    .submodule(&mut frame, "Create")?
                    // the same holds true for the function: the module is never reloaded so it's globally rooted
                    .function(&mut frame, "compile_app")?
                    //
                    // CALLING A FUNCTION
                    //
                    // Call the function with the two arguments it requires
                    //
                    .call2(
                        &mut frame,
                        source_code_path.as_value(),
                        target_directory_path.as_value(),
                    )
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

    println!("Result: {:?}", app.expect("compile app failed"));
}
