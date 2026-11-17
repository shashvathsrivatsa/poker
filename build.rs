fn main() {
    cc::Build::new()
        .file("c/hand_index.c")
        .include("c")
        .flag("-w")
        .compile("hand_index");
}
