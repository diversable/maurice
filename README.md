[![Julia-Rust](https://github.com/diversable/gaston/actions/workflows/julia-rust.yml/badge.svg?branch=main)](https://github.com/diversable/gaston/actions/workflows/julia-rust.yml)

NOTICE:

This software is under development and is currently in the 'Alpha' phase.




_Windows Compatibility_:

The CLI is currently not fully functional / tested on Windows; it works well on Linux though (and presumably Windows' WSL as well as MacOS, too).

Note:

The Julia language must be installed and on the user's path for this software to function properly. If Julia is not installed, the `gt` tool will ask if you want to install Julia (on Linux or MacOS). Julia install will be performed with [the `juliaup` command line tool](https://github.com/JuliaLang/juliaup).

---

The `gt` (Gaston) CLI has some useful functionality - so feel free to give it a try and give feedback on the commands / workflow! There's more to come!

The available commands are summarized here:
(Full descriptions are explicated below)

- gt
  - new | generate
    - script [?script_name]
    - app [?app_name]
    - package [?package_name]
  - compile | create
    - app (*in progress)
    <!-- - sysimage -->
  - jl
    - repl | run
    - pluto | notebook | nb
    - edit | code
  - pkg
    - status | list | ls
    - add [package_name]
    - remove | rm | delete [?package_name]
    - update [?package_name]

NOTE: Using partial commands also works with this CLI tool, so using a command like `gt p up` will work the same as the command `gt pkg update` and will update all packages; `gt j ed` will work the same as `gt jl edit` and will start up VSCode as well as start the Julia process in the terminal for you.

---

Some helpful workflow shortcuts with the `gt` (Gaston) CLI are:

- create an environment with default files & folders in the current working directory: `gt new` or `gt new env`

- Get the status of packages in your environment:
`gt pkg ls` or `gt p st` (or `gt pkg status` if you like typing)

- Install a package in the local environment:
  `gt p i <package_name>` or `gt p add <package_name>`
  (ie. gaston package install | add package)

- Start VSCode in the current directory and start the julia repl for interactive work on the side: `gt jl edit` or `gt j ed` (NB: currently, VSCode must already be installed for this command to work)

- Start a Pluto Notebook:
  `gt jl pluto` or `gt j p`


---

Currently, the CLI functions include:


> gt (new | generate)

=> _create a new environment & project structure; the CLI will ask for a project name_


> gt (new | generate) [?script_name]

=> _same as above: create a new environment & project structure; the CLI will ask for a project name if one is not provided_


> gt jl

=> _start Julia with the project in the current directory activated (default), or run the Julia Repl with the global env. if not in a Julia project directory_


> gt jl (repl | run)

=> _same as above: start Julia with the project in the current directory activated (default), or run the Julia Repl with the global env. if not in a Julia project directory_


> gt jl (pluto | notebook | nb)

=> _starts Pluto.jl, the notebook environment written in native Julia. If Pluto is not installed, gt will install it for you._


> gt jl (edit | code)

=> _open VSCode with the current directory, and start up a Julia process in the terminal for working / testing interactively as well; currently, VSCode must already be installed and on the the user's $PATH_


> gt pkg

=> _get status of installed packages. NB: all commands default to current local environment for adding/removing packages, and fall back to global environment if not working in a project directory_


> gt pkg (status | list | ls)

=> _get status of installed packages / list installed packages._


> gt pkg add [package_name]

=> _add a package from a Julia registry_


> gt pkg (remove | rm | delete) [package_name]

=> _remove an installed package; defaults to operating on local project environment, and falls back to global environment_


> gt pkg update [?package_name]

=> _update all packages if the 'package_name' is not provided, or update specific package in local environment if 'package_name' is given_


---


Building from source:

Rust language must be installed to compile this tool; Rust can be installed using [the rustup tool](https://rustup.rs/).

For non-Rust users: after cloning this repo from Github, the `gt` CLI tool can be compiled by typing `cargo build --release` onto the  command line in the project's root directory, and then copying the `gt` binary from the `(project root)/target/release/` folder into a directory on your $PATH (eg. `/usr/bin` or `~/bin`).

On Linux or Windows WSL, if you enter the `(project root)/target/release` directory, then you can use the command `cp ./gt ~/bin/` to add the binary to your $PATH (you may need to create the ~/bin/ directory if it doesn't exist). Then you're ready to start using the gt (Gaston) CLI tool.
