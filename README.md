This software is pre-Alpha.

Currently, Julia language must already be installed and on the user's path for this software to function properly ().

Also, the project must be run with `cargo run -- <cmd>` at the moment (creating a build and trying to run the binary from the `target` directory will fail).

The command I'd like help with is

```sh
cargo run -- jl
```

The above command should replicate a call like this:

```sh
julia --project=@.
```
which should activate a Julia environment in the current directory.

You can check which Julia environment is active by pressing the `]` key while in the Julia Repl - if the name of your $PWD is there in the terminal, instead of the Julia version number, then it's working - but that's what I'm currently having trouble with...



