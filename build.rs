use std::path::PathBuf;

fn main() {
    let path = PathBuf::from("./src");
    dubbo_build::prost::configure()
        .output_dir(path)
        .compile(&["proto/helloworld.proto"], &["proto/"])
        .unwrap();
}
