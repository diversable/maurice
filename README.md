This software is a prototype.

Currently, Julia language must already be installed and on the user's path for this software to function properly.

The cli is currently not fully functional / tested on Windows; it works well on Linux though (and presumably Windows' WSL as well as MacOS, too).


Otherwise, the cli has some useful functionality already - feel free to give it a try and give feedback on the API!

Currently, the CLI commands looks something like this:
- gsn
  - new
    - env [env name]: create a new environment in the current directory (default) or with the specified env name
  - jl: start Julia with current directory environment activated
    - run [?environment_name]: (same as above) Start the Julia REPL with curr. dir. activated if no arg is supplied, or start the specified env by adding an argument
    - pluto: start Pluto.jl (the notebook environment written in Julia); currently, must have pluto installed in the global Julia environment for this command to function properly
    - edit: open VSCode with the current directory, and start up a Julia process in the terminal for working / testing interactively; currently, VSCOde must already be installed and on the the user's $PATH
  - pkg: get status of installed packages
	- status: get status of packages
	- add [package_name]: add a package from the Julia registry
	- remove | rm [package_name]: remove package
	- update [?package_name]: update all packages (if package_name is not provided), or update specific package in global environment

Using partial commands also works with this CLI tool, so using a command like `gsn p up` will work to update all packages, or `gsn j ed` will start up VSCode and start Julia in the terminal for you.
