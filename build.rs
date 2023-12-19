// Build script for Building threadx and to create the bindings

use std::process::Command;
use std::env;
use cmake::Config;

fn main() {

    let out_dir = env::var("OUT_DIR").expect("OUT_DIR is not set");
    let src_path = "threadx"; // source code of threadx is vendored here

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
    let dst = Config::new(src_path)
        .define("CMAKE_TOOLCHAIN_FILE", toolchain_file)
        .generator("Ninja")
        .build_target("threadx")
        .build().join("build");

    println!("cargo:info=threadx build completed and output at {}", dst.display());

    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-lib=static=threadx");
}