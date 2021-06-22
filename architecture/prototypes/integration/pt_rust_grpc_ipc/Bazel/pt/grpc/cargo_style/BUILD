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
    edition = "2018",
)

rust_binary(
    name = "server",
    srcs = ["server.rs"],
    deps = [
        ":helloworld",
        "//third_party/cargo:tokio",
        "//third_party/cargo:tonic",
        "//third_party/cargo:prost",
        "//third_party/cargo:tower",

        # NOTE(dwtj): These two are needed for `async_stream::stream!` and
        # `tonic::transport::server::Router::serve_with_incoming()`.
        "//third_party/cargo:async_stream",
        "//third_party/cargo:futures",
    ],
    proc_macro_deps = [
        # Needed because `async fn` [isn't yet supported within traits][1].
        # [1]: https://rust-lang.github.io/async-book/07_workarounds/05_async_in_traits.html[]
        "//third_party/cargo:async_trait",
    ],
    edition = "2018",
)

rust_binary(
    name = "client",
    srcs = ["client.rs"],
    deps = [
        ":helloworld",
        "//third_party/cargo:tokio",
        "//third_party/cargo:tonic",
        "//third_party/cargo:prost",
        "//third_party/cargo:tower",
    ],
    edition = "2018",
)

sh_test(
    name = "helloworld_test",
    srcs = ["test.sh"],
    data = [
        ":server",
        ":client",
    ],
    args = [
        "$(rootpath :server)",
        "$(rootpath :client)",
    ],
    env = {"RUST_BACKTRACE": "1"},
    flaky = True,
)
