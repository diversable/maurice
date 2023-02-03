use jlrs::prelude::*;

/// install Pluto Notebooks in the user's global environment
pub fn install_or_update_pluto(julia: &mut Julia) {
    println!("\nEnsuring you've got the latest Pluto Notebooks environment...\n",);

    let status = julia
        .scope(|mut frame| {
            // let dim = Value::new(&mut frame, 4isize);
            // let iters = Value::new(&mut frame, 1_000_000isize);

            let jl_module_main = Module::main(&mut frame);
            unsafe {
                jl_module_main
                    // the submodule doesn't have to be rooted because it's never reloaded.
                    .submodule(&mut frame, "Gaston")?
                    .submodule(&mut frame, "Jl_Command")?
                    // the same holds true for the function: the module is never reloaded so it's globally rooted
                    .function(&mut frame, "install_or_update_pluto_nb")?
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

    println!(
        "\nStatus: {:?}",
        status.expect("installing Pluto Notebook failed...")
    );
}
