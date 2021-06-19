load("@rules_rust//rust:defs.bzl", "rust_binary")
load("@rules_rust//cargo:cargo_build_script.bzl", "cargo_build_script")

cargo_build_script(
    name = "helloworld",
    srcs = ["helloworld_build.rs"],
    deps = [
        "//third_party/cargo:tonic_build",
        "//third_party/cargo:prost_build",
    ],
    data = ["//pt/grpc:helloworld_service.proto"],
    build_script_env = {
        "PROTOC": "/usr/bin/protoc",
        "PROTOC_INCLUDE": "/usr/include",
    },
)

rust_binary(
    name = "test_server",
    srcs = ["test_server.rs"],
    deps = [
        ":helloworld",
        "//third_party/cargo:tokio",
        "//third_party/cargo:tonic",
        "//third_party/cargo:prost",
    ],
    proc_macro_deps = [
        # Needed because `async fn` [isn't yet supported within traits][1].
        # [1]: https://rust-lang.github.io/async-book/07_workarounds/05_async_in_traits.html[]
        "//third_party/cargo:async_trait",
    ],
)
