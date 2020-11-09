load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_test",
)

alias(
    name = "rust",
    actual = "svm",
    visibility = ["//phasicj/agent:__subpackages__"],
)

rust_library(
    name = "svm",
    srcs = ["lib.rs"],
    deps = [":svm_raw"],
    edition = "2018",
)

# TODO(dwtj): Consider rewriting this using a `@rules_rust` `bindgen` rule.
genrule(
    name = "svm_bindgen",
    srcs = [
        "//phasicj/agent/svm:svm.h",
        "//phasicj/agent/svm:graal_isolate.h",
    ],
    outs = ["svm_raw.rs"],
    cmd_bash = 'bindgen --output "$@" $(execpath //phasicj/agent/svm:svm.h) -- -I`dirname "$(execpath //phasicj/agent/svm:graal_isolate.h)"`',
)

rust_library(
    name = "svm_raw",
    srcs = ["svm_raw.rs"],
    edition = "2018",
    rustc_flags = [
        "-A",
        "non-camel-case-types",
        "-A",
        "non-upper-case-globals",
    ],
    deps = ["//phasicj/agent/svm/cc"],
)

rust_test(
    name = "test",
    srcs = ["test.rs"],
    rustc_env = {
        "SVM_TEST_CLASS_EXEC_PATH": "$(execpath //phasicj/agent/svm/test:SvmTestClass.class)"
    },
    edition = "2018",
    deps = [
        ":svm",
        "//phasicj/agent/svm/rust/embed",
    ],
    # NOTE(dwtj): This data isn't used at run-time. It is used at compile-time
    #  to embed these bytes into the test binary.
    data = ["//phasicj/agent/svm/test:SvmTestClass.class"],
)
