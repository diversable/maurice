use anyhow::{anyhow, Context, Ok, Result};
use capitalize::Capitalize;
use dialoguer::{console::Term, theme::ColorfulTheme, Input, Select};
use jlrs::prelude::*;
use std::env::{self};
use std::fs::{DirBuilder, File};
use std::io::prelude::*;
use std::path::{self, PathBuf};

const JL_RUNTESTS_CONTENTS_1: &str = r###"module Test
using Test
using "###;
const JL_RUNTESTS_CONTENTS_2: &str = r###": greet
# write tests here...
Test.@test greet() == print("Hello World!")

end # module Test
"###;

pub fn new_script_ask_name(julia: &mut Julia) {
    let input_script_name: String = Input::new()
        .with_prompt("What name would you like to give your script?")
        .interact_text()
        // .unwrap_or("Main".to_string());
        .unwrap();

    let script_name = input_script_name;
    new_script_w_name(julia, script_name.as_str());
}

// TODO! create default files unless ./src/Main.jl & /tests/run_tests.jl files exist
pub fn new_script_w_name(julia: &mut Julia, script_name: &str) {
    // Ensure script names are capitalized, as per standard Julia practice
    let script_name = script_name.to_string().capitalize();

    // Option 1: fail on directory exists
    let script = generate_script(julia, &script_name);

    // println!("script result is: {:?}", script);

    if script.is_ok() {
        // Run PostHook
        new_script_posthook(julia, &script_name).expect("Couldn't run user posthook");
    }

    // Option 2: check and attempt to force script creation in pre-existing directory.
    // check if directory already exists: if so, ask if you want to continue (y / n)...
    // TODO: hook into Pkg at a lower level, or make a pull request asking to enable "force" functionality for creating a script in a pre-existinig directory..
    // let mut script_path = PathBuf::new();
    // let current_dir = env::current_dir().expect("couldn't get current directory");
    // script_path.push(current_dir);
    // script_path.push(&script_name);

    // if script_path.exists() {
    //     let dir_exists_options = vec!["yes", "no"];
    //     let check_to_continue_dir_exists_selection = Select::with_theme(&ColorfulTheme::default())
    //         .with_prompt("Directory already exists; would you like to continue?")
    //         .items(&dir_exists_options)
    //         .default(1)
    //         .interact()
    //         .expect("couldn't get your answer");

    //     println!(
    //         "I got your answer: {:?}",
    //         check_to_continue_dir_exists_selection
    //     );
    //     // if yes..
    //     if check_to_continue_dir_exists_selection == 0 {
    //         generate_script(julia, &script_name).expect("Couldn't generate script")
    //     } else {
    //     // if no...
    //         println!("Couldn't generate script for you :(")
    //     }
    // } else {
    //     generate_script(julia, &script_name).expect("Couldn't generate script")
    // }
}

fn generate_script(julia: &mut Julia, script_name: &String) -> Result<()> {
    let new_script = julia.scope(|mut frame| {
        let jl_module_main = Module::main(&mut frame);

        let script_name = JuliaString::new(&mut frame, &script_name);

        unsafe {
            jl_module_main
                // the submodule doesn't have to be rooted because it's never reloaded.
                .submodule(&mut frame, "Maurice")?
                .submodule(&mut frame, "New")?
                // the same holds true for the function: the module is never reloaded so it's globally rooted
                .function(&mut frame, "new_script_in_target_dir")?
                //
                // CALLING A FUNCTION
                //
                // Call the function with the target Julia frame and 1 argument
                .call1(&mut frame, script_name.as_value())
                //
                // If you don't want to use the exception, it can be converted to a `JlrsError`
                // In this case the error message will contain the message that calling `display` in Julia would show
                .into_jlrs_result()?
                .unbox::<i64>()
        }
    })?;

    match new_script {
        0 => {
            let current_dir = env::current_dir().expect("couldn't get current directory");

            let mut tests_dir_path = path::PathBuf::new();
            tests_dir_path.push(&current_dir);
            tests_dir_path.push("./test");

            DirBuilder::new()
                .recursive(true)
                .create(&tests_dir_path)
                .expect("Could not create `test` directory");

            let mut tests_file_path = path::PathBuf::new();
            tests_file_path.push(tests_dir_path);
            tests_file_path.push("./runtests.jl");
            let mut jl_runtests_file =
                File::create(&tests_file_path).expect("could not create runtests.jl file");

            write!(
                jl_runtests_file,
                "{}{}{}",
                JL_RUNTESTS_CONTENTS_1, &script_name, JL_RUNTESTS_CONTENTS_2
            )
            .expect("Could not write test file contents");

            Ok(())
        }
        1 => return Err(anyhow!("creating new script failed")),
        _ => return Err(anyhow!("Error parsing return value")),
    }
}

fn new_script_posthook(julia: &mut Julia, script_name: &String) -> Result<()> {
    let posthook = julia.scope(|mut frame| {
        let jl_module_main = Module::main(&mut frame);

        let script_name = JuliaString::new(&mut frame, &script_name);

        unsafe {
            jl_module_main
                // the submodule doesn't have to be rooted because it's never reloaded.
                .submodule(&mut frame, "Maurice")?
                .submodule(&mut frame, "Hooks")?
                // the same holds true for the function: the module is never reloaded so it's globally rooted
                .function(&mut frame, "new_script_posthook")?
                //
                // CALLING A FUNCTION
                //
                // Call the function with the target Julia frame and 1 argument
                .call1(&mut frame, script_name.as_value())
                //
                // If you don't want to use the exception, it can be converted to a `JlrsError`
                // In this case the error message will contain the message that calling `display` in Julia would show
                .into_jlrs_result()?
                .unbox::<i32>()
        }
    })?;
    // .expect("Result is an error");

    match posthook {
        0 => Ok(()),
        1 => {
            return Err(anyhow!("Could not execute user posthook: new script"));
        }
        _ => return Err(anyhow!("Error parsing return value: new script")),
    }
}
