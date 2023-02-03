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
    # Try to activate locally scoped environment. If not found, fall back to global env.
    local_env_else_global_env()

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

function activate_env_in_target_dir(project_env_name::String)
    Pkg.activate(project_env_name)
    try
        Pkg.add("Test")
        Pkg.add("Documenter")
        return "New project created: $project_env_name"
    catch
        return "Could not add foundational packages for your project. Please try again when you're connected to the network..."
    end
end

function make_env_in_current_dir()
    # Activate project in current dir..
    Pkg.activate(".")

    # Must add a package in order to generate Project.toml file
    # So... add a package that *everyone* should use in their packages:

    Pkg.add("Documenter")

end

function make_project_in_defined_directory(directory::String)
    install_pkgtemplates()
    Pkg.activate(".")

    # Must add a package in order to generate Project.toml file
    # So... add a package that *everyone* should use in their packages:

    Pkg.add("Documenter")
end

function install_pkgtemplates()
    Pkg.activate()
    try
        Pkg.update("PkgTemplates")
        return "PkgTemplates is up to date"
    catch
        Pkg.add("PkgTemplates")
        return "Added PkgTemplates"
    end
end

end # module New

end # module Gaston