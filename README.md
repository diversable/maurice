WARNING:

This software is under development and is currently in the prototype (pre-Alpha) phase.

_Pre-Requisites_:

Currently, Julia language must already be installed and on the user's path for this software to function properly.

FYI: the easiest way to install Julia is to use [the `Juliaup` command line tool](https://github.com/JuliaLang/juliaup).

Rust language must be installed to compile this tool; Rust can be installed using [the rustup tool](https://rustup.rs/).

For non-Rust users: after cloning this repo from Github, the `gsn` CLI tool can be compiled by typing `cargo build --release` onto the  command line in the project's root directory, and then copying the `gsn` binary from the `(project root)/target/release/` folder into a directory on your $PATH (eg. `/usr/bin`).

On Linux or Windows WSL, if you enter the `(project root)/target/release` directory, then you can use the command `cp ./gsn ~/bin/` to add the binary to your $PATH (you may need to create the ~/bin/ directory if it doesn't exist). Then you're ready to start using the gsn (Gaston) CLI tool.


Once the codebase is more fully-formed, proper builds will be available for download.

_Windows Compatibility_:

The CLI is currently not fully functional / tested on Windows; it works well on Linux though (and presumably Windows' WSL as well as MacOS, too).

---

The `gsn` (Gaston) CLI does have some useful functionality already - so feel free to give it a try and give feedback on the commands / workflow!

Currently, the CLI looks something like this:
- gsn
  - new
    - env [?env_name]  _(create a new environment in the current directory (default) or with the specified env name)_
  - jl 					 		   _( start Julia with current directory environment activated)_
    - run | repl [?environment_name]   _((same as above) Start the Julia REPL with current dir. activated if no arg is supplied, or start the specified env by adding an argument)_
    - pluto | notebook | nb   	_(start Pluto.jl (the notebook environment written in Julia); the Gaston CLI tool will ensure you have the latest version of Pluto in your global environment)_
    - edit						          _(open VSCode with the current directory, and start up a Julia process in the terminal for working / testing interactively; currently, VSCOde must already be installed and on the the user's $PATH)_
  - pkg 						_(get status of installed packages; all commands default to current local environment for adding/removing packages))_
	- status | list | ls			_(get status of packages / list packages)_
	- add [package_name]		  _(add a package from the Julia registry)_
	- remove | rm | delete [package_name] _(remove package)_
	- update [?package_name]	_(update all packages (if package_name is not provided), or update specific package in local environment)_

Using partial commands also works with this CLI tool, so using a command like `gsn p up` will work the same as the command `gsn pkg update` and will update all packages; `gsn j ed` will work the same as `gsn jl edit` and will start up VSCode as well as start the Julia process in the terminal for you.

---

Some helpful workflow shortcuts with the `gsn` (Gaston) CLI are:

- create an environment in the current working directory: `gsn new env` or `gsn n env`

- Get the status of packages in your environment:
`gsn p st`

- Install a package in the local environment:
  `gsn p i <package_name>` or `gsn p add <package_name>`
  (ie. gaston package install | add package)

- Start VSCode in the current directory and start the julia repl for interactive work on the side: `gsn jl edit` or `gsn jl ed` (NB: currently, VSCode must already be installed for this command to work)

