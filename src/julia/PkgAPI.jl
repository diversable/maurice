module PkgAPI

using Pkg

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

function update()
    update = Pkg.update()
    return "Success"
end

# update specific package:
function update(pkgname::String)
    update = Pkg.update(pkgname)
    return "Success"
end

# update multiple packages:
function update(pkgnames::Vector{String})
    update = Pkg.update(pkgnames)
    return "Success"
end

function add_package(pkgname::String)
    Pkg.add(pkgname)
    return "$pkgname added to project environment"
end

function activate_environment(project_env_name::String)
    Pkg.activate(project_env_name)
    return "New project created: $project_env_name"
end

function remove_package(pkgname::String)
    Pkg.rm(pkgname)
    return "$pkgname has been removed"
end

# dbg!
# status()
# update(["CSV", "Makie"])

# debug!
# packages = ["CSV", "Makie"]
# println(packages)
# update(packages)


end # module
