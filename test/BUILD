subinclude("///rust//build_defs:rust")

rust_test(
    name = "service_test",
    root = "src/lib.rs",
    deps = [
        "//test/src/proto:mod",
        "//test/src/proto:_proto#rust",
        "//third_party/rust:tonic",
        "//third_party/rust:prost",
    ],
    edition="2021",
)
