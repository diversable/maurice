use jlrs::prelude::*;
use std::env::{self, current_dir};
use std::fs::{self, DirBuilder, File};
use std::io::prelude::*;
use std::path;

const JL_SCRIPT_CONTENTS: &str = r###"module Main

function main()
    println("Hello, world!")
end

main()

end # module Main
"###;

const JL_RUNTESTS_CONTENTS: &str = r###"module Test

# write tests here...

end # module Test
"###;

// TODO! Change this function to handle setting up files in a target directory
fn create_default_files_for_env() -> std::io::Result<()> {
    // Check to ensure that you're not going to over-write any pre-existing user content...
    let src_main_file = path::PathBuf::from("./src/Main.jl");
    if src_main_file.exists() {
        println!(
            "\nActivated environment in {:?}.\nYou're ready to add packages!",
            env::current_dir().expect("couldn't retrieve current directory")
        );
    } else {
        let current_dir = env::current_dir()?;

        // Create ./src directory
        let mut src_path = path::PathBuf::new();
        src_path.push(&current_dir);
        src_path.push("src/");

        DirBuilder::new()
            .recursive(true)
            .create(src_path)
            .expect("Could not create `/src/` directory");

        // write `Main.jl` to current_dir/src/Main.jl
        let mut jl_main_file =
            File::create("./src/Main.jl").expect("could not create ./src/Main.jl file");

        // write JL_SCRIPT_CONTENTS to the julia main file
        write!(jl_main_file, "{}", JL_SCRIPT_CONTENTS)?;

        let mut tests_path = path::PathBuf::new();
        tests_path.push(&current_dir);
        tests_path.push("./tests/");

        DirBuilder::new()
            .recursive(true)
            .create(tests_path)
            .expect("Could not create `/tests/` directory");

        let mut jl_runtests_file = File::create("./tests/run_tests.jl")
            .expect("could not create `./tests/run_tests.jl` file");

        write!(jl_runtests_file, "{}", JL_RUNTESTS_CONTENTS)?;

        println!(
            "\nActivated new project environment and files in {:?}.\nYou're ready to add packages!",
            env::current_dir().expect("couldn't retrieve current directory")
        );
    }

    Ok(())
}

pub fn activate_env_in_current_dir(julia: &mut Julia) {
    println!(
        "\nActivating environment {:?}\n",
        env::current_dir().expect("couldn't retrieve current directory")
    );

    let _activate = julia
        .scope(|mut frame| {
            let jl_module_main = Module::main(&mut frame);

            unsafe {
                jl_module_main
                    // the submodule doesn't have to be rooted because it's never reloaded.
                    .submodule(&mut frame, "Gaston")?
                    .submodule(&mut frame, "New")?
                    // the same holds true for the function: the module is never reloaded so it's globally rooted
                    .function(&mut frame, "make_env_in_current_dir")?
                    //
                    // CALLING A FUNCTION
                    //
                    // Call the function with the target Julia frame
                    .call0(&mut frame)
                    //
                    // If you don't want to use the exception, it can be converted to a `JlrsError`
                    // In this case the error message will contain the message that calling `display` in Julia would show
                    .into_jlrs_result()?
                    //
                    // If the Julia function that was called returns a `Cint`, then it can be unboxed on the Rust side as an `i32`
                    // .unbox()::<i32>()
                    //
                    //
                    // Here, unbox a function where the return type from Julia which is `Nothing`
                    .unbox::<Nothing>()
            }
        })
        .expect("Result is an error");

    create_default_files_for_env().expect("Couldn't write default files for the environment");
}

// TODO! create default files unless ./src/Main.jl & /tests/run_tests.jl files exist
pub fn activate_env_w_name(julia: &mut Julia, env_name: &str) {
    println!("\nActivating environment \"{}\"\n", &env_name);

    let env_name = env_name.to_string();
    let activate = julia
        .scope(|mut frame| {
            let jl_module_main = Module::main(&mut frame);

            let environment = JuliaString::new(&mut frame, env_name);

            unsafe {
                jl_module_main
                    // the submodule doesn't have to be rooted because it's never reloaded.
                    .submodule(&mut frame, "Gaston")?
                    .submodule(&mut frame, "New")?
                    // the same holds true for the function: the module is never reloaded so it's globally rooted
                    .function(&mut frame, "activate_env_in_target_dir")?
                    //
                    // CALLING A FUNCTION
                    //
                    // Call the function with the target Julia frame and 1 argument
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

    // TODO! Change the create_default.... fn to handle a target directory for this funcion!
    // create_default_files_for_env().expect("Couldn't write default files for the environment");

    println!(
        "\nActivated new project environment\n{:?}",
        activate.unwrap()
    );
}
