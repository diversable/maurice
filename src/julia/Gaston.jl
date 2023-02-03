module Gaston

module Jl_Command
using Pkg

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

    # Activate local scope environment
    Pkg.activate(".")
    status = Pkg.status(; IO=stdout)
    # no errors occurred
    return "Ready"

end

function status_global()
    # Activate global scope
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