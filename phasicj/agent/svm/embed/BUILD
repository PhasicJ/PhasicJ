load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_test",
)
load("//bazel:util/files/hash_file.bzl", "sha1_hash_file")
load("//bazel:util/files/copy_file.bzl", "copy_file")

# NOTE(dwtj): We copy this file to abreviate its usage in this `BUILD` file and
#  to make the `include_bytes!` macro not need to look any further than this
#  package.
copy_file(
    name = "copy_libsvm_so",
    src = "//phasicj/agent/svm:libsvm.so",
    out = "libsvm.so",
)

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
    rustc_env = {
        "SVM_EXEC_PATH": "$(execpath :libsvm.so)",
        "SVM_SHA1_EXEC_PATH": "$(execpath :libsvm.so.sha1)",
    },
    visibility = ["//phasicj/agent:__subpackages__"],
)

rust_test(
    name = "test",
    srcs = ["test.rs"],
    edition = "2018",
    deps = ["phasicj_agent_svm_embed"],
)
