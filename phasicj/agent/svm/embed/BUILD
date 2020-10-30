load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_test",
)
load("//bazel:util/files/hash_file.bzl", "sha1_hash_file")
load("//bazel:util/files/copy_file.bzl", "copy_file")
load("//bazel/util/rust/cargo_build_scripts:do_nothing.bzl", "do_nothing_cargo_build_script")

# NOTE(dwtj): We copy this file to abreviate its usage in this `BUILD` file and
#  to make the `include_bytes!` macro not need to look any further than this
#  package.
copy_file(
    name = "copy_libsvm_so",
    src = "//phasicj/agent/svm:libsvm.so",
    out = "libsvm.so",
)

do_nothing_cargo_build_script()

sha1_hash_file(
    name = "libsvm_so_sha1",
    src = "libsvm.so",
    out = "libsvm.so.sha1",
)

rust_library(
    name = "phasicj_agent_svm_embed",
    srcs = ["lib.rs"],
    data = [
        "libsvm.so",
        "libsvm.so.sha1",
    ],
    edition = "2018",
    visibility = ["//phasicj/agent:__subpackages__"],
    deps = [":do_nothing_cargo_build_script"],
)

rust_test(
    name = "test",
    srcs = ["test.rs"],
    edition = "2018",
    deps = ["phasicj_agent_svm_embed"],
)
