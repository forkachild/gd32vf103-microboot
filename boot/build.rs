fn main() {
    cc::Build::new()
        .file("src/startup.S")
        .compile("startup");

    println!("cargo:rerun-if-changed=src/startup.S");
}