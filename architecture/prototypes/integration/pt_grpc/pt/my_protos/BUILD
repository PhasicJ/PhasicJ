load("@rules_proto//proto:defs.bzl", "proto_library")
load("@rules_cc//cc:defs.bzl", "cc_proto_library")

proto_library(
    name = "a_proto",
    srcs = ["a.proto"],
)

cc_proto_library(
    name = "a_cc_proto",
    deps = [":a_proto"],
    visibility = ["//visibility:public"],
)
