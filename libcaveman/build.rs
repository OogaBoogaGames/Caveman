use protobuf_codegen::Codegen;

fn main() {
    Codegen::new()
        .protoc()
        .cargo_out_dir("protobuf")
        .input("src/protobuf/Caveman.proto")
        .include("src/protobuf")
        .run_from_script();
}