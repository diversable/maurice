#![allow(dead_code)]

use nix::unistd::execvp;
use std::ffi::{CString, OsString};

use std::path::PathBuf;

use clap::{arg, ColorChoice, Command};
use jlrs::prelude::*;

mod julia;

mod pkg;

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
        .infer_subcommands(true)
        .subcommand(
            Command::new("jl")
                .about("start the Julia REPL using the project in the current directory"),
        )
        .subcommand(
            Command::new("pkg")
                .about("uses the Julia Pkg manager API")
                .args_conflicts_with_subcommands(true)
                .subcommand(Command::new("status"))
                .subcommand(Command::new("add").arg(arg!([PACKAGE_NAME])))
                .subcommand(Command::new("rm").arg(arg!([PACKAGE_NAME])))
                .subcommand(Command::new("remove").arg(arg!([PACKAGE_NAME])))
                .subcommand(Command::new("update").arg(arg!([PACKAGE_NAME]))),
        )
        .subcommand(
            Command::new("new")
                .about("creates new projects, packages, and environments")
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

    // Include some custom code defined in <file>.
    // This is safe because the included code doesn't do any strange things.
    unsafe {
        let path = PathBuf::from("./julia/Gaston.jl");
        if path.exists() {
            julia.include(path).expect("Could not include file");
        } else {
            julia
                .include("src/julia/Gaston.jl")
                .expect("Else path: Could not include file");
        }
    }

    // CLI
    let matches = cli().get_matches();

    match matches.subcommand() {
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
        Some(("jl", _sub_matches)) => {
            let julia_executable_string = CString::new("julia").expect("CString::new failed...");
            let julia_executable = julia_executable_string.as_c_str();

            let julia_args = CString::new("--project='@.'").expect("CString::new failed...");

            execvp(julia_executable, &[julia_args]).expect("failed to exec Julia process...");
        }
        Some(("pkg", sub_matches)) => {
            let pkg_command = sub_matches.subcommand().unwrap_or(("status", sub_matches));

            match pkg_command {
                ("status", _sub_matches) => {
                    // TODO!!!
                    status(&mut julia);
                }
                ("add", sub_matches) => {
                    // TODO!!!
                    let add_one_pkg = sub_matches.get_one::<String>("PACKAGE_NAME");

                    add_one_package(
                        &mut julia,
                        add_one_pkg.expect("Must provide a package name to add a package!"),
                    );
                }
                // SHORT FORM of remove (one) package
                ("rm", sub_matches) => {
                    // TODO!!!
                    let remove_one_pkg = sub_matches.get_one::<String>("PACKAGE_NAME");

                    remove_one_package(
                        &mut julia,
                        remove_one_pkg.expect("must provide a package name to remove the package!"),
                    );
                }
                // LONG FORM of remove (one) package
                ("remove", sub_matches) => {
                    // TODO!!!
                    let remove_one_pkg = sub_matches.get_one::<String>("PACKAGE_NAME");

                    remove_one_package(
                        &mut julia,
                        remove_one_pkg.expect("must provide a package name to remove the package!"),
                    );
                }
                ("update", sub_matches) => {
                    // TODO!!!

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
        Some(("new", sub_matches)) => {
            let new_command = sub_matches
                .subcommand()
                // If an argument isn't supplied to `gs new `, then default to creating a new environment
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
