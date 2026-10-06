fn main() {
    tonic_prost_build::configure()
        .compile_protos(&["Proto/optionpractice.proto"], &["Proto"])
        .unwrap();
}