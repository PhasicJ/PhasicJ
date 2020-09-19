load("@rules_cc//cc:defs.bzl", "cc_binary")

cc_binary(
    name = "my_server",
    srcs = ["my_server.cc"],
    deps = [
        "//pt/my_protos:a_cc_proto",
        "//pt/my_services:greeter_cc_proto",
        "//pt/my_services:greeter_cc_grpc",
        "@com_github_grpc_grpc//:grpc++",
        "@com_github_grpc_grpc//:grpc++_reflection",
    ],
)
