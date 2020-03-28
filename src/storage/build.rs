extern crate protoc_rust;

use protoc_rust::Customize;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    set_env();

    let proto_files = vec!["src/data.proto"];

    protoc_rust::run(protoc_rust::Args {
        input: &proto_files[..],
        out_dir: "src",
        includes: &[],
        customize: Customize {
            ..Default::default()
        },
    })?;
    Ok(())
}

#[cfg(target_os = "windows")]
fn set_env() {
//  Reference to https://doc.rust-lang.org/cargo/reference/build-scripts.html
    println!("cargo:rustc-env=CHIBIDB_ROOT_PATH=../");
    println!("cargo:rustc-env=CHIBIDB_SCHEME_PATH=../schemes/");
    println!("cargo:rustc-env=CHIBIDB_DATA_PATH=../data/");
}