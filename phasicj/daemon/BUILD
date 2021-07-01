load("@rules_rust//rust:defs.bzl", "rust_binary")

package(
    default_visibility = ["//visibility:public"],
)

alias(
    name = "daemon",
    actual = "pjeventd",
)

rust_binary(
    name = "pjeventd",
    srcs = ["main.rs"],
    deps = ["//phasicj/services/rtevents/rust"],
)
