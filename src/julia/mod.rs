//
// The Gaston.jl file contents should be written to the <home_dir>.julia/gaston/Gaston.jl file/folder
//
//
#![allow(dead_code)]

// use jlrs::prelude::*;
use dirs::home_dir;
use std::fs::{self, File};
use std::io::prelude::*;
use std::io::{Error, Write};
use std::path::PathBuf;

// If the Julia script isn't in the proper place in the filesystem, write/output the julia code to a file named `Gaston.jl`
pub fn write_julia_script_to_disk() -> std::io::Result<()> {
    // unimplemented!();
    let home_dir = home_dir().expect("Couldn't find the user's home directory");
    let gaston_dir = PathBuf::from(".julia/gaston/");

    let mut gaston_folder = PathBuf::new();
    gaston_folder.push(&home_dir);
    gaston_folder.push(&gaston_dir);

    let julia_file_path = PathBuf::from(".julia/gaston/Gaston.jl");
    let mut gaston_file_path = PathBuf::new();
    gaston_file_path.push(&home_dir);
    gaston_file_path.push(&julia_file_path);

    let _dotjulia_gaston_dir =
        fs::create_dir(gaston_folder).expect("Couldn't create $HOME/.julia/gaston/ directory");

    // create Gaston.jl file in `$HOME/.julia/gaston/`
    let mut gaston_jl_file = File::create(gaston_file_path)?;

    // let gaston_jl = (JULIA_FILE_CONTENTS);
    write!(gaston_jl_file, "{}", JULIA_FILE_CONTENTS)
}

const JULIA_FILE_CONTENTS: &str = r###"
module Gaston

module Jl_Command
using Pkg

# Install Pluto Notebooks in global environment
function install_pluto_nb()

    # Activate global scope
    Pkg.activate()

    try
        # update Pluto Notebook environment
        Pkg.update("Pluto")
    catch
        # Add Pluto Notebook environment
        Pkg.add("Pluto")
    end

    return "Pluto Notebooks is up to date"
end

end # jl_command

module PkgAPI
using Pkg

# Activate global scope environment
function activate_global_scope_environment()
    Pkg.activate()
end

# Activate local scope environment
function activate_local_scope_environment()
    Pkg.activate(".")
end

#
# STATUS METHOD BEGIN
#
# function status()::Cint

#     # Pkg.status(; outdated=true, IO=stderr)
#     Pkg.status(; IOBuffer=String)
#     # no errors occurred
#     return 0

# end

# get the status of the project in the current dir
function status()

    # Pkg.status(; outdated=true, IO=stderr)
    # status = Pkg.status(; IO=String)

    if isfile("./Project.toml")
        activate_local_scope_environment()
    else
        activate_global_scope_environment()
    end


    status = Pkg.status(; IO=stdout)
    # no errors occurred
    return "Ready"

end

function status_global()
    # Activate global scope env
    Pkg.activate()

    status = Pkg.status(; IO=stdout)
    # no errors occurred
    return "Ready"
end
#
# STATUS METHOD END
#

#
# UPDATE METHODS BEGIN
#
# update all packages
function update()
    # Activate local scope environment
    Pkg.activate(".")

    update = Pkg.update()
    return "Success"
end

# update specific package:
function update(pkgname::String)
    # Activate local scope environment
    Pkg.activate(".")

    try
        update = Pkg.update(pkgname)
        return "Success"
    catch
        return "Package `$pkgname` not found"
    end
end

# update multiple packages:
function update(pkgnames::Vector{String})
    # Activate local scope environment
    Pkg.activate(".")

    update = Pkg.update(pkgnames)
    return "Success"
end
#
# UPDATE METHODS END
#

function add_package(pkgname::String)
    # Activate local scope environment
    Pkg.activate(".")

    try
        Pkg.add(pkgname)
        return "$pkgname added to project environment"
    catch
        return "Could not find $pkgname in the Julia Registry"
    end
end



function remove_package(pkgname::String)
    # Activate local scope environment
    Pkg.activate(".")

    Pkg.rm(pkgname)
    return "$pkgname has been removed"
end


function activate_environment(project_env_name::String)
    Pkg.activate(project_env_name)
    try
        Pkg.add("Test")
        Pkg.add("Documenter")
        return "New project created: $project_env_name"
    catch
        return "Could not add foundational packages for your project. Please try again when you're connected to the network..."
    end
end

function make_project_in_current_dir()
    # Activate project in current dir..
    Pkg.activate(".")

    # Must add a package in order to generate Project.toml file
    # So... add a package that *everyone* should use in their packages:

    Pkg.add("Documenter")

end

function make_project_in_defined_directory(directory::String)
    Pkg.activate(".")

    # Must add a package in order to generate Project.toml file
    # So... add a package that *everyone* should use in their packages:

    Pkg.add("Documenter")
end
# dbg!
# status()
# update(["CSV", "Makie"])

# debug!
# packages = ["CSV", "Makie"]
# println(packages)
# update(packages)


end # module PkgAPI

end # module Gaston
"###;
