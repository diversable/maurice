//
// The Gaston.jl file contents should be written to the <home_dir>.julia/gaston/Gaston.jl file/folder
//
//
#![allow(dead_code)]

// use jlrs::prelude::*;
use dirs::home_dir;
use std::fs::{self, File};
// use std::io::prelude::*;
use std::io::Write;
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

    if gaston_folder.exists() {
        // println!("Found .julia/gaston/ folder")
    } else {
        let _dotjulia_gaston_dir =
            fs::create_dir(gaston_folder).expect("Couldn't create $HOME/.julia/gaston/  directory");
    }

    // create Gaston.jl file in `$HOME/.julia/gaston/`
    let mut gaston_jl_file = File::create(gaston_file_path)?;

    // let gaston_jl = (JULIA_FILE_CONTENTS);
    write!(gaston_jl_file, "{}", JULIA_FILE_CONTENTS)
}

pub const JULIA_FILE_CONTENTS: &str = r###"
module Gaston

module Jl_Command
using Pkg


# Install Pluto Notebooks in global environment
function check_pluto_is_installed_jl()

    # Activate global scope
    Pkg.activate()

    if "Pluto" in keys(Pkg.project().dependencies)
        println("Pluto is installed")
    else
        println("Adding Pluto to your global environment...")
        Pkg.add("Pluto")
    end

    # try
    #     # update Pluto Notebook environment
    #     Pkg.update("Pluto")
    # catch
    #     # Add Pluto Notebook environment
    #     Pkg.add("Pluto")
    # end

    return "Pluto Notebooks is ready"
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

function local_env_else_global_env()
    if isfile("./Project.toml")
        activate_local_scope_environment()
    else
        activate_global_scope_environment()
    end
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

    # Try to activate locally scoped environment. If not found, fall back to global env.
    local_env_else_global_env()

    status = Pkg.status(; IO=stdout)
    # no errors occurred
    return "Ready"

end

function status_global()
    # Activate global scope env
    activate_global_scope_environment()

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
    # Try to activate locally scoped environment. If not found, fall back to global env.
    local_env_else_global_env()

    update = Pkg.update()
    return "Success"
end

# update specific package:
function update(pkgname::String)
    # Activate local scope environment
    local_env_else_global_env()

    try
        update = Pkg.update(pkgname)
        return "Success"
    catch
        return "Package `$pkgname` not found"
    end
end

# update multiple packages:
function update(pkgnames::Vector{String})
    # Try to activate locally scoped environment. If not found, fall back to global env.
    local_env_else_global_env()

    update = Pkg.update(pkgnames)
    return "Success"
end
#
# UPDATE METHODS END
#

function add_package(pkgname::String)
    # Try to activate locally scoped environment. If not found, fall back to global env.
    local_env_else_global_env()

    try
        Pkg.add(pkgname)
        return "$pkgname added to project environment"
    catch
        return "Could not find $pkgname in the Julia Registry"
    end
end



function remove_package(pkgname::String)
    # Try to activate locally scoped environment. If not found, fall back to global env.
    local_env_else_global_env()

    Pkg.rm(pkgname)
    return "$pkgname has been removed"
end

end # module PkgAPI

# New script / environment, app project, or package (using PkgTemplates)
module New
using Pkg

Pkg.activate()

if ("Documenter" in keys(Pkg.project().dependencies) && "DocumenterTools" in keys(Pkg.project().dependencies) && "PkgTemplates" in keys(Pkg.project().dependencies) && "Test" in keys(Pkg.project().dependencies))
    println("Dependencies are ready")
else
    println("Adding the packages necessary to set up your project...")
    Pkg.add("Documenter")
    Pkg.add("DocumenterTools")
    Pkg.add("PkgTemplates")
    Pkg.add("Test")
    Pkg.add("PackageCompiler")
end

using Documenter
using DocumenterTools
using PkgTemplates

function activate_script_in_target_dir(project_env_name::String)
    try
        Pkg.generate(project_env_name)
        Pkg.activate(project_env_name)
        # TODO! This function is fallible; fix this implementation!
        generate_docs(project_env_name)

        return "New script created: $project_env_name"
    catch
        return "Unable to create script in the target directory"
    end
end

function generate_docs(project_env_name)
    cd(project_env_name)
    Pkg.activate(project_env_name)

    DocumenterTools.generate()

end

function make_env_in_current_dir()
    # Activate project in current dir..
    Pkg.activate(".")

    # Must add a package in order to generate Project.toml file
    # So... add a package that *everyone* should use in their packages:

    Pkg.add("Documenter")

end

function make_app_in_target_dir(app_name::String)
    try
        Pkg.generate(app_name)
        Pkg.activate(app_name)
        generate_docs(app_name)

        if ("PackageCompiler" in keys(Pkg.project().dependencies))
            println("PackageCompiler is ready...")
        else
            Pkg.add("PackageCompiler")
        end


        # TODO: create the rest of the App req's - eg.
    catch
    end
end

# function make_project_in_defined_directory(directory::String)
#     install_pkgtemplates()
#     Pkg.activate(".")

#     # Must add a package in order to generate Project.toml file
#     # So... add a package that *everyone* should use in their packages:
#       yup

#     Pkg.add("Documenter")
# end

# function install_pkgtemplates()
#     Pkg.activate()
#     try
#         Pkg.update("PkgTemplates")
#         return "PkgTemplates is up to date"
#     catch
#         Pkg.add("PkgTemplates")
#         return "Added PkgTemplates"
#     end
# end

end # module New

end # module Gaston
"###;
