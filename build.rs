fn main() {
    cxx_build::bridge("src/main.rs")
        .file("include/triangle.cpp")
        .include("/opt/homebrew/include")
        .flag_if_supported("-std=c++14")
        .compile("triangle");

    println!("cargo:rerun-if-changed=src/triangle.cpp");
    println!("cargo:rerun-if-changed=src/main.rs");
}
