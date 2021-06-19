load("@rules_rust//rust:defs.bzl", "rust_binary")

alias(
    name = "server",
    actual = "pt_server",
)

rust_binary(
    name = "pt_server",
    srcs = ["server.rs"],
    deps = ["//third_party/cargo:tokio"],
    edition = "2018",
)

alias(
    name = "client",
    actual = "pt_client",
)

rust_binary(
    name = "pt_client",
    srcs = ["client.rs"],
    deps = ["//third_party/cargo:tokio"],
    edition = "2018",
)
