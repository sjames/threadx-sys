// Build script for Building threadx and to create the bindings

use std::io::{Write, BufRead};
use std::os::unix::fs::OpenOptionsExt;
use std::path::PathBuf;
use std::process::Command;
use std::env;
use cmake::Config;

fn main() {

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR is not set"));
    let src_path = out_dir.join("threadx"); // source code of threadx is vendored here
    let threadx_gh = "https://github.com/azure-rtos/threadx.git";
    let threadx_tag = "v6.3.0_rel";
    
    // Clone threadx
    std::process::Command::new("git")
        .arg("clone")
        .arg(threadx_gh)
        .current_dir(out_dir.clone())
        .output()
    .expect("Failed to fetch git submodules!");

    // checkout tag
    std::process::Command::new("git")
        .arg("checkout")
        .arg(threadx_tag)
        .current_dir(src_path.clone())
        .output()
        .expect("Unable to checkout threadx tag");


    let target = env::var("TARGET").expect("TARGET is not set");

    /*
    # target = "thumbv6m-none-eabi"    # Cortex-M0 and Cortex-M0+
    # target = "thumbv7m-none-eabi"    # Cortex-M3
    # target = "thumbv7em-none-eabi"   # Cortex-M4 and Cortex-M7 (no FPU)
    # target = "thumbv7em-none-eabihf" # Cortex-M4F and Cortex-M7F (with FPU)
    # target = "thumbv8m.base-none-eabi"   # Cortex-M23
    # target = "thumbv8m.main-none-eabi"   # Cortex-M33 (no FPU)
    # target = "thumbv8m.main-none-eabihf" # Cortex-M33 (with FPU)
     */

    let build_commands = out_dir.join("build_commands.txt");

    // We create a wrapper script to capture the commands passed to the compiler
    let launcher_script = format!(r#"
    #!/bin/sh
    #echo "Wrapper $@"
    set -e
    echo "$@" >> {}
    exec $@
    "#, out_dir.join("build_commands.txt").display());

    let compiler_wrapper_path = out_dir.join("compiler_wrapper.sh");

    let _ = std::fs::remove_file(compiler_wrapper_path.as_path());

    let mut file = std::fs::OpenOptions::new().write(true).mode(0o700).create_new(true).open(compiler_wrapper_path.as_path()).expect("Unable to open wrapper");
    file.write_all(launcher_script.as_bytes()).expect("Unable to write wrapper");
    file.flush().expect("Unable to flush wrapper");
    drop(file);


    let toolchain_file = match target.as_str() {
        "thumbv6m-none-eabi" => "cmake/cortex_m0.cmake",
        "thumbv7m-none-eabi" => "cmake/cortex_m3.cmake",
        "thumbv7em-none-eabi" => "cmake/cortex_m4.cmake",
        "thumbv7em-none-eabihf" => "cmake/cortex_m7.cmake",
        "thumbv8m.base-none-eabi" => "cmake/cortex_m23.cmake",
        _ => {
            println!("cargo:error=Unsupported cortex M target: {}", target);
            panic!("Unsupported cortex M target: {}", target);
            
        }
    };

    // Build threadx
    let dst = Config::new(src_path.to_owned())
        .define("CMAKE_TOOLCHAIN_FILE", toolchain_file)
        .generator("Ninja")
        .build_target("threadx")
        .env("CMAKE_C_COMPILER_LAUNCHER", compiler_wrapper_path.as_path())
        .env("CMAKE_CXX_COMPILER_LAUNCHER", compiler_wrapper_path.as_path())
        .build().join("build");

    println!("cargo:info=threadx build completed and output at {}", dst.display());

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=threadx");

    // Parse the build_commands.txt file to find the include directories and other compiler flags
    let build_commands = std::fs::OpenOptions::new().read(true).open(build_commands.as_path()).expect("Unable to open build_commands.txt");
    let build_commands = std::io::BufReader::new(build_commands).lines();
    let mut include_dirs = Vec::new();
    let mut defines =  Vec::new();
    let mut compiler = None;

    for line in build_commands {
        if let Ok(line) = line {

            if compiler.is_none() {
                // get the compiler from the first line
                let compiler_cmd = line.split(" ").take(1).next().unwrap();
                compiler = Some(compiler_cmd.to_string());
            }

            for cmd in line.split(" ") {
                if cmd.starts_with("-I") {
                    let include_dir = cmd.trim_start_matches("-I");
                    include_dirs.push(include_dir.to_string());
                } else if cmd.starts_with("-D") {
                    let define = cmd.trim_start_matches("-D");
                    defines.push(define.to_string());
                }
            }
        }
    }

    include_dirs.sort();
    include_dirs.dedup();
    defines.sort();
    defines.dedup();

    let threadx_api_path = src_path.join("common/inc/tx_api.h");
    let bindings_path = out_dir.join("generated.rs");
    let mut bindings = bindgen::Builder::default()
        .header(threadx_api_path.to_str().unwrap())
        .use_core()
        .allowlist_function("_tx.*")
        .allowlist_recursively(true);
    for include_dir in include_dirs {
        bindings = bindings.clang_arg(format!("-I{}", include_dir));
    }
    for define in defines {
        bindings = bindings.clang_arg(format!("-D{}", define));
    }

    

    // Get the standard include paths from the compiler
    // Create an empty file to pass to the compiler
    std::fs::OpenOptions::new().create(true).truncate(true).write(true).open(out_dir.join("empty.c")).expect("Unable to create empty.c");
    let output = Command::new(compiler.unwrap())
        .arg("-xc")
        .arg("-E")
        .arg("-v")
        .arg(out_dir.join("empty.c"))
        .output()
        .expect("Unable to run compiler");

    let output = String::from_utf8(output.stderr).expect("Unable to parse compiler output");
    
    let mut inside_include_dirs = false;
    let mut compiler_include_dirs = Vec::new();
    for line in output.lines() {
        if line.contains("search starts here:") {
            inside_include_dirs = true;
            continue;
        }
        if line.contains("End of search list.") {
            inside_include_dirs = false;
            break;
        }
        if inside_include_dirs {
            compiler_include_dirs.push(line.trim().to_string());
        }
    }

    println!("Found compiler include dirs: {:?}", compiler_include_dirs);

    for include_dir in compiler_include_dirs {
        bindings = bindings.clang_arg(format!("-I{}", include_dir));
    }

    let bindings = bindings
        .generate()
        .expect("Unable to generate bindings");

    bindings.write_to_file(bindings_path.clone())
        .expect("Couldn't write bindings");
    // Copy the file to src/generated.rs to keep the documentation build happy
    std::fs::copy(PathBuf::from(bindings_path), PathBuf::from("src/generated.rs")).unwrap();
}