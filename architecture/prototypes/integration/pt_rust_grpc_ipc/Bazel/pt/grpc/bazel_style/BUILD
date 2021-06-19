load("@rules_rust//rust:defs.bzl", "rust_binary")
load("@rules_rust//proto:proto.bzl", "rust_grpc_library")

alias(
    name = "bazel_style",
    actual = "pt_grpc_helloworld",
)

rust_grpc_library(
    name = "pt_grpc_helloworld_rust_codegen",
    deps = ["//pt/grpc:helloworld_service"],
)

rust_binary(
    name = "pt_grpc_helloworld",
    srcs = ["test_server.rs"],
    deps = [":pt_grpc_helloworld_rust_codegen"],
)
