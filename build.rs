// This compiles mathlib.c and links the result with the Rust binary
fn main() {
    cc::Build::new()
        .file("native/mathlib.c")
        .compile("mathlib");
}
