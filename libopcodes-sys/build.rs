use std::{env, path::PathBuf};

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=/usr/include/dis-asm.h");
    println!("cargo:rerun-if-changed=c/print.c");
    println!("cargo:rerun-if-changed=c/print.h");
    println!("cargo:rustc-link-lib=opcodes");
    println!("cargo:rustc-link-lib=bfd");
    println!("cargo:rustc-link-lib=elf");
    println!("cargo:rustc-link-lib=iberty");
    println!("cargo:rustc-link-lib=dl");
    println!("cargo:rustc-link-lib=z");
    println!("cargo:rustc-link-lib=zstd");

    let bindings = bindgen::Builder::default()
        .header("/usr/include/dis-asm.h")
        .header("c/print.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .derive_default(true)
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    cc::Build::new()
        .file("c/print.c")
        .compile("print");
}