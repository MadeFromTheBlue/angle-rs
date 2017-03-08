use std::process::Command;

fn main() {
    println!("Make sure you have installed protoc, run \"rust install protobuf\", and added your cargo bin to the path");
    Command::new("protoc")
        .arg("src/netdata.proto")
        .args(&["--rust_out", "src/"])
        .status().unwrap();
}