load("@rules_rust//rust:defs.bzl", "rust_binary")
load("//:phasicj_version.bzl", "PHASICJ_VERSION")

package(
    default_visibility = ["//visibility:public"],
)

alias(
    name = "daemon",
    actual = "pjeventsd",
)

rust_binary(
    name = "pjeventsd",
    srcs = [
        "main.rs",
        "rtevents.rs",
        "cli.rs",
    ],
    deps = [
        "//phasicj/services/rtevents/rust",
        "//phasicj/services/util/tonic",

        "//third_party/cargo:log",
        "//third_party/cargo:env_logger",

        "//third_party/cargo:tokio",
        "//third_party/cargo:tonic",
        "//third_party/cargo:prost",
        "//third_party/cargo:tower",

        # NOTE(dwtj): These two are needed for `async_stream::stream!` and
        # `tonic::transport::server::Router::serve_with_incoming()`.
        "//third_party/cargo:async_stream",
        "//third_party/cargo:futures",

        "//third_party/cargo:clap",

        "//third_party/cargo:rusqlite",
    ],
    rustc_env = {"PHASICJ_VERSION": PHASICJ_VERSION},
)
