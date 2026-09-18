use ethos_zero::{Actualizing, File, Generating, Potential};

fn main() {
    let root = std::path::PathBuf::from(std::env::var_os("CARGO_MANIFEST_DIR").expect("manifest"));
    println!("cargo:rerun-if-changed=ethos/signal.ethos");
    println!("cargo:rerun-if-changed=src/generated/signal.rs");
    let source = std::fs::read_to_string(root.join("ethos/signal.ethos")).expect("source");
    let file = Potential::<File>::from(source)
        .actualize()
        .unwrap_or_else(|_| panic!("read Signal"));
    let generated = file
        .generate()
        .unwrap_or_else(|_| panic!("generate Signal"));
    assert_eq!(
        generated,
        std::fs::read_to_string(root.join("src/generated/signal.rs")).expect("generated")
    );
}
