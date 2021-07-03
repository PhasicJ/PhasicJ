load("@rules_rust//rust:defs.bzl", "rust_binary")

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
        "rtevents.rs"
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
    ],
)
