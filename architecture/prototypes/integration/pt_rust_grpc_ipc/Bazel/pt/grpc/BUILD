load("@rules_proto//proto:defs.bzl", "proto_library")

package(
    default_visibility = [
        "//pt/grpc/bazel_style:__pkg__",
        "//pt/grpc/cargo_style:__pkg__",
    ],
)

exports_files(
    ["helloworld_service.proto"],
)

proto_library(
    name = "helloworld_service",
    srcs = ["//pt/grpc:helloworld_service.proto"],
)
