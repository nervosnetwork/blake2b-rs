fn main() {
    if is_x86_feature_detected!("sse4.1") || is_x86_feature_detected!("sse2") {
        cc::Build::new()
            .file("BLAKE2/sse/blake2b.c")
            .compile("libblake2b.a");
    } else {
    cc::Build::new()
        .file("BLAKE2/ref/blake2b-ref.c")
        .compile("libblake2b.a");
    }
}
