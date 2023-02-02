This software is a prototype.

Currently, Julia language must already be installed and on the user's path for this software to function properly.

The cli is currently not fully functional / tested on Windows; it works well on Linux though (and presumably Windows' WSL as well as MacOS, too).


Otherwise, the cli has some useful functionality already - feel free to give it a try and give feedback on the API!

Currently, the CLI commands looks something like this:
- gsn
  - new
    - env [env name]: create a new environment in the current directory (default) or with the specified env name
  - jl: start Julia with current directory environment activated
    - run: (same as above - start Julia with curr. dir. activated)
    - pluto: start Pluto.jl (the notebook environment written in Julia)
    - edit: open VSCode with the current directory, and start up a Julia process in the terminal for working / testing interactively
  - pkg: get status of packages
	- status: get status of packages
	- add [package_name]: add a package from the Julia registry
	- remove | rm [package_name]: remove package
	- update [?package_name]: update all packages (if package_name is not provided), or update specific package in global environment

Using partial commands also works with this CLI tool, so using a command like `gsn p up` will work to update all packages, or `gsn j ed` will start up VSCode and start Julia in the terminal for you.
