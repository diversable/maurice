module Maurice
include("./Hooks.jl")


module Jl_Command
using Pkg


# Install Pluto Notebooks in global environment
function check_pluto_is_installed_jl()

    # Activate global scope
    Pkg.activate()

    if "Pluto" in keys(Pkg.project().dependencies)
        # println("Pluto is installed")
    else
        println("Adding Pluto to your global environment...")
        Pkg.add("Pluto")
    end

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
    if (isfile("./Project.toml") || isfile("../Project.toml"))
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
    # println("Dependencies are ready")
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

function new_script_in_target_dir(script_name::String)
    try
        Pkg.generate(script_name)
        Pkg.activate(script_name)
        # TODO! This function is fallible; fix this implementation!
        generate_docs(script_name)

        # return "New script created: $script_name"
        return 0
    catch
        # return "Unable to create script in the target directory"
        return 1
    end
end

function generate_docs(project_name)
    cd(project_name)
    Pkg.activate(project_name)

    DocumenterTools.generate()
end

function make_app_in_target_dir(app_name::String)
    try
        # Ensure PackageCompiler is in the global environment stack
        if ("PackageCompiler" in keys(Pkg.project().dependencies))
            println("PackageCompiler is ready to make your app...")
        else
            Pkg.add("PackageCompiler")
        end

        println("Getting your app ready...")
        Pkg.generate(app_name)
        Pkg.activate(app_name)
        try
            generate_docs(app_name)
        catch
            println("couldn't generate documentation folder for your app...")
        end

        return "success"

    catch
        return "Could not generate app"
    end
end

function make_pkg_in_target_dir(pkg_name::String)
    try
        # Ensure PackageTemplates is in the global environment stack
        if ("PkgTemplates" in keys(Pkg.project().dependencies))
            # println("Maurice is ready to generate your package...")
        else
            Pkg.add("PkgTemplates")
        end

        println("Getting your package ready...")


        make_package(pkg_name)

        return "success"

    catch
        return "Could not generate package"
    end
end



end # module New

module Create
using Pkg

# Ensure PackageCompiler is in the global environment stack
if ("PackageCompiler" in keys(Pkg.project().dependencies))
    # println("PackageCompiler is ready to make your app...")
else
    Pkg.add("PackageCompiler")
end
using PackageCompiler

function compile_app(source_code_path::String, target_directory_path::String)
    try
        create_app(source_code_path, target_directory_path)
        return "Your app has been compiled! You can find the executable in the $target_directory_path/bin folder"
    catch
        return "Could not compile app :(  Please provide valid source and target directories"
    end

end

end # module Create

module Test_Command
using Pkg

function run_tests()

    # move into the "./test" directory...
    cd("test")

    # activate tests package
    Pkg.activate(".")

    # Ensure Test is in Project.toml deps
    if ("Test" in keys(Pkg.project().dependencies))
        println("Maurice is ready to test your project...")
    else
        Pkg.add("Test")
    end

    try
        # Move back to root dir to test package
        cd("..")
        # Activate Main project prior to testing...
        Pkg.activate(".")
        Pkg.test()
        return "done"
    catch
        return "failed to test project..."
    end

end

end # module Test_Command

end # module Maurice

