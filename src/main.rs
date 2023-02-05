#![allow(dead_code)]

use std::ffi::{CString, OsString};
use std::fs;
use std::io;
use std::io::Read;
use std::path::PathBuf;
use std::process;

use clap::{arg, ColorChoice, Command};
use jlrs::prelude::*;
// TODO! The `nix` crate is Unix-only! Find a Windows-compatible way to provide the same functionality!
use nix::unistd::execvp;

use dirs::home_dir;

mod compile;
mod jl_command;
mod julia;
// mod lib;
mod new;
mod pkg;

use jl_command::pluto_nb::check_pluto_nb_is_installed;
use julia::{write_julia_script_to_disk, JULIA_FILE_CONTENTS};
// use lib::run_pluto_nb;
use compile::application::{self, compile_app};
use new::app::{new_app_ask_name, new_app_w_name};
use new::script::{new_script_ask_name, new_script_w_name};
use pkg::add_package::*;
use pkg::remove_package::*;
use pkg::status::*;
use pkg::update::*;

use dialoguer;

fn cli() -> Command {
    Command::new("gsn")
        .about("\nGaston (gt): The Julia project manager")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .color(ColorChoice::Always)
        .visible_alias("gaston")
        .display_name("gaston")
        // TODO! The workflow tip about `infer_subcommands` should be noted in the CLI's help text!
        .infer_subcommands(true)
        .subcommand(
            Command::new("jl")
                .about("start the Julia REPL using the project in the current directory")
                .args_conflicts_with_subcommands(true)
                // TODO! should I implement a `repl` subcommand as well, which does the same thing as the 'run' command? I'm leaning towards yes...
                .subcommand(
                    Command::new("run")
                    .about("runs the Julia REPL in the existing process in the terminal; defaults to the project in the current environment")
                    .visible_alias("repl")
                )
                // TODO! add argument to Pluto to start on a different port, etc....
                .subcommand(
                    Command::new("pluto")
                    .about("starts up the Pluto Notebook environment in the browser")
                    .visible_alias("notebook")
                    .visible_alias("nb")
                )
                // start the default editor / VSCode ?
                .subcommand(
                    Command::new("edit")
                    .visible_alias("code")
                    .about("Both (a) starts VSCode using the current directory, and (b) starts the Julia REPL in the terminal for interactive use / testing")
                ),
        )
        .subcommand(
            Command::new("pkg")
                .about("gets the status of installed packages; sub-commands are available to manage packages in your project")
                .args_conflicts_with_subcommands(true)

                // SUB-COMMANDS

                // Status
            .subcommand(
                Command::new("status")
                    .visible_alias("list")
                    .visible_alias("ls")
                    .about("gets the status of the installed packages; defaults to current directory's environment, if it exists. Otherwise, reports global env package status")

                    // .long_flag("global")
                    // .short_flag('g')
            )

                // Add Package
            .subcommand(
                Command::new("add")
                    .arg(arg!([PACKAGE_NAME]))
                    .visible_alias("install")
                    .about("add a package to the local environment: eg. `gsn pkg add [package_name]` ")
                    // .long_flag("global")
                    // .short_flag('g')
            )

                // Remove Package
            .subcommand(
                Command::new("rm")
                    .arg(arg!([PACKAGE_NAME]))
                    .visible_alias("remove")
                    .visible_alias("delete")
                    .about("remove a package; defaults to local environment")
            )
                //
                //
                // TODO! need to have a flag to differentiate between updating the local environment and the global env
                // Todo! (con'td) Which should be the default? Local env as default, or global env as default?
                //
                // Update Package
                // NB! Because `infer_subcommands` is turned on, above, you can also use `gsn p up` as a short form to activate the `update` command.
            .subcommand(
                Command::new("update")
                    .arg(arg!([PACKAGE_NAME]))
                    .about("updates all packages, or updates a single package if a package_name is supplied; defaults to working on local environment. Eg. `gsn pkg update` or `gsn p up CSV`")
            )
        ) // END PKG Sub-command
        .subcommand(
            Command::new("new")
                .about("creates new environments (scripts), projects (binaries), and packages (libaries)")
                .args_conflicts_with_subcommands(true)
                // .arg_required_else_help(true)
                .visible_alias("generate")
                .subcommand(
                    Command::new("script")
                        .arg(arg!([NAME_FOR_NEW_SCRIPT]))
                        .about("Create a new script, using the name the user provides")

                )
                .subcommand(
                    Command::new("app")
                    .arg(arg!([APP_NAME]))
                    .about("Create a new app using the name the user provides")
                    .visible_alias("application")
                )
        )
        .subcommand(Command::new("compile")
            .visible_alias("create")
            .about("create apps and system images (sysimages)")
                .subcommand(Command::new("app")
                    .about("compiles an app; requires 2 args: a path to the source code, and a path to where the compiled app will be placed")
                    .visible_alias("application")
                    .arg(arg!([JULIA_PROJECT_PATH]))
                    .arg(arg!([COMPILED_APP_PATH_TARGET]))
            )
    )
}

// pub fn run_pluto_nb(julia: &mut Julia) {
//     check_pluto_nb_is_installed(julia);

//     let _output = process::Command::new("julia")
//         .arg("-E")
//         .arg("using Pluto; Pluto.run()")
//         .spawn()
//         .expect("Could not run Julia -> Pluto notebook");
// }

pub fn run_pluto_nb(julia: &mut Julia) {
    check_pluto_nb_is_installed(julia);

    let julia_executable_string = CString::new("julia").expect("CString::new failed...");
    let julia_executable = julia_executable_string.as_c_str();

    let julia_args_julia = CString::new("julia").expect("CString::new failed...");
    let julia_args_project = CString::new("--project=@.").expect("CString::new failed...");
    let julia_args_exec = CString::new("-E").expect("couldn't write -E flag for julia process");

    let julia_args_pluto =
        CString::new("using Pluto; Pluto.run()").expect("couldn't create c-string for using Pluto");

    execvp(
        julia_executable,
        &[
            julia_args_julia,
            julia_args_project,
            julia_args_exec,
            julia_args_pluto,
        ],
    )
    .expect("failed to exec Julia process...");
}

fn main() {
    // If Julia is already installed...
    //
    let mut julia_pending = unsafe { RuntimeBuilder::new().start().expect("Could not init Julia") };

    let mut frame = StackFrame::new();
    let mut julia = julia_pending.instance(&mut frame);

    let home_dir = home_dir().expect("Couldn't find the user's home directory");
    let julia_dir = PathBuf::from(".julia/gaston/Gaston.jl");
    let mut gaston_jl_path = PathBuf::new();
    gaston_jl_path.push(home_dir);
    gaston_jl_path.push(julia_dir);

    // Include some custom code defined in <file>.
    // This is safe because the included code doesn't do any strange things.

    // TODO! update this check to ensure that the contents of the Gaston.jl file matches what's in the julia/mod.rs -> JULIA_FILE_CONTENTS &str...

    if gaston_jl_path.exists() {
        let latest_jl_file_contents = JULIA_FILE_CONTENTS.to_string();

        let gaston_jl_file_contents =
            fs::read_to_string(&gaston_jl_path).expect("Couldn't read Gaston.jl file...");

        let update_file_maybe = latest_jl_file_contents.eq(&gaston_jl_file_contents);

        unsafe {
            if update_file_maybe {
                println!("Gaston path exists @: {:?}", gaston_jl_path);
                julia
                    .include(gaston_jl_path)
                    .expect("Could not include file");
            } else {
                println!("Ensuring you have the latest Gaston.jl file. Writing Gaston.jl file to `$HOME/.julia/gaston/Gaston.jl`", );

                write_julia_script_to_disk()
                    .expect("couldn't write Gaston.jl file to $HOME/.julia/gaston/Gaston.jl");

                julia
                    .include(gaston_jl_path)
                    .expect("Could not include file - please file a bug report!");
            }
        }
    } else {
        println!("Couldn't find Gaston.jl file. Writing Gaston.jl file to `$HOME/.julia/gaston/Gaston.jl`", );

        write_julia_script_to_disk()
            .expect("couldn't write Gaston.jl file to $HOME/.julia/gaston/Gaston.jl");

        unsafe {
            julia
                .include(gaston_jl_path)
                .expect("Could not include file - please file a bug report!");
        }
    }

    // CLI
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("new", sub_matches)) => {
            let new_command = sub_matches
                .subcommand()
                // If an argument isn't supplied to `gsn new <nothing>`, then default to creating a new environment
                .unwrap_or(("script", sub_matches));

            match new_command {
                // if you get an argument, call env with the arg. Otherwise, activate environment in current directory
                ("script", sub_matches) => {
                    if let Some(_) = sub_matches.get_one::<String>("NAME_FOR_NEW_SCRIPT") {
                        let script_name = sub_matches.get_one::<String>("NAME_FOR_NEW_SCRIPT");

                        new_script_w_name(
                            &mut julia,
                            script_name.expect("tried and failed to create a new script..."),
                        );
                    } else {
                        new_script_ask_name(&mut julia);
                    }
                }
                ("app", sub_matches) => {
                    if let Some(_) = sub_matches.get_one::<String>("APP_NAME") {
                        let app_name = sub_matches.get_one::<String>("APP_NAME");

                        new_app_w_name(
                            &mut julia,
                            app_name.expect("tried and failed to create a new app..."),
                        );
                    } else {
                        new_app_ask_name(&mut julia);
                    }
                }

                _ => {
                    unreachable!("Unsupported `new` subcommand",)
                }
            }
        }
        Some(("compile", sub_matches)) => {
            let compile_cmd = sub_matches.subcommand().unwrap();

            match compile_cmd {
                ("app", sub_matches) => {
                    let source_code_path = sub_matches
                        .get_one::<String>("JULIA_PROJECT_PATH")
                        .expect("couldn't parse command line input in compile app command");

                    let target_directory_path = sub_matches
                        .get_one::<String>("COMPILED_APP_PATH_TARGET")
                        .expect("couldn't parse command line input in compile app command");

                    compile_app(&mut julia, source_code_path, target_directory_path);
                }
                _ => {
                    unreachable!("Unsupported compilation subcommand",)
                }
            }
        }
        Some(("jl", sub_matches)) => {
            let jl_command = sub_matches.subcommand().unwrap_or(("run", sub_matches));
            // .expect("messed up gsn jl command");

            match jl_command {
                ("pluto", _sub_matches) => {
                    // TODO! Need to check to make sure that Pluto is installed in the global env. first, or just install it in the environment...
                    //
                    // Run Pluto via `julia -E 'using Pluto; Pluto.run()'` command
                    //
                    //

                    run_pluto_nb(&mut julia);

                    // check_pluto_nb_is_installed(&mut julia);

                    // let _output = process::Command::new("julia")
                    //     .arg("-E")
                    //     .arg("using Pluto; Pluto.run()")
                    //     .spawn()
                    //     .expect("Could not run Julia -> Pluto notebook");
                }

                // };

                // if run with `gsn jl run`, then start the julia process using the current directory as the active environment
                ("run", _sub_matches) => {
                    let julia_executable_string =
                        CString::new("julia").expect("CString::new failed...");
                    let julia_executable = julia_executable_string.as_c_str();

                    let julia_args_julia = CString::new("julia").expect("CString::new failed...");
                    let julia_args_project =
                        CString::new("--project=@.").expect("CString::new failed...");

                    execvp(julia_executable, &[julia_args_julia, julia_args_project])
                        .expect("failed to exec Julia process...");
                }
                ("edit", _sub_matches) => {
                    // TODO! Introduce checks to ensure that VSCode is installed; if not, ask the user if they want to install it...
                    // start VSCode...
                    let _launch_vscode = if cfg!(target_os = "windows") {
                        // TODO! Test this on Windows!
                        process::Command::new("cmd")
                            .args(["/C", "code", "./"])
                            // .output()
                            .spawn()
                            .expect("failed to launch VSCode");
                    } else {
                        // Launch VSCode in $PWD
                        process::Command::new("sh")
                            .current_dir("./")
                            .arg("-c")
                            .arg("code .")
                            // .output()
                            .spawn()
                            .expect("Failed to execute VSCode...");
                    };

                    // start Julia repl in the background as well...
                    let julia_executable_string =
                        CString::new("julia").expect("CString::new failed...");
                    let julia_executable = julia_executable_string.as_c_str();

                    let julia_args_julia = CString::new("julia").expect("CString::new failed...");
                    let julia_args_project =
                        CString::new("--project=@.").expect("CString::new failed...");

                    // process::Command::new("code")
                    execvp(julia_executable, &[julia_args_julia, julia_args_project])
                        .expect("failed to exec Julia process...");
                }
                // if no sub-command, then start the julia process...
                _ => {
                    let julia_executable_string =
                        CString::new("julia").expect("CString::new failed...");
                    let julia_executable = julia_executable_string.as_c_str();

                    let julia_args_julia = CString::new("julia").expect("CString::new failed...");
                    let julia_args_project =
                        CString::new("--project=@.").expect("CString::new failed...");

                    execvp(julia_executable, &[julia_args_julia, julia_args_project])
                        .expect("failed to exec Julia process...");
                }
            }

            // let julia_executable_string = CString::new("julia").expect("CString::new failed...");
            // let julia_executable = julia_executable_string.as_c_str();

            // let julia_args_julia = CString::new("julia").expect("CString::new failed...");
            // let julia_args_project = CString::new("--project=@.").expect("CString::new failed...");

            // execvp(julia_executable, &[julia_args_julia, julia_args_project])
            //     .expect("failed to exec Julia process...");
        }
        Some(("pkg", sub_matches)) => {
            let pkg_command = sub_matches.subcommand().unwrap_or(("status", sub_matches));

            match pkg_command {
                ("status", _sub_matches) => {
                    status(&mut julia);
                    // println!("sub_matches: {:?}", sub_matches);
                }
                ("add", sub_matches) => {
                    if let Some(add_one_pkg) = sub_matches.get_one::<String>("PACKAGE_NAME") {
                        add_one_package(&mut julia, add_one_pkg);
                    } else {
                        let package_name = dialoguer::Input::<String>::new()
                            .with_prompt("Which package would you like to add?")
                            .interact()
                            .expect("Must add a package name to add a package!");

                        add_one_package(&mut julia, &package_name);
                    }
                }
                // SHORT FORM of remove (one) package
                ("rm", sub_matches) => {
                    if let Some(remove_one_pkg) = sub_matches.get_one::<String>("PACKAGE_NAME") {
                        remove_one_package(&mut julia, remove_one_pkg);
                    } else {
                        let package_name = dialoguer::Input::<String>::new()
                            .with_prompt("Which package would you like to remove?")
                            .interact()
                            .expect("Must provide a package name to remove a package!");

                        remove_one_package(&mut julia, &package_name);
                    }
                }
                // LONG FORM of remove (one) package
                ("remove", sub_matches) => {
                    let remove_one_pkg = sub_matches.get_one::<String>("PACKAGE_NAME");

                    remove_one_package(
                        &mut julia,
                        remove_one_pkg.expect("must provide a package name to remove the package!"),
                    );
                }
                ("update", sub_matches) => {
                    // if you get an argument, call update package, otherwise call update_all_packages

                    if let Some(_) = sub_matches.get_one::<String>("PACKAGE_NAME") {
                        let update_one_pkg = sub_matches.get_one::<String>("PACKAGE_NAME");

                        update_one_package(
                            &mut julia,
                            update_one_pkg
                                .expect("Must provide a package name to update that package!"),
                        );
                    } else {
                        update_all_packages(&mut julia);
                    };
                }
                (name, _) => {
                    unreachable!("Unsupported subcommand `{}`", name)
                }
            }
        }

        Some((ext, sub_matches)) => {
            let args = sub_matches
                .get_many::<OsString>("id")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            println!("Calling out to {:?} with {:?}", ext, args);
        }
        // If all subcommands are defined above, anything else is unreachable!
        _ => unreachable!(),
    }
}

//---------------------------------------------------------------------------------
//
//
// Example Clap sub-commands
// for reference
//
//
//
//
// .subcommand(
//     Command::new("clone")
//         .about("clones repos")
//         .arg(arg!(<REMOTE> "The remote to clone"))
//         .arg_required_else_help(true),
// )
// .subcommand(
//     Command::new("diff")
//         .about("Compare two commits")
//         .arg(arg!(base: [COMMIT]))
//         .arg(arg!(head: [COMMIT]))
//         .arg(arg!(path: [PATH]).last(true))
//         .arg(
//             arg!(--color <WHEN>)
//                 .value_parser(["always", "auto", "never"])
//                 .num_args(0..=1)
//                 .require_equals(true)
//                 .default_value("auto")
//                 .default_missing_value("always"),
//         ),
// )
// .subcommand(
//     Command::new("push")
//         .about("pushes repo to a remote")
//         .arg(arg!(<REMOTE> "The remote to target"))
//         .arg_required_else_help(true),
// )
// .subcommand(
//     Command::new("add")
//         .about("add things to the repo")
//         .arg_required_else_help(true)
//         .arg(
//             arg!(<PATH> ... "Files and folders to add")
//                 .value_parser(clap::value_parser!(PathBuf)),
//         ),
// )
// .subcommand(
//     Command::new("stash")
//         .about("stashes changes for later")
//         .args_conflicts_with_subcommands(true)
//         .args(push_args())
//         .subcommand(Command::new("push").args(push_args()))
//         .subcommand(Command::new("pop").arg(arg!([STASH])))
//         .subcommand(Command::new("apply").arg(arg!([STASH]))),
// )
//
//
// fn push_args() -> Vec<clap::Arg> {
//     vec![arg!(-m --message <MESSAGE>)]
// }
//
//
// -----------------------------------------
//
//
//
// Example Sub-command matches
//
//
// match matches.subcommand() {
// Some(("clone", sub_matches)) => {
//     println!(
//         "Cloning {}",
//         sub_matches.get_one::<String>("REMOTE").expect("required")
//     );
// }
// Some(("diff", sub_matches)) => {
//     let color = sub_matches
//         .get_one::<String>("color")
//         .map(|s| s.as_str())
//         .expect("defaulted in Clap");

//     let mut base = sub_matches.get_one::<String>("base").map(|s| s.as_str());
//     let mut head = sub_matches.get_one::<String>("head").map(|s| s.as_str());
//     let mut path = sub_matches.get_one::<String>("path").map(|s| s.as_str());

//     if path.is_none() {
//         path = head;
//         head = None;
//         if path.is_none() {
//             path = base;
//             base = None;
//         }
//     }
//     let base = base.unwrap_or("stage");
//     let head = head.unwrap_or("worktree");
//     let path = path.unwrap_or("");
//     println!("Diffing {} ... {} {} (color={})", base, head, path, color);
// }
// Some(("push", sub_matches)) => {
//     println!(
//         "Pushing to {}",
//         sub_matches.get_one::<String>("REMOTE").expect("required")
//     );
// }
// Some(("add", sub_matches)) => {
//     let paths = sub_matches
//         .get_many::<PathBuf>("PATH")
//         .into_iter()
//         .flatten()
//         .collect::<Vec<_>>();
//     println!("Adding {:?}", paths);
// }
// Some(("stash", sub_matches)) => {
//     let stash_command = sub_matches.subcommand().unwrap_or(("push", sub_matches));
//     match stash_command {
//         ("apply", sub_matches) => {
//             let stash = sub_matches.get_one::<String>("STASH");
//             println!("Applying {:?}", stash);
//         }
//         ("pop", sub_matches) => {
//             let stash = sub_matches.get_one::<String>("STASH");
//             println!("Popping {:?}", stash);
//         }
//         ("push", sub_matches) => {
//             let message = sub_matches.get_one::<String>("message");
//             println!("Pushing {:?}", message);
//         }
//         (name, _) => {
//             unreachable!("Unsupported subcommand `{}`", name)
//         }
//     }
// }
