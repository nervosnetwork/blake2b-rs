fn main() {
    // Some target(e.g. wasm32-unknown-unknown) won't have this flag
    // defined since it has not features.
    let features = std::env::var("CARGO_CFG_TARGET_FEATURE").unwrap_or_default();
    let target_arch = std::env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    let bindings_builder = bindgen::Builder::default().use_core().ctypes_prefix("cty");

    if features.contains("sse4.1") || features.contains("sse2") {
        let bindings = bindings_builder
            .header("BLAKE2/sse/blake2.h")
            .generate()
            .expect("unable to generate bindings");
        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("unable to write bindings");

        cc::Build::new()
            .file("BLAKE2/sse/blake2b.c")
            .compile("libblake2b.a");
    } else if features.contains("neon") {
        let bindings = bindings_builder
            .header("BLAKE2/neon/blake2.h")
            .generate()
            .expect("unable to generate bindings");
        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("unable to write bindings");

        cc::Build::new()
            .file("BLAKE2/neon/blake2b.c")
            .compile("libblake2b.a");
    } else {
        let bindings = bindings_builder
            .header("BLAKE2/ref/blake2.h")
            .generate()
            .expect("unable to generate bindings");
        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("unable to write bindings");

        let mut build = cc::Build::new();
        if target_arch == "riscv64" {
            // Blake2b only requires a small part of libc, e.g., stdint.h for
            // common type definitions, as well as prototypes for memory related
            // functions. It's thus fine to use libc headers from picolibc as
            // stubs here.
            build.include("/usr/lib/picolibc/riscv64-unknown-elf/include");
        }
        build
            .file("BLAKE2/ref/blake2b-ref.c")
            .compile("libblake2b.a");
    }
}
