use capitalize::Capitalize;
use dialoguer::{console::Term, theme::ColorfulTheme, Input};
use jlrs::prelude::*;
use std::env::{self};
use std::fs::{DirBuilder, File};
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
fn create_default_files_for_script() -> std::io::Result<()> {
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

pub fn new_script_ask_name(julia: &mut Julia) {
    let input_script_name: String = Input::new()
        .with_prompt("What name would you like to give your script?")
        .interact_text()
        .unwrap_or("Main".to_string());

    let script_name = input_script_name.capitalize();
    new_script_w_name(julia, script_name.as_str());
}

// TODO! create default files unless ./src/Main.jl & /tests/run_tests.jl files exist
pub fn new_script_w_name(julia: &mut Julia, script_name: &str) {
    println!("\nActivating environment \"{}\"\n", &script_name);

    let script_name = script_name.to_string();
    let activate = julia
        .scope(|mut frame| {
            let jl_module_main = Module::main(&mut frame);

            let script_name = JuliaString::new(&mut frame, &script_name);

            unsafe {
                jl_module_main
                    // the submodule doesn't have to be rooted because it's never reloaded.
                    .submodule(&mut frame, "Gaston")?
                    .submodule(&mut frame, "New")?
                    // the same holds true for the function: the module is never reloaded so it's globally rooted
                    .function(&mut frame, "activate_script_in_target_dir")?
                    //
                    // CALLING A FUNCTION
                    //
                    // Call the function with the target Julia frame and 1 argument
                    .call1(&mut frame, script_name.as_value())
                    //
                    // If you don't want to use the exception, it can be converted to a `JlrsError`
                    // In this case the error message will contain the message that calling `display` in Julia would show
                    .into_jlrs_result()?
                    .unbox::<String>()
            }
        })
        .expect("Result is an error");

    // TODO! Change the create_default.... fn to handle a target directory for this funcion!
    // create_default_files_for_env().expect("Couldn't write default files for the environment");

    let current_dir = env::current_dir().expect("couldn't get current directory");

    let mut tests_dir_path = path::PathBuf::new();
    tests_dir_path.push(&current_dir);
    tests_dir_path.push("./tests");

    DirBuilder::new()
        .recursive(true)
        .create(&tests_dir_path)
        .expect("Could not create `tests` directory");

    let mut tests_file_path = path::PathBuf::new();
    tests_file_path.push(tests_dir_path);
    tests_file_path.push("./run_tests.jl");
    let mut jl_runtests_file =
        File::create(&tests_file_path).expect("could not create runtests.jl file");

    write!(jl_runtests_file, "{}", JL_RUNTESTS_CONTENTS)
        .expect("Could not write test file contents");

    println!("\n{:?}", activate.unwrap());
}
