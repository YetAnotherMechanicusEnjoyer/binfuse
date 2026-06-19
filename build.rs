use std::{path::Path, process::Command};

fn main() {
    let zig_status = Command::new("zig")
        .args(["build", "-Dtarget=native"])
        .status()
        .expect("Failed to compile Zig library with build.zig");

    if !zig_status.success() {
        panic!("Zig compilation failed");
    }

    let lib_path = Path::new("zig-out/lib");

    println!("cargo:rustc-link-search={}", lib_path.display());
    println!("cargo:rustc-link-lib=static=binfuse_zig");

    println!("cargo:rerun-if-changed=zig/");
    println!("cargo:rerun-if-changed=build.zig");
}
