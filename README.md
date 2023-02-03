This software is in the prototype (pre-Alpha) phase.

Currently, Julia language must already be installed and on the user's path for this software to function properly.

The CLI is currently not fully functional / tested on Windows; it works well on Linux though (and presumably Windows' WSL as well as MacOS, too).


However, the `gsn` (Gaston) CLI does have some useful functionality already - so feel free to give it a try and give feedback on the commands / workflow!

Currently, the CLI commands looks something like this:
- gsn
  - new
    - env [?env_name] 			(description: create a new environment in the current directory (default) or with the specified env name)
  - jl 					 		(description: start Julia with current directory environment activated)
    - run | repl [?environment_name]   (description: (same as above) Start the Julia REPL with curr. dir. activated if no arg is supplied, or start the specified env by adding an argument)
    - pluto | notebook | nb   	(description: start Pluto.jl (the notebook environment written in Julia); the Gaston CLI tool will ensure you have the latest version of Pluto in your global environment)
    - edit						(description: open VSCode with the current directory, and start up a Julia process in the terminal for working / testing interactively; currently, VSCOde must already be installed and on the the user's $PATH)
  - pkg 						(description: get status of installed packages; all commands default to current local environment for adding/removing packages))
	- status					(description: get status of packages)
	- add [package_name]		(description: add a package from the Julia registry)
	- remove | rm [package_name] (description: remove package)
	- update [?package_name]	(description: update all packages (if package_name is not provided), or update specific package in local environment)

Using partial commands also works with this CLI tool, so using a command like `gsn p up` will work the same as the command `gsn pkg update` and will update all packages; `gsn j ed` will work the same as `gsn jl edit` and will start up VSCode as well as start the Julia process in the terminal for you.
