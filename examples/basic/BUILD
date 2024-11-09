subinclude("///proto//build_defs:proto")

# proto_library(
#     name = "api_proto",
#     srcs = ["api.proto"],
#     languages = {
#         "rust": rust_proto_language(),
#     },
#     visibility = ["PUBLIC"],
#     deps = [
#         "//third_party/rust:prost",
#         "//third_party/rust:tonic",
#     ],
# )

grpc_library(
    name = "api_grpc",
    srcs = ["api.proto"],
    languages = {
        "rust": rust_grpc_language(),
    },
    visibility = ["PUBLIC"],
    deps = [
        "//third_party/rust:prost",
        "//third_party/rust:tonic",
    ],
)
