// test/src/proto/mod.rs
pub mod test_proto {
    include!("test.proto.rs");
}

pub mod test_proto_tonic {
    include!("test.proto.rs");
    include!("test.proto.tonic.rs");
}
