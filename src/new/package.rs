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
use anyhow::{anyhow, Context, Result};
use capitalize::Capitalize;
use dialoguer::{console::Term, theme::ColorfulTheme, Input};
use jlrs::prelude::*;
use nix::unistd::execvp;
use std::env::{self, current_dir};
use std::ffi::{CString, OsString};
use std::fs::{DirBuilder, File};
use std::io::prelude::*;
use std::path::{self, PathBuf};

// const JL_RUNTESTS_CONTENTS: &str = r###"module Test
// using Test

// include("../src/Main.jl")

// # write tests here...
// @testset "Main functions work" begin
//     username = "Alice"

//     @testset "Main.main(?username) methods work" begin
//         @test Main.main() == println("Hello, app user!")
//         @test Main.main(username) == println("Hello, Alice !!!")
//     end

// end

// end # module Test
// "###;

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

pub fn new_package_ask_name(julia: &mut Julia) -> Result<()> {
    let input_pkg_name: String = Input::new()
        .with_prompt("What name would you like to give your package?")
        .interact()
        // .unwrap_or("Main".to_string());
        .unwrap();

    let pkg_name = input_pkg_name;
    new_package_w_name(julia, pkg_name)?;
    Ok(())
}
// pub fn new_package_w_name(julia: &mut Julia, sub_matches: &ArgMatches) -> Result<()> {
pub fn new_package_w_name(julia: &mut Julia, package_name: String) -> Result<()> {
    // let package_name = sub_matches.get_one::<String>("PACKAGE_NAME");

    // TODO: Check to make sure that PkgTemplates is installed....
    // check_pluto_nb_is_installed(julia);

    let julia_executable_string = CString::new("julia").expect("CString::new failed...");
    let julia_executable = julia_executable_string.as_c_str();

    let julia_args_julia = CString::new("julia").expect("CString::new failed...");
    let julia_args_project = CString::new("--project=@.").expect("CString::new failed...");
    let julia_args_exec = CString::new("-E").expect("couldn't write -E flag for julia process");

    // Option 1: julia_args_pkgtemplates
    // let template_cmd = format!(
    //     "using PkgTemplates; t = Template(; user=\"diversable\"); t({:?})",
    //     package_name.expect("couldn't get package name from string")
    // );
    // let julia_args_pkgtemplates = CString::new(template_cmd)?;

    // Option 2: julia_args_pkgtemplates

    let arg = format!("make_package({:?})", package_name);
    let julia_args_pkgtemplates = CString::new(arg)?;

    execvp(
        julia_executable,
        &[
            julia_args_julia,
            julia_args_project,
            julia_args_exec,
            julia_args_pkgtemplates,
        ],
    )
    .expect("failed to exec Julia process...");

    Ok(())
}

// // TODO! create default files unless ./src/Main.jl & /tests/run_tests.jl files exist
// pub fn new_pkg_w_name(julia: &mut Julia, pkg_name: &str) {
//     println!("\nActivating environment \"{}\"\n", &pkg_name);

//     // Ensure app names are capitalized, as per standard Julia practice
//     let pkg_name = pkg_name.to_string().capitalize();
//     let activate = julia
//         .scope(|mut frame| {
//             let jl_module_main = Module::main(&mut frame);

//             let pkg_name = JuliaString::new(&mut frame, &pkg_name);

//             unsafe {
//                 jl_module_main
//                     // the submodule doesn't have to be rooted because it's never reloaded.
//                     .submodule(&mut frame, "Gaston")?
//                     .submodule(&mut frame, "New")?
//                     // the same holds true for the function: the module is never reloaded so it's globally rooted
//                     .function(&mut frame, "make_pkg_in_target_dir")?
//                     //
//                     // CALLING A FUNCTION
//                     //
//                     // TODO! Set up PkgTemplates Pkg Generation....!!!!
//                     // Call the function with the target Julia frame and ...
//                     .call1(&mut frame, pkg_name.as_value())
//                     //
//                     // If you don't want to use the exception, it can be converted to a `JlrsError`
//                     // In this case the error message will contain the message that calling `display` in Julia would show
//                     .into_jlrs_result()?
//                     .unbox::<String>()
//             }
//         })
//         .expect("Result is an error");

//     println!("\n{:?}", activate.unwrap());
// }
