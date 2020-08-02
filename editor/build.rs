use schemars::schema_for;
use silver_editor_event_types::events::{Event, SendEvents};
use std::{
    env,
    io::Write,
    process::{Command, Stdio},
};

fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo:rerun-if-changed=js");
    println!("cargo:rerun-if-changed=build");

    Command::new("yarn")
        .args(["install"].iter())
        .output()
        .expect("Can't run yarn install");

    let types_to_generate = vec![
        (schema_for!(Event), "outgoing_events"),
        (schema_for!(SendEvents), "incomming_events"),
    ];

    for (type_to_generate, location) in types_to_generate {
        let as_bytes = serde_json::to_vec(&type_to_generate)
            .expect("Could not deserialize generatedjson scheme");

        let mut x = Command::new("yarn")
            .args(&[
                "create-type",
                "-o",
                &format!("./js/generated/{}.d.ts", location),
            ])
            .stdin(Stdio::piped())
            .spawn()
            .expect("Could not run commando to create ts type");
        match &mut x.stdin {
            Some(y) => y
                .write_all(&as_bytes)
                .expect("Could not write to ts type generation command"),
            None => panic!("Command to generate ts type has no stdin"),
        }
        
        x.wait().unwrap_or_else(|v|panic!("error while generating ts types : {:?}",v));
        
        std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(format!("./js/generated/{}.json", location))
            .expect("could not create/open json schem file")
            .write_all(&as_bytes)
            .expect("Could not write to json schema file");
    }
    let out_dir = env::var_os("OUT_DIR").unwrap();
    Command::new("yarn")
        .args(
            [
                "build",
                "--output-path",
                &out_dir
                    .into_string()
                    .expect("Couldn't turn the path into string"),
            ]
            .iter(),
        )
        .output()
        .expect("can't run yarn build");
}
