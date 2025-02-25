use std::env;
use std::path::PathBuf;
use std::process::Command;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let expected = {
        let mut buf = PathBuf::new();
        buf.push("target");
        buf.push("wasm32-unknown-unknown");
        buf.push(env::var("PROFILE").unwrap());
        buf.push("client.wasm");
        buf
    };
    Command::new("wasm-bindgen")
        .args(["--out-dir", &out_dir])
        .args(["--target", "web"])
        .arg("--no-typescript")
        .args(["--out-name", "client"])
        .arg(expected.as_path())
        .output()
        .unwrap();
    println!("cargo::rerun-if-changed=target/wasm32-unknown-unknown/debug/client.wasm");
    println!("cargo::rerun-if-changed=target/wasm32-unknown-unknown/release/client.wasm");
}
