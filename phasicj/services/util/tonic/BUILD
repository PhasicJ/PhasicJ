load("@rules_rust//rust:defs.bzl", "rust_library")

package(
    default_visibility = [
        "//phasicj/services/rtevents/rust/test:__pkg__",
        "//phasicj/daemon:__pkg__",
    ],
)

alias(
    name = "tonic",
    actual = "phasicj_services_util_tonic",
)

rust_library(
    name = "phasicj_services_util_tonic",
    srcs = [
        "lib.rs",
        "uds.rs",
    ],
    deps = [
        "//third_party/cargo:tokio",
        "//third_party/cargo:tonic",
    ],
)
