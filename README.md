NOTICE:

This software is under development and is currently in the 'Alpha' phase.

_Pre-Requisites_:

Currently, the Julia language must already be installed and on the user's path for this software to function properly.

FYI: the easiest way to install Julia is to use [the `Juliaup` command line tool](https://github.com/JuliaLang/juliaup).


_Windows Compatibility_:

The CLI is currently not fully functional / tested on Windows; it works well on Linux though (and presumably Windows' WSL as well as MacOS, too).

---

The `gsn` (Gaston) CLI does have some useful functionality already - so feel free to give it a try and give feedback on the commands / workflow!

The available commands are summarized here:
(Full descriptions are explicated below)

- gsn
  - new | generate
    - script
    - app (*in progress)
    <!-- - package (*yet to be implemented) -->
  - jl
    - repl | run
    - pluto | notebook | nb
    - edit
  - pkg
    - status | list | ls
    - add [package_name]
    - remove | rm | delete [package_name]
    - update [?package_name]

NOTE: Using partial commands also works with this CLI tool, so using a command like `gsn p up` will work the same as the command `gsn pkg update` and will update all packages; `gsn j ed` will work the same as `gsn jl edit` and will start up VSCode as well as start the Julia process in the terminal for you.

---

Some helpful workflow shortcuts with the `gsn` (Gaston) CLI are:

- create an environment with default files & folders in the current working directory: `gsn new` or `gsn new env`

- Get the status of packages in your environment:
`gsn pkg ls` or `gsn p st` (or `gsn pkg status` if you like typing)

- Install a package in the local environment:
  `gsn p i <package_name>` or `gsn p add <package_name>`
  (ie. gaston package install | add package)

- Start VSCode in the current directory and start the julia repl for interactive work on the side: `gsn jl edit` or `gsn j ed` (NB: currently, VSCode must already be installed for this command to work)

- Start a Pluto Notebook:
  `gsn jl pluto` or `gsn j p`


---

Currently, the CLI functions include:


> gsn (new | generate)

=> _create a new environment & project structure; the CLI will ask for a project name_


> gsn (new | generate) [?script_name]

=> _same as above: create a new environment & project structure; the CLI will ask for a project name if one is not provided_


> gsn jl

=> _start Julia with the project in the current directory activated (default), or run the Julia Repl with the global env. if not in a Julia project directory_


> gsn jl (repl | run)

=> _same as above: start Julia with the project in the current directory activated (default), or run the Julia Repl with the global env. if not in a Julia project directory_


> gsn jl (pluto | notebook | nb)

=> _starts Pluto.jl, the notebook environment written in native Julia. If Pluto is not installed, gsn will install it for you._


> gsn jl edit

=> _open VSCode with the current directory, and start up a Julia process in the terminal for working / testing interactively as well; currently, VSCode must already be installed and on the the user's $PATH_


> gsn pkg

=> _get status of installed packages. NB: all commands default to current local environment for adding/removing packages, and fall back to global environment if not working in a project directory_


> gsn pkg (status | list | ls)

=> _get status of installed packages / list installed packages._


> gsn pkg add [package_name]

=> _add a package from a Julia registry_


> gsn pkg (remove | rm | delete) [package_name]

=> _remove an installed package; defaults to operating on local project environment, and falls back to global environment_


> gsn pkg update [?package_name]

=> _update all packages if the 'package_name' is not provided, or update specific package in local environment if 'package_name' is given_


---


Building from source:

Rust language must be installed to compile this tool; Rust can be installed using [the rustup tool](https://rustup.rs/).

For non-Rust users: after cloning this repo from Github, the `gsn` CLI tool can be compiled by typing `cargo build --release` onto the  command line in the project's root directory, and then copying the `gsn` binary from the `(project root)/target/release/` folder into a directory on your $PATH (eg. `/usr/bin` or `~/bin`).

On Linux or Windows WSL, if you enter the `(project root)/target/release` directory, then you can use the command `cp ./gsn ~/bin/` to add the binary to your $PATH (you may need to create the ~/bin/ directory if it doesn't exist). Then you're ready to start using the gsn (Gaston) CLI tool.
