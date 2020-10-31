load("@io_bazel_rules_rust//rust:rust.bzl", "rust_test")

rust_test(
    name = "test",
    srcs = ["test.rs"],
    deps = ["//third_party/cargo:log"],
    edition = "2018",
)
