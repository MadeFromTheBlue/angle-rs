use std::process::Command;
use std::env;

fn main() {
    Command::new("protoc")
        .arg("src/netdata.proto")
        .args(&["--rust_out", "src/"])
        .status().unwrap();
}