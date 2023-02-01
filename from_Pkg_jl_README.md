

From Pkg.jl help docstring:


  Using the development version of Pkg.jl
  =========================================

  If you want to develop this package do the following steps:

    •  Make a fork and then clone the repo locally on your computer

    •  In line 2 of the Project.toml file (the line that begins with uuid = ...), modify the UUID, e.g. change the 44cf... to 54cf....

    •  Change the current directory to the Pkg repo you just cloned and start julia with julia --project.

    •  import Pkg will now load the files in the cloned repo instead of the Pkg stdlib .

    •  To test your changes, simply do include("test/runtests.jl").

    •  Before you commit and push your changes, remember to change the UUID in the Project.toml file back to the original UUID

  If you need to build Julia from source with a Git checkout of Pkg, then instead use make DEPS_GIT=Pkg when building Julia. The Pkg repo is in stdlib/Pkg, and created initially with a detached HEAD. If you're
  doing this from a pre-existing Julia repository, you may need to make clean beforehand.

  If you need to build Julia from source with Git checkouts of two or more stdlibs, please see the instructions in the Building Julia from source with a Git checkout of a stdlib
  (https://github.com/JuliaLang/julia/blob/master/doc/src/devdocs/build/build.md#building-julia-from-source-with-a-git-checkout-of-a-stdlib) section of the doc/src/devdocs/build/build.md
  (https://github.com/JuliaLang/julia/blob/master/doc/src/devdocs/build/build.md) file within the Julia devdocs.