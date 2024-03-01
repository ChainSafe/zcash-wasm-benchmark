fn main() {
    protobuf_codegen::Codegen::new()
        .includes(&["src/protos"])
        .input("src/protos/service.proto")
        .input("src/protos/compact_formats.proto")
        .out_dir("src/protos")
        .run_from_script();
    // println!("cargo:rerun-if-changed=src/protos/service.proto");
}
