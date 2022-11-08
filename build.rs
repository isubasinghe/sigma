
fn main() {
    ::capnpc::CompilerCommand::new()
        .file("./src/proto/proto.capnp")
        .run()
        .expect("compiling schema");
}
