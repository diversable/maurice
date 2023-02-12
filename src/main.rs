#![allow(dead_code)]

use std::ffi::{CString, OsString};
use std::fs::{self, DirBuilder};
use std::io::prelude::*;
use std::path::PathBuf;
use std::process;

use clap::{arg, ArgMatches, ColorChoice, Command};
use jlrs::prelude::*;
// TODO! The `nix` crate is Unix-only! Find a Windows-compatible way to provide the same functionality!
use nix::unistd::execvp;

use anyhow::{anyhow, Context, Result};

use dialoguer;
use dialoguer::theme::ColorfulTheme;
use dialoguer::{console::Term, Select};
use duct::cmd;

use dirs::home_dir;

mod compile;
mod jl_command;
mod julia;
mod new;
mod pkg;
mod test_command;

use compile::application::{compile_app, get_app_compile_target_path, get_app_source_path};
use jl_command::pluto_nb::check_pluto_nb_is_installed;
use julia::{write_julia_script_to_disk, JULIA_FILE_CONTENTS};
use new::app::{new_app_ask_name, new_app_w_name};
use new::package::{new_package_ask_name, new_package_w_name};
use new::script::{new_script_ask_name, new_script_w_name};
use pkg::add_package::*;
use pkg::remove_package::*;
use pkg::status::*;
use pkg::update::*;
use test_command::run_tests::run_tests;

fn cli() -> Command {
    Command::new("mce")
        .about("\nMaurice (mce): The Julia project manager")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .color(ColorChoice::Always)
        .visible_alias("maurice")
        .display_name("maurice")
        // TODO! The workflow tip about `infer_subcommands` should be noted in the CLI's help text!
        .infer_subcommands(true)
        .subcommand(Command::new("jl")
                .about("start the Julia REPL using the project in the current directory; sub-commands start Pluto notebooks and VSCode")
                .args_conflicts_with_subcommands(true)
                .subcommand(
                    Command::new("repl")
                    .about("runs the Julia REPL in the existing process in the terminal; defaults to the project in the current environment")
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
        .subcommand(Command::new("pkg")
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
                    .about("add a package to the local environment: eg. `mce pkg add [package_name]` ")
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
                // NB! Because `infer_subcommands` is turned on, above, you can also use `gt p up` as a short form to activate the `update` command.
            .subcommand(
                Command::new("update")
                    .arg(arg!([PACKAGE_NAME]))
                    .about("updates all packages, or updates a single package if a package_name is supplied; defaults to working on local environment. Eg. `mce pkg update` or `mce p up CSV`")
            )
        ) // END PKG Sub-command
        .subcommand(Command::new("new")
                .about("creates new scripts, apps, and packages* (*feature in progress)")
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
                    .visible_alias("application")
                    .arg(arg!([APP_NAME]))
                    .about("Create a new app using the name the user provides")
                )
                .subcommand(
                    Command::new("package")
                        .visible_alias("pkg")
                        .about("Create a new package using the name and configuration the user provides")
                        .arg(arg!([PACKAGE_NAME]))
                )
        )
        .subcommand(Command::new("compile")
            .visible_alias("create")
            .about("create/compile apps and system images* (sysimages) (*feature in-progress)")
                .subcommand(Command::new("app")
                    .about("compiles an app; requires 2 args: a path to the source code, and a path to where the compiled app will be placed")
                    .visible_alias("application")
                    .arg(arg!([JULIA_PROJECT_PATH]))
                    .arg(arg!([COMPILED_APP_PATH_TARGET]))
            )
        )
        .subcommand(Command::new("run")
            .visible_alias("exec")
            // .visible_alias("command")
            // .visible_alias("cmd")
            .about("execute julia code from the command line...")
            .arg(arg!([JL_CODE]))
        )
        .subcommand(Command::new("test")
            .about("test your script, app, or package")
            .subcommand(Command::new("run")
                .about("run tests defined in `test/runtests.jl` file")
            )
            // .subcommand("doctest")
            // .subcommand(Command::new("add")
            //     .about("add a package to the `test` dir's Project.toml")
            // )
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
        CString::new("using Pluto; Pluto.run()").expect("couldn't create C-string for using Pluto");

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
fn handle_run_script(jl_code: &str) -> Result<()> {
    // if the name passed in contains ".jl", then run it as a script, otherwise, execute the input string (`jl_code`) as raw Julia code..
    let julia_executable_string = CString::new("julia").expect("CString::new failed...");
    let julia_executable = julia_executable_string.as_c_str();

    let julia_args_julia = CString::new("julia").expect("CString::new failed...");

    let julia_args_project = CString::new("--project=@.").expect("CString::new failed...");

    let julia_args_to_script =
        CString::new(jl_code).expect("couldn't create C-string for running command");

    execvp(
        julia_executable,
        &[julia_args_julia, julia_args_project, julia_args_to_script],
    )
    .expect("failed to exec Julia process...");
    Ok(())
}

fn handle_raw_jl_string(jl_code: &str) -> Result<()> {
    // execute code as raw julia code just like `julia -e "..."`
    let julia_executable_string = CString::new("julia").expect("CString::new failed...");
    let julia_executable = julia_executable_string.as_c_str();

    let julia_args_julia = CString::new("julia").expect("CString::new failed...");
    let julia_args_project = CString::new("--project=@.").expect("CString::new failed...");
    let julia_args_exec = CString::new("-E").expect("couldn't write -E flag for julia process");

    let julia_args_to_run =
        CString::new(jl_code).expect("couldn't create C-string for running command");

    execvp(
        julia_executable,
        &[
            julia_args_julia,
            julia_args_project,
            julia_args_exec,
            julia_args_to_run,
        ],
    )
    .expect("failed to exec Julia process...");
    Ok(())
}

fn handle_cli(mut julia: Julia, matches: ArgMatches) {
    match matches.subcommand() {
        Some(("test", sub_matches)) => {
            let test_cmd = sub_matches.subcommand().unwrap_or(("run", sub_matches));

            match test_cmd {
                ("run", _sub_matches) => run_tests(&mut julia),
                _ => {
                    unreachable!("Unsupported `new` subcommand",)
                }
            }
        }
        Some(("run", sub_matches)) => {
            let empty = "ask_for_input".to_string();
            let jl_code = sub_matches
                .get_one::<String>("JL_CODE")
                // .expect("couldn't extract run code argument");
                .unwrap_or(&empty);

            let jl_code = jl_code.as_str();

            if jl_code.contains("ask_for_input") {
                let run_input = dialoguer::Input::<String>::new().with_prompt("\nWhat would you like to run?\n Please enter the path to a file/script, or enter a command in quotes such as \"using Pluto; Pluto.run()\"\n")
                    .interact().expect("couldn't understand your `mce run <input>` input..");

                if run_input.is_empty() {
                    process::exit(0);
                } else if run_input.contains(".jl") {
                    handle_run_script(&run_input).expect("Could not run script for you :(");
                } else {
                    println!("run_input: {}", &run_input);
                    handle_raw_jl_string(&run_input)
                        .expect("Couldn't execute that Julia string for you... :(")
                }
            } else if jl_code.contains(".jl") {
                handle_run_script(jl_code).expect("Could not run script for you :(");
            } else {
                handle_raw_jl_string(jl_code)
                    .expect("Could not execute that Julia code for you... :(");
            }
        }
        Some(("new", sub_matches)) => {
            let new_command = sub_matches
                .subcommand()
                // If an argument isn't supplied to `mce new <nothing>`, then default to creating a new environment
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
                ("package", sub_matches) => {
                    if let Some(_) = sub_matches.get_one::<String>("PACKAGE_NAME") {
                        // for both Option 1 & 2
                        let package_name = sub_matches.get_one::<String>("PACKAGE_NAME");

                        if let Some(package_name) = package_name {
                            // new_package_w_name(&mut julia, sub_matches)
                            new_package_w_name(&mut julia, package_name.to_owned())
                                .expect("couldn't create new package...");
                        }
                    } else {
                        new_package_ask_name(&mut julia).expect("couldn't create a package, even though I asked for the package name...");
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
                    // Check for positional arguments (first, app source path), if the user provided it; if not, ask for it with the get_... functions...
                    if let Some(_) = sub_matches.get_one::<String>("JULIA_PROJECT_PATH") {
                        let source_code_path = sub_matches
                            .get_one::<String>("JULIA_PROJECT_PATH")
                            .expect("couldn't parse command line input in compile app command");

                        // check for second positional argment, if the user provided it...
                        if let Some(_) = sub_matches.get_one::<String>("COMPILED_APP_PATH_TARGET") {
                            let target_directory_path = sub_matches
                                .get_one::<String>("COMPILED_APP_PATH_TARGET")
                                .expect("couldn't parse command line input in compile app command");

                            compile_app(&mut julia, source_code_path, target_directory_path);
                        } else {
                            let source_code_path = sub_matches
                                .get_one::<String>("JULIA_PROJECT_PATH")
                                .expect("couldn't parse command line input in compile app command");
                            let target_directory_path = get_app_compile_target_path();

                            compile_app(
                                &mut julia,
                                source_code_path,
                                target_directory_path.as_str(),
                            );
                        }
                    } else {
                        // if neither source nor target path is provided, get it from user input

                        let source_path = get_app_source_path();
                        let target_path = get_app_compile_target_path();
                        compile_app(&mut julia, source_path.as_str(), target_path.as_str())
                    }
                }
                _ => {
                    unreachable!("Unsupported compilation subcommand",)
                }
            }
        }
        Some(("jl", sub_matches)) => {
            let jl_command = sub_matches.subcommand().unwrap_or(("repl", sub_matches));
            // .expect("messed up gt jl command");

            match jl_command {
                ("pluto", _sub_matches) => {
                    // TODO! Need to check to make sure that Pluto is installed in the global env. first, or just install it in the environment...
                    //
                    // Run Pluto via `julia -E 'using Pluto; Pluto.run()'` command
                    //
                    //

                    run_pluto_nb(&mut julia);
                }

                // if run with `gt jl run`, then start the julia process using the current directory as the active environment
                ("repl", _sub_matches) => {
                    let julia_executable_string =
                        CString::new("julia").expect("CString::new failed...");
                    let julia_executable = julia_executable_string.as_c_str();

                    let julia_args_julia = CString::new("julia").expect("CString::new failed...");

                    let local_jl_project_toml = PathBuf::from("./Project.toml");
                    let parent_jl_project_toml = PathBuf::from("../Project.toml");
                    if local_jl_project_toml.exists() {
                        println!("Activating local project...");
                        let julia_args_project =
                            CString::new("--project=@.").expect("CString::new failed...");
                        execvp(julia_executable, &[julia_args_julia, julia_args_project])
                            .expect("failed to exec Julia process...");
                    } else if parent_jl_project_toml.exists() {
                        println!("Activating parent project...");
                        let julia_args_project =
                            CString::new("--project=..").expect("CString::new failed...");
                        execvp(julia_executable, &[julia_args_julia, julia_args_project])
                            .expect("failed to exec Julia process...");
                    } else {
                        println!("No local project found...");
                        execvp(julia_executable, &[julia_args_julia])
                            .expect("failed to exec Julia process...");
                    }
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

            // TODO! Command / run template:
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
    };
}

fn main() -> Result<()> {
    // Signal_hook implementation of CTRL+C handler...
    // let term = Arc::new(AtomicBool::new(false));
    // signal_hook::flag::register(signal_hook::consts::SIGTERM, Arc::clone(&term));

    // while !term.load(Ordering::Relaxed) {

    //
    //
    // }
    //
    //
    // let running = Arc::new(AtomicBool::new(true));
    // let r = running.clone();

    // ctrlc::set_handler(move || {
    //     r.store(false, Ordering::SeqCst);
    //     println!("\nrunning.clone (r) = {:?}", &r);
    //     println!("Got Ctrl+C signal!");
    //     process::exit(1);
    // })
    // .expect("Error setting CTRL + C handler");

    // println!("running = {:?}", running);
    // while running.load(Ordering::SeqCst) {
    //     // println!("inside running.load....");
    //     // End CtrlC handling
    // } // end running.load

    //
    //
    // If Julia is not installed, install Julia using juliaup:
    //
    // Mac & Linux:
    // curl -fsSL https://install.julialang.org | sh
    //
    // Windows:
    // winget install julia -s msstore
    //
    let home_dir = home_dir().expect("Couldn't find the user's home directory");

    let mut dot_julia_dir = PathBuf::new();
    dot_julia_dir.push(&home_dir);
    dot_julia_dir.push(".julia");

    // if $HOME/.julia folder exists, then set to false and skip Julia installation; if .julia folder doesn't exist, set to true and execute the 'if' block to install Julia...
    if !(dot_julia_dir.exists()) {
        // for debugging...
        // if dot_julia_dir.exists() {
        println!("Couldn't find Julia on your system...");

        let install_options = vec!["Yes", "No"];
        let choice = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Would you like to install the Julia language?")
            .items(&install_options)
            .default(0)
            .interact_on_opt(&Term::stderr())?;

        // match on user's choice of yes / no to install Julia or not...
        match choice {
            // user selects "Yes"
            Some(0) => {
                // Install Julia on Linux / MacOS -- iff the .julia directory doesn't exist
                cmd!("curl", "-fsSL", "https://install.julialang.org")
                    .pipe(cmd!("sh"))
                    .run()?;

                // TODO! For Windows..
                // cmd!(sh, "winget install julia -s msstore").run()?;

                // TODO! Make this better / test this!!!
                println!("Please ensure that Julia is on your $PATH before continuing!",);
            }
            // user selects "No"
            Some(1) => {
                println!("You selected {:?}. Julia is required for this tool to work; please install Julia and add it to your $PATH before continuing...",install_options[1]
            );
                return Err(anyhow!("Unable to proceed; Julia language not installed"));
            }
            _ => unreachable!(),
        }
    }

    // Create `startup.jl` file, if not already present:
    let mut startup_jl_dir = PathBuf::new();
    startup_jl_dir.push(dot_julia_dir);
    startup_jl_dir.push("config");

    // if `./julia/config` dir does not exist, then create it and create startup.jl
    if !(startup_jl_dir.exists()) {
        // create the ./julia/config dir
        DirBuilder::new().recursive(true).create(&startup_jl_dir)?;

        let mut startup_jl_file_path = PathBuf::new();
        startup_jl_file_path.push(&startup_jl_dir);
        startup_jl_file_path.push("startup.jl");
        let mut startup_jl_file = fs::File::create(startup_jl_file_path)?;

        let github_username = dialoguer::Input::<String>::new()
            .with_prompt("What is your github.com username? \nIf you don't have a Github username, go create one now, then enter it here:")
            .interact()?;

        // get the user's github name for default package creation....

        const STARTUP_JL_FILE_CONTENTS_1: &str = r###"
function template()
    @eval begin
        using PkgTemplates
        Template(;
            user=""###;
        const STARTUP_JL_FILE_CONTENTS_2: &str = r###"",
            # uncomment and change the `dir` variable value if you want your packages created in a different directory
            # dir="~/.julia/dev", # Default Directory
            dir=".", # Maurice's default directory
            julia=v"1.8",
            plugins=[
                License(; name="MIT"),
                Git(; manifest=true, ssh=true),
                GitHubActions(; x86=true),
                Codecov(),
                Documenter{GitHubActions}(),
                Develop(),
            ])
    end
end

make_package = template()
        "###;
        write!(
            startup_jl_file,
            "{}{}{}",
            STARTUP_JL_FILE_CONTENTS_1, &github_username, STARTUP_JL_FILE_CONTENTS_2
        )?;
        println!("Wrote you a startup.jl file at ~/.julia/config/startup.jl");
    }
    // Once Julia is already installed...
    //
    let mut julia_pending = unsafe { RuntimeBuilder::new().start().expect("Could not init Julia") };

    let mut frame = StackFrame::new();
    let mut julia = julia_pending.instance(&mut frame);

    let julia_dir = PathBuf::from(".julia/maurice/Maurice.jl");
    let mut maurice_jl_path = PathBuf::new();
    maurice_jl_path.push(home_dir);
    maurice_jl_path.push(julia_dir);

    // Include some custom code defined in <file>.
    // This is safe because the included code doesn't do any strange things.

    // TODO! Create config which allows users to hack on the Maurice.jl file without overwriting the file every time the app starts up...

    if maurice_jl_path.exists() {
        let latest_jl_file_contents = JULIA_FILE_CONTENTS.to_string();

        let maurice_jl_file_contents =
            fs::read_to_string(&maurice_jl_path).expect("Couldn't read Maurice.jl file...");

        let update_file_maybe = latest_jl_file_contents.eq(&maurice_jl_file_contents);

        unsafe {
            if update_file_maybe {
                // println!("Maurice path exists @: {:?}", maurice_jl_path);
                julia
                    .include(maurice_jl_path)
                    .expect("Could not include file");
            } else {
                println!("Ensuring you have the latest Maurice.jl file. Writing Maurice.jl file to `$HOME/.julia/maurice/Maurice.jl`", );

                write_julia_script_to_disk()
                    .expect("couldn't write maurice.jl file to $HOME/.julia/maurice/Maurice.jl");

                julia
                    .include(maurice_jl_path)
                    .expect("Could not include file - please file a bug report!");
            }
        }
    } else {
        println!("Couldn't find Maurice.jl file. Writing Maurice.jl file to `$HOME/.julia/maurice/Maurice.jl`", );

        write_julia_script_to_disk()
            .expect("couldn't write Maurice.jl file to $HOME/.julia/maurice/Maurice.jl");

        unsafe {
            julia
                .include(maurice_jl_path)
                .expect("Could not include file - please file a bug report!");
        }
    }

    // CLI

    // let username = &github_username;
    // println!("username = {}", username);
    let matches = cli().get_matches();

    handle_cli(julia, matches);

    Ok(())
}
