use schemars::schema_for;
use silver_editor_event_types::Event;
use std::io::Write;
use std::process::{Stdio, Command};

fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=js");
    println!("cargo:rerun-if-changed=build");
    println!("cargo:rerun-if-changed=json_schemas");

    Command::new("yarn")
        .args(["install"].iter())
        .output()
        .expect("Can't run yarn install");

    let outgoing_types = vec![(schema_for!(Event), "incomming_events")];
    for (outgoing_ts_type, location) in outgoing_types {
        let x = Command::new("yarn")    
        .args(&[
                "create-type",
                "-o",
                &format!("./js/generated/{}.d.ts", location),
            ])
            .stdin(Stdio::piped())
            .spawn()
            .expect("Could not run commando to create ts type");
        match x.stdin {
            Some(mut x) => x
                .write_all(
                    &serde_json::to_vec(&outgoing_ts_type)
                        .expect("Could not deserialize generatedjson scheme"),
                )
                .expect("Could not write to ts type generation command"),
            None => panic!("Command to generate ts type has no stdin"),
        }
    }

    Command::new("yarn")
        .args(["build"].iter())
        .output()
        .expect("can't run yarn build");
}
