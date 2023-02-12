use dirs::home_dir;
use std::fs::{self, File};
// use std::io::prelude::*;
use std::io::Write;
use std::path::PathBuf;

// If the Julia script isn't in the proper place in the filesystem, write/output the julia code to a file named `Maurice.jl`
pub fn write_hooks_script_to_disk() -> std::io::Result<()> {
    // unimplemented!();
    let home_dir = home_dir().expect("Couldn't find the user's home directory");
    let maurice_dir = PathBuf::from(".julia/maurice/");

    let mut maurice_folder = PathBuf::new();
    maurice_folder.push(&home_dir);
    maurice_folder.push(&maurice_dir);

    let julia_file_path = PathBuf::from(".julia/maurice/Hooks.jl");
    let mut hooks_file_path = PathBuf::new();
    hooks_file_path.push(&home_dir);
    hooks_file_path.push(&julia_file_path);

    if maurice_folder.exists() {
        // println!("Found .julia/maurice/ folder")
    } else {
        let _dotjulia_maurice_dir = fs::create_dir(maurice_folder)
            .expect("Couldn't create $HOME/.julia/maurice/  directory");
    }

    // create Maurice.jl file in `$HOME/.julia/maurice/`
    let mut maurice_jl_file = File::create(hooks_file_path)?;

    // let maurice_jl = (JULIA_FILE_CONTENTS);
    write!(maurice_jl_file, "{}", JULIA_FILE_CONTENTS)
}

pub const JULIA_FILE_CONTENTS: &str = r###"
module Hooks
# All functions must return either a "0" ('success') or "1" (error) value in order to work with the Maurice (mce) app...

function new_script_posthook(script_name::String)
    try
        println("hello from the new script posthook! You gave me the script name: `$script_name`")
        # return "success"
        return 0
    catch
        # return "error"
        return 1
    end

end


end # module Hooks
"###;
