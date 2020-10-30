load("@io_bazel_rules_rust//cargo:cargo_build_script.bzl", "cargo_build_script")

def do_nothing_cargo_build_script(name = "do_nothing_cargo_build_script"):
    '''This declares a `cargo_build_script` target which does nothing.

    This macro is useful as part of a hack to support the `include_bytes!` and
    `include_str!` macros when the included files are Bazel-built. See [this
    GitHub issue][1] for some context on why the hack is needed and how it is
    used.

    [1]: https://github.com/bazelbuild/rules_rust/issues/459
    '''

    cargo_build_script(
        name = name,
        srcs = ["//bazel/util/rust/cargo_build_scripts:do_nothing.rs"],
    )
