# NOTE(dwtj): Some references on `cc_grpc_library` macro:
#
#  - [Example](https://github.com/grpc/grpc/blob/f05d6ed3b9495b71c7b2c5922af834abe809103d/examples/BUILD#L37)
#  - [cc_grpc_library](https://github.com/grpc/grpc/blob/f05d6ed3b9495b71c7b2c5922af834abe809103d/bazel/cc_grpc_library.bzl#L7)

load("@rules_proto//proto:defs.bzl", "proto_library")
load("@rules_cc//cc:defs.bzl", "cc_proto_library")
load("@rules_java//java:defs.bzl", "java_proto_library")
load("@com_github_grpc_grpc//bazel:cc_grpc_library.bzl", "cc_grpc_library")
load("@com_github_grpc_grpc_java//:java_grpc_library.bzl", "java_grpc_library")

proto_library(
    name = "greeter_proto",
    srcs = ["greeter.proto"],
)

cc_proto_library(
    name = "greeter_cc_proto",
    visibility = ["//visibility:public"],
    deps = [":greeter_proto"],
)

cc_grpc_library(
    name = "greeter_cc_grpc",
    srcs = [":greeter_proto"],
    grpc_only = True,
    visibility = ["//visibility:public"],
    deps = [":greeter_cc_proto"],
)

java_proto_library(
    name = "greeter_java_proto",
    visibility = ["//visibility:public"],
    deps = [":greeter_proto"],
)

java_grpc_library(
    name = "greeter_java_grpc",
    srcs = [":greeter_proto"],
    visibility = ["//visibility:public"],
    deps = [":greeter_java_proto"],
)
