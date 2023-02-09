// using PkgTemplates to generate packages....
// install Documenter & DocumenterTools

use jlrs::prelude::*;

// use Rs crate Dialoguer to get the project name from the CLI user, if not supplied in command line args - it's required for generating docs properly!

// use DocumenterTools.generate() to create docs for projects. See the guide to the Documenter.jl pkg for more info...

// install PackageCompiler in Env
// install:
// Documenter
// DocumenterTools
//

use capitalize::Capitalize;
use dialoguer::{console::Term, theme::ColorfulTheme, Input};
use jlrs::prelude::*;
use std::env::{self, current_dir};
use std::fs::{DirBuilder, File};
use std::io::prelude::*;
use std::path::{self, PathBuf};

const JL_RUNTESTS_CONTENTS: &str = r###"module Test
using Test

include("../src/Main.jl")


# write tests here...
@testset "Main functions work" begin
    username = "Alice"

    @testset "Main.main(?username) methods work" begin
        @test Main.main() == println("Hello, app user!")
        @test Main.main(username) == println("Hello, Alice !!!")
    end

end

end # module Test
"###;

// const JL_APP_MAIN_MOD_CONTENTS: &str = r###"module Main

// function julia_main()::Cint
//     try
//         main()
//     catch
//         Base.invokelatest(Base.display_error, Base.catch_stack())
//         return 1
//     end
//     return 0
// end

// main()

// end # module Main
// "###;

pub fn new_pkg_ask_name(julia: &mut Julia) {
    let input_pkg_name: String = Input::new()
        .with_prompt("What name would you like to give your package?")
        .interact()
        // .unwrap_or("Main".to_string());
        .unwrap();

    let pkg_name = input_pkg_name;
    new_pkg_w_name(julia, pkg_name.as_str());
}

// TODO! create default files unless ./src/Main.jl & /tests/run_tests.jl files exist
pub fn new_pkg_w_name(julia: &mut Julia, pkg_name: &str) {
    println!("\nActivating environment \"{}\"\n", &pkg_name);

    // Ensure app names are capitalized, as per standard Julia practice
    let pkg_name = pkg_name.to_string().capitalize();
    let activate = julia
        .scope(|mut frame| {
            let jl_module_main = Module::main(&mut frame);

            let pkg_name = JuliaString::new(&mut frame, &pkg_name);

            unsafe {
                jl_module_main
                    // the submodule doesn't have to be rooted because it's never reloaded.
                    .submodule(&mut frame, "Gaston")?
                    .submodule(&mut frame, "New")?
                    // the same holds true for the function: the module is never reloaded so it's globally rooted
                    .function(&mut frame, "make_pkg_in_target_dir")?
                    //
                    // CALLING A FUNCTION
                    //
                    // TODO! Set up PkgTemplates Pkg Generation....!!!!
                    // Call the function with the target Julia frame and ...
                    .call1(&mut frame, pkg_name.as_value())
                    //
                    // If you don't want to use the exception, it can be converted to a `JlrsError`
                    // In this case the error message will contain the message that calling `display` in Julia would show
                    .into_jlrs_result()?
                    .unbox::<String>()
            }
        })
        .expect("Result is an error");

    println!("\n{:?}", activate.unwrap());
}

//     // Get handle to current directory
//     let current_dir = env::current_dir().expect("couldn't get current directory");

//     // Create Tests directory & run_tests.jl file
//     let mut tests_dir_path = path::PathBuf::new();
//     tests_dir_path.push(&current_dir);
//     tests_dir_path.push("./test");

//     DirBuilder::new()
//         .recursive(true)
//         .create(&tests_dir_path)
//         .expect("Could not create `test` directory");

//     let mut tests_file_path = path::PathBuf::new();
//     tests_file_path.push(tests_dir_path);
//     tests_file_path.push("./runtests.jl");
//     let mut jl_runtests_file =
//         File::create(&tests_file_path).expect("could not create runtests.jl file");

//     write!(jl_runtests_file, "{}", JL_RUNTESTS_CONTENTS)
//         .expect("Could not write test file contents");

//     // Next, create the proper structure for creating an app
//     // create julia_main()::CInt fn for PkgCompiler App creation
//     let mut main_file_path = path::PathBuf::new();
//     main_file_path.push(&current_dir);

//     // the generate_docs fn has moved us into the package dir, so just use ./src to get a handle to the main source file
//     main_file_path.push("src");

//     let main_file_name = pkg_name.clone() + ".jl";

//     main_file_path.push(&main_file_name);

//     let mut jl_main_mod_file =
//         File::create(main_file_path).expect("could not create / overwrite main app's module file");

//     // Prepare content to write to the primary (named) module file...
//     let jl_app_main_mod_part1: &str = r#"module "#;

//     let jl_app_main_mod_part2: &str = r###"
// include("Main.jl")

// # Write the necessary functionality for your app using the `Main.jl` file / `main()` function; this file is primarily set up for compiling your app with the PackageCompiler.jl infrastructure.
// # If you need to handle arguments from the command line for your app, a brief demonstration is given below..

// function handle_command_line(ARGS)
//     # do something based on ARGS? ARGS will come from the command line when you run your app..

//     try
//         if isempty(ARGS)
//             # use the Main module, and call the main() function
//             Main.main()
//         else
//             # use the Main module, and call the main() function with 1 arg from cmd line
//             Main.main(ARGS[1])
//         end
//     catch
//         Base.invokelatest(Base.display_error, Base.catch_stack())
//         # return 1 means `return with 1 error' - this is useful for piping the output of one app into another on the command line
//         return 1
//     end
//     # return 0 means 'returned with 0 errors'
//     return 0
// end

// function julia_main()::Cint
//     handle_command_line(ARGS)
// end

// # If this app is run as a script with Julia, handle the execution.
// if abspath(PROGRAM_FILE) == @__FILE__
//     handle_command_line(ARGS)
// end

// end # module "###;

//     write!(
//         jl_main_mod_file,
//         "{}{}{}{}",
//         jl_app_main_mod_part1, &pkg_name, jl_app_main_mod_part2, &pkg_name
//     )
//     .expect("could not write app file");

//     // Create a "Main.jl" file where user writes their code

//     let mut user_main_file_path = path::PathBuf::new();
//     user_main_file_path.push(&current_dir);
//     user_main_file_path.push("src");
//     user_main_file_path.push("Main.jl");

//     println!("{:?}", &user_main_file_path);

//     let mut user_main_file =
//         File::create(user_main_file_path).expect("Could not create user main file for app..");

//     let user_main_file_contents = r###"module Main

// function main()
// 	# write your code here
// 	println("Hello, app user!")
// end

// function main(username::String)
//     println("Hello, $username !!!")
// end

// end # module Main"###;

//     write!(user_main_file, "{}", user_main_file_contents)
//         .expect("could not write user main file contents for app....");
