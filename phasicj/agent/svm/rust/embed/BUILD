load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_test",
)
load("//bazel:util/files/hash_file.bzl", "sha1_hash_file")
load("//bazel:util/files/copy_file.bzl", "copy_file")

alias(
    name = "embed",
    actual = ":phasicj_agent_svm_rust_embed",
    visibility = ["//phasicj/agent:__subpackages__"],
)

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
    name = "phasicj_agent_svm_rust_embed",
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
)

rust_test(
    name = "test",
    srcs = ["test.rs"],
    rustc_env = {
        "SVM_TEST_CLASS_EXEC_PATH": "$(execpath //phasicj/agent/svm/test/data:SvmTestClass.class)"
    },
    deps = [
        ":phasicj_agent_svm_rust_embed",
        "//phasicj/agent/svm/rust",
    ],
    # NOTE(dwtj): This data isn't used at run-time. It is used at compile-time
    #  to embed these bytes into the test binary.
    data = ["//phasicj/agent/svm/test/data:SvmTestClass.class"],
    edition = "2018",
)
