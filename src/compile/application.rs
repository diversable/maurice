use dialoguer::{console::Term, theme::ColorfulTheme, Input};
use jlrs::prelude::*;
use std::env::current_dir;
use std::path::PathBuf;

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
                    .submodule(&mut frame, "Maurice")?
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
                    // The function that was called returns a String
                    .unbox::<String>()
            }
        })
        .expect("Result is an error");

    println!("Result: {:?}", app.expect("compile app failed"));
}

pub fn get_app_source_path() -> String {
    let app_source_path = Input::<String>::new()
        .with_prompt("Please enter the (relative or absolute) path to the Julia source code you want to compile into an app")
        .interact()
        .expect("Must provide a path to the source Julia code you want to compile into an app!");
    app_source_path
}

pub fn get_app_compile_target_path() -> String {
    let current_dir = current_dir().expect("couldn't parse current directory");
    let compiled_path_segment = PathBuf::from("./compiled");

    let mut compiled_path = PathBuf::new();
    compiled_path.push(current_dir);
    compiled_path.push(compiled_path_segment);

    let default_path = compiled_path
        .to_str()
        .expect("could not construct string from PathBuf")
        .to_string();

    let compiled_app_path = Input::<String>::new()
        .with_prompt("Please enter a path for where your compiled app should be output")
        .interact()
        .unwrap_or(default_path);

    compiled_app_path
}
