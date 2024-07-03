use std::{env, fs::File, path::PathBuf, io::Write};
use std::process::Command;

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

    let ldconfig = Command::new("/sbin/ldconfig")
        .args([ "-p" ])
        .output()
        .expect("unable to execute ldconfig");

    assert!(ldconfig.status.success());
    let ldconfig = String::from_utf8(ldconfig.stdout).unwrap();
    if ldconfig.contains("libsframe") {
        println!("cargo:rustc-link-lib=sframe");
    }

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

    let header_contents = std::fs::read_to_string("/usr/include/dis-asm.h").unwrap();

    let mut f = File::create(out_path.join("init_disassemble_info_snippet.rs"))
        .expect("Couldn't open file in OUT_DIR");
    
    write!(f, "{}", if header_contents.contains("fprintf_styled_ftype") {
        "#[inline(always)] pub unsafe fn init_disassemble_info(disasm_info: &mut sys::disassemble_info) {
            sys::init_disassemble_info(disasm_info, std::ptr::null_mut(), Some(sys::print_to_buffer), Some((sys::print_to_styled_buffer)))
        }"
    } else {
        "#[inline(always)] pub unsafe fn init_disassemble_info(disasm_info: &mut sys::disassemble_info) {
            sys::init_disassemble_info(disasm_info, std::ptr::null_mut(), Some(sys::print_to_buffer))
        }"
    }).unwrap();

    cc::Build::new()
        .file("c/print.c")
        .compile("print");
}