fn main() {
    println!("cargo:rerun-if-changed=src/cbst.c");
    cc::Build::new().file("src/cbst.c").compile("cbst");
}
