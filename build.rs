use std::process::Command;
// Example custom build script.
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=js");
    println!("cargo:rerun-if-changed=build");
    Command::new("yarn")
        .args(["install"].iter())
        .output()
        .expect("Can't run yarn install");
    Command::new("yarn")
        .args(["build"].iter())
        .output()
        .expect("can't run yarn build");
}
