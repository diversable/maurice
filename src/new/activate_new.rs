use jlrs::prelude::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;

// TODO! Create Julia environments' (scripts') `Main.jl` file & write standard content into it...
fn create_main_jl() -> std::io::Result<()> {
    // TODO!
    // Get current directory

    // write `Main.jl` to current dir
    let mut jl_main_file = File::create("Main.jl")?;

    // ...
    unimplemented!();
}

pub fn activate_env_in_current_dir(julia: &mut Julia) {
    println!(
        "\nActivating environment {:?}\n",
        env::current_dir().expect("couldn't retrieve current directory")
    );

    // TODO! Call into create_main_jl() fn to create a standard Main.jl file for the env.

    let _activate = julia
        .scope(|mut frame| {
            let jl_module_main = Module::main(&mut frame);

            unsafe {
                jl_module_main
                    // the submodule doesn't have to be rooted because it's never reloaded.
                    .submodule(&mut frame, "Gaston")?
                    .submodule(&mut frame, "PkgAPI")?
                    // the same holds true for the function: the module is never reloaded so it's globally rooted
                    .function(&mut frame, "make_project_in_current_dir")?
                    //
                    // CALLING A FUNCTION
                    //
                    // Call the function with the two arguments it takes
                    // .call2(&mut frame, dim, iters)
                    //
                    // .call0(&mut frame)
                    //
                    .call0(&mut frame)
                    //
                    // If you don't want to use the exception, it can be converted to a `JlrsError`
                    // In this case the error message will contain the message that calling `display` in Julia would show
                    .into_jlrs_result()?
                    // The function that was called returns a `Cint`, which can be unboxed as `i32`
                    // .unbox()::<i32>()
                    //
                    //
                    // unbox a function where the return type from Julia which is `Nothing`
                    .unbox::<Nothing>()
            }
        })
        .expect("Result is an error");

    println!(
        "\nActivated new project environment in {:?}.\nYou're ready to add packages!",
        env::current_dir().expect("couldn't retrieve current directory")
    );
}

pub fn activate_env_w_name(julia: &mut Julia, env_name: &str) {
    println!("\nActivating environment \"{}\"\n", &env_name);

    // TODO! Call into create_main_jl() fn to create a standard Main.jl file for the env.

    let env_name = env_name.to_string();
    let activate = julia
        .scope(|mut frame| {
            let jl_module_main = Module::main(&mut frame);

            let environment = JuliaString::new(&mut frame, env_name);

            unsafe {
                jl_module_main
                    // the submodule doesn't have to be rooted because it's never reloaded.
                    .submodule(&mut frame, "Gaston")?
                    .submodule(&mut frame, "PkgAPI")?
                    // the same holds true for the function: the module is never reloaded so it's globally rooted
                    .function(&mut frame, "activate_environment")?
                    //
                    // CALLING A FUNCTION
                    //
                    // Call the function with the two arguments it takes
                    // .call2(&mut frame, dim, iters)
                    //
                    // .call0(&mut frame)
                    //
                    .call1(&mut frame, environment.as_value())
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

    println!(
        "\nActivated new project environment\n{:?}",
        activate.unwrap()
    );
}
