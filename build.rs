fn main() {
    // Some target(e.g. wasm32-unknown-unknown) won't have this flag
    // defined since it has not features.
    let features = std::env::var("CARGO_CFG_TARGET_FEATURE").unwrap_or_default();
    if features.contains("sse4.1") || features.contains("sse2") {
        cc::Build::new()
            .file("BLAKE2/sse/blake2b.c")
            .compile("libblake2b.a");
    } else {
        cc::Build::new()
            .file("BLAKE2/ref/blake2b-ref.c")
            .compile("libblake2b.a");
    }
}
