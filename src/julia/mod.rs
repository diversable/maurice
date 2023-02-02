//
// The Gaston.jl file contents should be written to the <home_dir>.julia/gaston/Gaston.jl file/folder
//
//
#![allow(dead_code)]

// use jlrs::prelude::*;
use dirs::home_dir;
use std::fs::File;
use std::io::prelude::*;
use std::io::{Error, Write};
use std::path::PathBuf;

// If the Julia script isn't in the proper place in the filesystem, write/output the julia code to a file named `Gaston.jl`
pub fn write_julia_script_to_disk() -> std::io::Result<()> {
    // unimplemented!();
    let home_dir = home_dir().expect("Couldn't find the user's home directory");
    let julia_dir = PathBuf::from(".julia/gaston/Gaston.jl");
    let mut gaston_path = PathBuf::new();
    gaston_path.push(home_dir);
    gaston_path.push(julia_dir);
    let gaston_jl_path = &gaston_path;

    let mut gaston_jl_file = File::create(gaston_jl_path)?;

    // let gaston_jl = (JULIA_FILE_CONTENTS);
    write!(gaston_jl_file, "{}", JULIA_FILE_CONTENTS)
}

const JULIA_FILE_CONTENTS: &str = r###"
module Gaston

module PkgAPI
using Pkg

#
# STATUS METHOD BEGIN
#
# function status()::Cint

#     # Pkg.status(; outdated=true, IO=stderr)
#     Pkg.status(; IOBuffer=String)
#     # no errors occurred
#     return 0

# end

function status()

    # Pkg.status(; outdated=true, IO=stderr)
    # status = Pkg.status(; IO=String)
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
    update = Pkg.update()
    return "Success"
end

# update specific package:
function update(pkgname::String)
    try
        update = Pkg.update(pkgname)
        return "Success"
    catch
        return "Package `$pkgname` not found"
    end
end

# update multiple packages:
function update(pkgnames::Vector{String})
    update = Pkg.update(pkgnames)
    return "Success"
end
#
# UPDATE METHODS END
#

function add_package(pkgname::String)
    try
        Pkg.add(pkgname)
        return "$pkgname added to project environment"
    catch
        return "Could not find $pkgname in the Julia Registry"
    end
end



function remove_package(pkgname::String)
    Pkg.rm(pkgname)
    return "$pkgname has been removed"
end


function activate_environment(project_env_name::String)
    Pkg.activate(project_env_name)
    Pkg.add("Documenter")
    return "New project created: $project_env_name"
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
