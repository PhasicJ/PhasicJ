load("@rules_rust//rust:defs.bzl", "rust_binary")

rust_binary(
    name = "test_server",
    srcs = ["test_server.rs"],
    deps = [
        "//phasicj/services/rtevents/rust",
        "//phasicj/services/util/tonic",
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
)

rust_binary(
    name = "test_client",
    srcs = ["test_client.rs"],
    deps = [
        "//phasicj/services/rtevents/rust",
        "//third_party/cargo:tokio",
        "//third_party/cargo:tonic",
        "//third_party/cargo:prost",
        "//third_party/cargo:tower",
    ],
)

sh_test(
    name = "test",
    srcs = ["test.sh"],
    data = [
        ":test_server",
        ":test_client",
    ],
    args = [
        "$(rootpath :test_server)",
        "$(rootpath :test_client)",
    ],
    env = {"RUST_BACKTRACE": "1"},
    flaky = True,
)
