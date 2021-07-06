load("@rules_rust//cargo:cargo_build_script.bzl", "cargo_build_script")
load("@rules_rust//rust:defs.bzl", "rust_library")

alias(
    name = "rust",
    actual = "phasicj_services_rtevents",
    visibility = ["//visibility:public"],
)

rust_library(
    name = "phasicj_services_rtevents",
    srcs = ["lib.rs"],
    deps = [
        ":cargo_build_script",
        "//third_party/cargo:tonic",
        "//third_party/cargo:prost",
        "//third_party/cargo:tower",
    ],
)

# NOTE(dwtj): We perform our own selection of the `protoc` executable based on
# our platform because `@rules_proto` has not implemented a toolchain to help us
# perform this selection.
# TODO(dwtj): Select between CPU architectures when more are supported in
# PhasicJ.
# TODO(dwtj): Figure out how to make this more concise.
cargo_build_script(
    name = "cargo_build_script",
    srcs = ["rtevents.build.rs"],
    deps = [
        "//third_party/cargo:tonic_build",
        "//third_party/cargo:prost_build",
    ],
    data = select({
        "@platforms//os:macos": [
            "rtevents_proto",
            "@com_google_protobuf_protoc_macos_x86_64//:protoc",
            "@rust_darwin_x86_64//:rustfmt_bin",
        ],
        "@platforms//os:linux": [
            "rtevents_proto",
            "@com_google_protobuf_protoc_linux_x86_64//:protoc",
            "@rust_linux_x86_64//:rustfmt_bin",
        ]
    }),
    build_script_env = select({
        "@platforms//os:macos": {
            "PROTOC": "$(execpath @com_google_protobuf_protoc_macos_x86_64//:protoc)",
            "RUSTFMT": "$(execpath @rust_darwin_x86_64//:rustfmt_bin)",
        },
        "@platforms//os:linux": {
            "PROTOC": "$(execpath @com_google_protobuf_protoc_linux_x86_64//:protoc)",
            "RUSTFMT": "$(execpath @rust_linux_x86_64//:rustfmt_bin)",
        },
    }),
)

alias(
    name = "rtevents_proto",
    actual = "//phasicj/services:rtevents.proto",
)
