# Rust Proto Rules
Rust protobuf definitions for the [Proto](https://github.com/please-build/proto-rules) plugin, for the [Please](https://please.build) build system.

# Basic Usage
First, add the proto plugin and this plugin to your project:
```python
# BUILD
plugin_repo(
    name = "proto",
    revision = "<Some git tag, commit, or other reference>",
)

plugin_repo(
    name = "rust_proto",
    owner = "odonate",
    plugin = "rust-proto",
    revision = "<Some git tag, commit, or other reference>",
)
```

Then add rust to the list of language definitions for the proto plugin:
```ini
[Plugin "proto"]
LanguageDef = ///rust_proto//build_defs:rust_proto
```

You'll then need to add the [protobuf sdk](https://github.com/tokio-rs/prost), as well as the [gRPC sdk](https://github.com/hyperium/tonic) if you want to use `grpc_library()`. You can copy the `rust_crate()` rules from `third_party/rust/BUILD` in this repo to get started.

Now add the rust proto plugin to your .plzconfig:
```ini
[Plugin "rust_proto"]
Target = //plugins:rust_proto
ProtoPlugin = //third_party/rust:protoc_gen_prost
ProtoDep = //third_party/rust:prost
GrpcPlugin = //third_party/rust:protoc_gen_tonic
GrpcDep = //third_party/rust:tonic
```

The Rust plugin and Rust Proto plugin expects a specific package structure:
```ini
path/to/your/package/
├── BUILD
└── src
    ├── lib.rs || main.rs
    └── proto
        ├── BUILD
        ├── mod.rs
        └── service.proto
```

You can then use `proto_library()` or `grpc_library()` to generate Rust code for your .proto files:

```python
grpc_library(
    name = "proto",
    srcs = ["service.proto"],
    visibility = ["PUBLIC"],
    deps = [
        "//third_party/rust:prost",
        "//third_party/rust:tonic",
    ],
)

export_file(
    name = "mod",
    src = "mod.rs",
    visibility = ["PUBLIC"],
)
```

You must define a `mod.rs` module to expose the generated proto files:
```rust
pub mod your_proto_package {
    include!("your.proto.package.rs");
}

pub mod your_proto_package_tonic {
    include!("your.proto.package.rs");
    include!("your.proto.package.tonic.rs");
}
```

```python
rust_binary(
    name = "service_test",
    edition = "2021",
    root = "src/main.rs",
    deps = [
        "//path/to/your/package/src/proto:_proto#rust",
        "//path/to/your/package/src/proto:mod",
        "//third_party/rust:prost",
        "//third_party/rust:tonic",
    ],
)
```

See the [Proto](https://github.com/please-build/proto-rules) plugin for more information on these rules.

# Configuration
This plugin can be configured via the plugins section as follows:
```ini
[Plugin "rust_proto"]
SomeConfig = some-value
```
