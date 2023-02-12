[![Julia-Rust](https://github.com/diversable/Maurice/actions/workflows/julia-rust.yml/badge.svg?branch=main)](https://github.com/diversable/Maurice/actions/workflows/julia-rust.yml)

The `mce` (Maurice) CLI has useful functionality for beginners and advanced programmers - so feel free to give it a try and give feedback on the commands / workflow in the Github "issues".

Just be aware that this software is still under development and is currently in the 'Alpha' phase.


There's more to come!


_Windows Compatibility_:

The CLI is currently not fully functional / tested on Windows; it works well on Linux though (and presumably Windows' WSL as well as MacOS, too).

Note:

The Julia language must be installed and on the user's path for this software to function properly. If Julia is not installed, the `mce` tool will ask if you want to install Julia (on Linux or MacOS). Julia install will be performed with [the `juliaup` command line tool](https://github.com/JuliaLang/juliaup).

---



The available commands are summarized here:
(Full descriptions are explicated below)

- mce
  - new | generate
    - script [?script_name]
    - app [?app_name]
    - package [?package_name]
  - test
    - run
  - compile | create
    - app [app_src_dir] [compiled_app_output]
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

NOTE: Using partial commands also works with this CLI tool, so using a command like `mce p up` will work the same as the command `mce pkg update` and will update all packages; `mce j ed` will work the same as `mce jl edit` and will start up VSCode as well as start the Julia process in the terminal for you.

---

Some helpful workflow shortcuts with the `mce` (Maurice) CLI are:

- create an environment with default files & folders in the current working directory: `mce new` or `mce new env`

- Get the status of packages in your environment:
`mce pkg ls` or `mce p st` (or `mce pkg status` if you like typing)

- Install a package in the local environment:
  `mce p i <package_name>` or `mce p add <package_name>`
  (ie. Maurice package install | add package)

- Start VSCode in the current directory and start the julia repl for interactive work on the side: `mce jl edit` or `mce j ed` (NB: currently, VSCode must already be installed for this command to work)

- Start a Pluto Notebook:
  `mce jl pluto` or `mce j p`


---

Currently, the CLI functions include:

##### mce (new | generate)
> mce (new | generate)

create a new environment & project structure; the CLI will ask for a project name


> mce (new | generate) script [?script_name]

same as above: create a new environment & project structure; the CLI will ask for a project name if one is not provided


> mce (new | generate) app [?app_name]

create a new app project structure; the CLI will ask for an app name if one is not provided


> mce (new | generate) package [?package_name]

create a new package, using PkgTemplates.jl (and the template in the startup file written to ./julia/config); the CLI will ask for a package name if one is not provided


##### mce jl

> mce jl

start Julia with the project in the current directory activated (default), or run the Julia Repl with the global env. if not in a Julia project directory


> mce jl (repl | run)

same as above: start Julia with the project in the current directory activated (default), or run the Julia Repl with the global env. if not in a Julia project directory


> mce jl (pluto | notebook | nb)

starts Pluto.jl, the notebook environment written in native Julia. If Pluto is not installed, mce will install it for you.


> mce jl (edit | code)

open VSCode with the current directory, and start up a Julia process in the terminal for working / testing interactively as well; currently, VSCode must already be installed and on the the user's $PATH


##### mce test

> mce test

run tests defined in the ./test/runtests.jl file


> mce test run

same as above; run tests defined in the ./test/runtests.jl file

##### mce (compile | create)

> mce compile app [path_to_app_source] [path_for_compilation_output]

Compile an app into an executable which can be run without Julia being installed

##### mce pkg

> mce pkg

get status of installed packages. NB: all commands default to current local environment for adding/removing packages, and fall back to global environment if not working in a project directory


> mce pkg (status | list | ls)

get status of installed packages / list installed packages.


> mce pkg add [package_name]

add a package from a Julia registry


> mce pkg (remove | rm | delete) [package_name]

remove an installed package; defaults to operating on local project environment, and falls back to global environment


> mce pkg update [?package_name]

update all packages if the 'package_name' is not provided, or update specific package in local environment if 'package_name' is given


---


###### Building from source:

Rust language must be installed to compile this tool; Rust can be installed using [the rustup tool](https://rustup.rs/).

For non-Rust users: after cloning this repo from Github, the `mce` CLI tool can be compiled by typing `cargo build --release` onto the  command line in the project's root directory, and then copying the `mce` binary from the `(project root)/target/release/` folder into a directory on your $PATH (eg. `/usr/bin` or `~/bin`).

On Linux or Windows WSL, if you enter the `(project root)/target/release` directory, then you can use the command `cp ./mce ~/bin/` to add the binary to your $PATH (you may need to create the ~/bin/ directory if it doesn't exist). Then you're ready to start using the mce (Maurice) CLI tool.
