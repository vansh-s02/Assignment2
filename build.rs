fn main() {
    // Tell Cargo to rerun the build script if the .proto file changes
    // println!("cargo:rerun-if-changed=src/person.proto");

    // Use prost-build to compile the person.proto file into Rust code
    prost_build::Config::new()
        .out_dir("src/generated")  // Output directory for the generated Rust code
        .compile_protos(
            &["src/person.proto"],  // Path to the .proto file
            &["src"],                // Add the `src` directory as the proto path
        )
        .expect("Failed to compile protos");
}
