
fn main() {
    capnpc::CompilerCommand::new()
        .src_prefix("sigma::shared::proto_capnp")
        .file("./src/shared/proto.capnp")
        .run().expect("compiler command");
}
