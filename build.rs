fn main() {
    protobuf_codegen::Codegen::new()
        .includes(&["protos"])
        .input("protos/service.proto")
        .input("protos/compact_formats.proto")
        .out_dir("src/codegen")
        .pure()
        .run_from_script();
    // println!("cargo:rerun-if-changed=src/protos/service.proto");
}
