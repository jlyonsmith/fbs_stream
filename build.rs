use flatbuffers_build::BuilderOptions;
use std::path::Path;

fn main() {
    // Specify the directory containing your Flatbuffers schema files
    let schema_dir = Path::new("schemas");
    let mut schemas = vec![];

    // Add all .fbs files from the schema directory
    for entry in std::fs::read_dir(schema_dir).expect("Failed to read schema directory") {
        let entry = entry.expect("Failed to read directory entry");
        let path = entry.path();

        if path.extension().map_or(false, |ext| ext == "fbs") {
            schemas.push(path);
        }
    }

    // Tell Cargo to re-run this build script if any .fbs file changes
    println!("cargo:rerun-if-changed={}", schema_dir.to_str().unwrap());

    BuilderOptions::new_with_files(&schemas)
        .compile()
        .expect("Flatbuffers compilation failed");
}
