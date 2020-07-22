use std::process::Command;

const TYPES_OUTPUT_EVENT_EDITOR: [&str; 1] = ["add_rectangle"];

fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=js");
    println!("cargo:rerun-if-changed=build");
    println!("cargo:rerun-if-changed=json_schemas");

    Command::new("yarn")
        .args(["install"].iter())
        .output()
        .expect("Can't run yarn install");

    for outgoing_ts_type in &TYPES_OUTPUT_EVENT_EDITOR {
        Command::new("yarn")
            .args(&[
                "create-type",
                &format!("./json_schemas/{}.json", outgoing_ts_type),
                &format!("./js/generated/outgoing_events/{}.d.ts", outgoing_ts_type),
            ])
            .output()
            .unwrap_or_else(|v| {
                panic!(
                    "Could not generate type from {:0}. Error {:?}",
                    outgoing_ts_type, v
                )
            });
    }

    Command::new("yarn")
        .args(["build"].iter())
        .output()
        .expect("can't run yarn build");
}
