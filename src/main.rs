#![allow(dead_code)]

use std::ffi::{CString, OsString};
use std::path::PathBuf;
use std::process;

use clap::{arg, ColorChoice, Command};
use jlrs::prelude::*;
// TODO! The `nix` crate is Unix-only! Find a Windows-compatible way to provide the same functionality!
use nix::unistd::execvp;

use dirs::home_dir;

mod julia;
mod new;
mod pkg;

use julia::write_julia_script_to_disk;
use pkg::activate::{activate_env_in_current_dir, activate_env_w_name};
use pkg::add_package::*;
use pkg::remove_package::*;
use pkg::status::*;
use pkg::update::*;

fn cli() -> Command {
    Command::new("gsn")
        .about("\nGaston (gsn): The Julia project manager")
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
                )
                // TODO! add argument to Pluto to start on a different port, etc....
                .subcommand(
                    Command::new("pluto")
                    .about("starts up the Pluto Notebook environment in the browser")
                )
                // start the default editor / VSCode ?
                .subcommand(
                    Command::new("edit")
                    .about("Both (a) starts VSCode using the current directory, and (b) starts the Julia REPL in the terminal for interactive use / testing")
                ),
        )
        .subcommand(
            Command::new("pkg")
                .about("gets the status of installed packages; also has multiple sub-commands to manage packages in your project")
                .args_conflicts_with_subcommands(true)
                // use the `global` flags to operate on the global environment...
                // .long_flag("global")
                // .short_flag('g')
        // )


                // SUB-COMMANDS

                // Status
            .subcommand(
                Command::new("status")
                    .about("gets the status of the installed packages")
                    // .long_flag("global")
                    // .short_flag('g')
            )

                // Add Package
            .subcommand(
                Command::new("add")
                    .arg(arg!([PACKAGE_NAME]))
                    .about("add a package to the current environment: `gsn pkg add [package_name]")
                    // .long_flag("global")
                    // .short_flag('g')
            )

                // Remove Package
            .subcommand(
                Command::new("rm")
                    .arg(arg!([PACKAGE_NAME]))
                    .visible_alias("remove")
                    .about("remove a package")
            )

            // .subcommand(
            //     Command::new("remove")
            //         .arg(arg!([PACKAGE_NAME]))
            // )
                //
                //
                // NB! Because `infer_subcommands` is turned on, above, you can also use "up" as a short form to activate the `update` command.
                // TODO! This workflow tip about `infer_subcommands` should be noted as an example in the help docs printed on the cmd line.
                // TODO! (cont'd) Eg. use `gsn p up` to update packages.
                //
                //
                // TODO! need to have a flag to differentiate between updating the local environment and the global env
                // Todo! (con'td) Which should be the default? Local env as default, or global env as default?
                //
                // Update Package
            .subcommand(
                Command::new("update")
                    .arg(arg!([PACKAGE_NAME]))
            )
        ) // END PKG Sub-command
        .subcommand(
            Command::new("new")
                .about("creates new environments (scripts), projects (binaries), and packages (libaries)")
                .args_conflicts_with_subcommands(true)
                .subcommand(Command::new("env").arg(arg!([ENVIRONMENT_NAME]))),
        )
}

fn main() {
    // If Julia is already installed...
    //
    let mut julia_pending = unsafe { RuntimeBuilder::new().start().expect("Could not init Julia") };

    let mut frame = StackFrame::new();
    let mut julia = julia_pending.instance(&mut frame);

    let home_dir = home_dir().expect("Couldn't find the user's home directory");
    let julia_dir = PathBuf::from(".julia/gaston/Gaston.jl");
    let mut gaston_path = PathBuf::new();
    gaston_path.push(home_dir);
    gaston_path.push(julia_dir);
    let gaston_jl_path = &gaston_path;

    // Include some custom code defined in <file>.
    // This is safe because the included code doesn't do any strange things.
    unsafe {
        if gaston_jl_path.exists() {
            // println!("Gaston path exists @: {:?}", gaston_jl_path);
            julia
                .include(gaston_jl_path)
                .expect("Could not include file");
        } else {
            println!("Gaston.jl file not found. Writing Gaston.jl file to `$HOME/.julia/gaston/Gaston.jl`", );

            write_julia_script_to_disk()
                .expect("couldn't write Gaston.jl file to $HOME/.julia/gaston/Gaston.jl");

            julia
                .include(gaston_jl_path)
                .expect("Could not include file");
        }
    }

    // CLI
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("new", sub_matches)) => {
            let new_command = sub_matches
                .subcommand()
                // If an argument isn't supplied to `gsn new <nothing>`, then default to creating a new environment
                .unwrap_or(("env", sub_matches));

            match new_command {
                // if you get an argument, call env with the arg. Otherwise, activate environment in current directory
                ("env", sub_matches) => {
                    if let Some(_) = sub_matches.get_one::<String>("ENVIRONMENT_NAME") {
                        let activate_env = sub_matches.get_one::<String>("ENVIRONMENT_NAME");

                        activate_env_w_name(
                            &mut julia,
                            activate_env.expect("tried and failed to activate environment..."),
                        );
                    } else {
                        activate_env_in_current_dir(&mut julia);
                    }
                }
                _ => {
                    unreachable!("Unsupported subcommand",)
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
                    let julia_executable_string =
                        CString::new("julia").expect("CString::new failed...");
                    let julia_executable = julia_executable_string.as_c_str();

                    let julia_args_julia = CString::new("julia").expect("CString::new failed...");

                    let julia_args_execute = CString::new("-E").expect("CString::new failed...");
                    let julia_args_pluto =
                        CString::new("using Pluto; Pluto.run()").expect("CString::new failed...");

                    execvp(
                        julia_executable,
                        &[julia_args_julia, julia_args_execute, julia_args_pluto],
                    )
                    .expect("failed to exec Julia process...");
                }
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
                ("status", sub_matches) => {
                    status(&mut julia);
                    println!("sub_matches: {:?}", sub_matches);
                }
                ("add", sub_matches) => {
                    let add_one_pkg = sub_matches.get_one::<String>("PACKAGE_NAME");

                    add_one_package(
                        &mut julia,
                        add_one_pkg.expect("Must provide a package name to add a package!"),
                    );
                }
                // SHORT FORM of remove (one) package
                ("rm", sub_matches) => {
                    let remove_one_pkg = sub_matches.get_one::<String>("PACKAGE_NAME");

                    remove_one_package(
                        &mut julia,
                        remove_one_pkg.expect("must provide a package name to remove the package!"),
                    );
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
