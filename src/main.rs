use std::ffi::OsString;
use std::path::PathBuf;

use clap::{arg, Command};
use jlrs::prelude::*;

mod add_package;
mod julia;
mod status;
use crate::add_package::add_one_pkg;
use crate::status::status;

fn cli() -> Command {
    Command::new("gs")
        .about("Julia installer and project manager")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("clone")
                .about("clones repos")
                .arg(arg!(<REMOTE> "The remote to clone"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("diff")
                .about("Compare two commits")
                .arg(arg!(base: [COMMIT]))
                .arg(arg!(head: [COMMIT]))
                .arg(arg!(path: [PATH]).last(true))
                .arg(
                    arg!(--color <WHEN>)
                        .value_parser(["always", "auto", "never"])
                        .num_args(0..=1)
                        .require_equals(true)
                        .default_value("auto")
                        .default_missing_value("always"),
                ),
        )
        .subcommand(
            Command::new("push")
                .about("pushes repo to a remote")
                .arg(arg!(<REMOTE> "The remote to target"))
                .arg_required_else_help(true),
        )
        .subcommand(
            Command::new("add")
                .about("add things to the repo")
                .arg_required_else_help(true)
                .arg(
                    arg!(<PATH> ... "Files and folders to add")
                        .value_parser(clap::value_parser!(PathBuf)),
                ),
        )
        .subcommand(
            Command::new("stash")
                .about("stashes changes for later")
                .args_conflicts_with_subcommands(true)
                .args(push_args())
                .subcommand(Command::new("push").args(push_args()))
                .subcommand(Command::new("pop").arg(arg!([STASH])))
                .subcommand(Command::new("apply").arg(arg!([STASH]))),
        )
        .subcommand(
            Command::new("pkg")
                .about("uses the Julia Pkg manager API")
                .args_conflicts_with_subcommands(true)
                .subcommand(Command::new("status"))
                .subcommand(Command::new("add").arg(arg!([PACKAGE_NAME]))),
        )
}

fn push_args() -> Vec<clap::Arg> {
    vec![arg!(-m --message <MESSAGE>)]
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
        let path = PathBuf::from("PkgAPI.jl");
        if path.exists() {
            julia.include(path).expect("Could not include file");
        } else {
            julia
                .include("src/julia/PkgAPI.jl")
                .expect("Could not include file");
        }
    }

    // CLI
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("clone", sub_matches)) => {
            println!(
                "Cloning {}",
                sub_matches.get_one::<String>("REMOTE").expect("required")
            );
        }
        Some(("diff", sub_matches)) => {
            let color = sub_matches
                .get_one::<String>("color")
                .map(|s| s.as_str())
                .expect("defaulted in Clap");

            let mut base = sub_matches.get_one::<String>("base").map(|s| s.as_str());
            let mut head = sub_matches.get_one::<String>("head").map(|s| s.as_str());
            let mut path = sub_matches.get_one::<String>("path").map(|s| s.as_str());

            if path.is_none() {
                path = head;
                head = None;
                if path.is_none() {
                    path = base;
                    base = None;
                }
            }
            let base = base.unwrap_or("stage");
            let head = head.unwrap_or("worktree");
            let path = path.unwrap_or("");
            println!("Diffing {} ... {} {} (color={})", base, head, path, color);
        }
        Some(("push", sub_matches)) => {
            println!(
                "Pushing to {}",
                sub_matches.get_one::<String>("REMOTE").expect("required")
            );
        }
        Some(("add", sub_matches)) => {
            let paths = sub_matches
                .get_many::<PathBuf>("PATH")
                .into_iter()
                .flatten()
                .collect::<Vec<_>>();
            println!("Adding {:?}", paths);
        }
        Some(("stash", sub_matches)) => {
            let stash_command = sub_matches.subcommand().unwrap_or(("push", sub_matches));
            match stash_command {
                ("apply", sub_matches) => {
                    let stash = sub_matches.get_one::<String>("STASH");
                    println!("Applying {:?}", stash);
                }
                ("pop", sub_matches) => {
                    let stash = sub_matches.get_one::<String>("STASH");
                    println!("Popping {:?}", stash);
                }
                ("push", sub_matches) => {
                    let message = sub_matches.get_one::<String>("message");
                    println!("Pushing {:?}", message);
                }
                (name, _) => {
                    unreachable!("Unsupported subcommand `{}`", name)
                }
            }
        }
        Some(("pkg", sub_matches)) => {
            let pkg_command = sub_matches.subcommand().unwrap_or(("status", sub_matches));
            match pkg_command {
                ("status", sub_matches) => {
                    // TODO!!!
                    status(&mut julia);
                }
                ("add", sub_matches) => {
                    // TODO!!!
                    let add_one_package = sub_matches.get_one::<String>("PACKAGE_NAME");

                    add_one_pkg(
                        &mut julia,
                        add_one_package.expect("Must provide a package name to add a package!"),
                    );
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
