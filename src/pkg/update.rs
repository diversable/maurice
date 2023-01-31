use jlrs::prelude::*;

/// if no arguments are supplied, update all packages by default....
pub fn update_all_packages(julia: &mut Julia) {
    println!("\nUpdating all packages installed...\n");

    let update = julia
        .scope(|mut frame| {
            let jl_module_main = Module::main(&mut frame);

            unsafe {
                jl_module_main
                    // the submodule doesn't have to be rooted because it's never reloaded.
                    .submodule(&mut frame, "Gaston")?
                    .submodule(&mut frame, "PkgAPI")?
                    // the same holds true for the function: the module is never reloaded so it's globally rooted
                    .function(&mut frame, "update")?
                    //
                    // CALLING A FUNCTION
                    //
                    // Call the function with the two arguments it takes
                    // .call2(&mut frame, dim, iters)
                    //
                    // .call0(&mut frame)
                    //
                    .call0(&mut frame)
                    //
                    // If you don't want to use the exception, it can be converted to a `JlrsError`
                    // In this case the error message will contain the message that calling `display` in Julia would show
                    .into_jlrs_result()?
                    // The function that was called returns a `Cint`, which can be unboxed as `i32`
                    // .unbox()::<i32>()
                    //
                    // unbox a function return type from Julia which is `Nothing`
                    // .unbox::<Nothing>()
                    .unbox::<String>()
            }
        })
        .expect("Result is an error");

    println!("Result: {:?}", update.unwrap());
}

/// if one argument is supplied - that is, a Julia package name arg - then use the `update_one` function...
pub fn update_one_package(julia: &mut Julia, package: &str) {
    println!("\nUpdating package\"{}\"\n", &package);

    let package = package.to_string();
    let update = julia
        .scope(|mut frame| {
            let jl_module_main = Module::main(&mut frame);

            let pkg = JuliaString::new(&mut frame, package);

            unsafe {
                jl_module_main
                    // the submodule doesn't have to be rooted because it's never reloaded.
                    .submodule(&mut frame, "Gaston")?
                    .submodule(&mut frame, "PkgAPI")?
                    // the same holds true for the function: the module is never reloaded so it's globally rooted
                    .function(&mut frame, "update")?
                    //
                    // CALLING A FUNCTION
                    //
                    // Call the function with the two arguments it takes
                    // .call2(&mut frame, dim, iters)
                    //
                    // .call0(&mut frame)
                    //
                    .call1(&mut frame, pkg.as_value())
                    //
                    // If you don't want to use the exception, it can be converted to a `JlrsError`
                    // In this case the error message will contain the message that calling `display` in Julia would show
                    .into_jlrs_result()?
                    // The function that was called returns a `Cint`, which can be unboxed as `i32`
                    // .unbox()::<i32>()
                    //
                    // unbox a function return type from Julia which is `Nothing`
                    // .unbox::<Nothing>()
                    .unbox::<String>()
            }
        })
        .expect("Result is an error");

    println!("Result: {:?}", update.unwrap());
}

// pub fn update_multiple(julia: &mut Julia, packages: Vec<&str>) {
//     println!("\nUpdating {:?}\n", &packages);

//     let update = julia
//         .scope(|mut frame| {
//             let jl_module_main = Module::main(&mut frame);

//             let pkg_vec = vec![];
//             for package in packages {
//                 pkg_vec.push(JuliaString::new(&mut frame, package))
//             }

//             // let pkgs = packages
//             //     .into_iter()
//             //     .map(|package| {
//             //         JuliaString::new(&mut frame, package);
//             //     })
//             //     .collect();

//             // let pkgs = JuliaString::new(&mut frame, packages);

//             unsafe {
//                 jl_module_main
//                     // the submodule doesn't have to be rooted because it's never reloaded.
//						.submodule(&mut frame, "Gaston")?
//                     .submodule(&mut frame, "PkgAPI")?
//                     // the same holds true for the function: the module is never reloaded so it's globally rooted
//                     .function(&mut frame, "update")?
//                     //
//                     // CALLING A FUNCTION
//                     //
//                     // Call the function with the two arguments it takes
//                     // .call2(&mut frame, dim, iters)
//                     //
//                     // .call0(&mut frame)
//                     //
//                     .call(&mut frame, pkg_vec)
//                     //
//                     // If you don't want to use the exception, it can be converted to a `JlrsError`
//                     // In this case the error message will contain the message that calling `display` in Julia would show
//                     .into_jlrs_result()?
//                     // The function that was called returns a `Cint`, which can be unboxed as `i32`
//                     // .unbox()::<i32>()
//                     //
//                     // unbox a function return type from Julia which is `Nothing`
//                     // .unbox::<Nothing>()
//                     .unbox::<String>()
//             }
//         })
//         .expect("Result is an error");

//     println!("Result: {:?}", update.unwrap());
// }
