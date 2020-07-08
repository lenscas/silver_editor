use std::process::Command;
// Example custom build script.
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=js");
    Command::new("yarn install && yarn build")
        .output()
        .expect("Can't run yarn install && yarn build");
}
