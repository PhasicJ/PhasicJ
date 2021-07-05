load("@rules_rust//rust:defs.bzl", "rust_binary")

alias(
    name = "sqlite_replay_client",
    actual = "phasicj_daemon_tests_sqlite_replay_client",
    visibility = ["//phasicj/daemon/tests:__subpackages__"],
)

rust_binary(
    name = "phasicj_daemon_tests_sqlite_replay_client",
    srcs = ["main.rs"],
    deps = [
        "//phasicj/services/rtevents/rust",
        "//third_party/cargo:tokio",
        "//third_party/cargo:tonic",
        "//third_party/cargo:prost",
        "//third_party/cargo:tower",
        "//third_party/cargo:clap",
        "//third_party/cargo:rusqlite",
    ],
)
