load(
    "@rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_test",
)

alias(
    name = "rust",
    actual = "phasicj_agent_svm_rust",
    visibility = ["//phasicj/agent:__subpackages__"],
)

# TODO(dwtj): Consider rewriting this using a `@rules_rust` `bindgen` rule.
genrule(
    name = "svm_dynamic_bindgen",
    srcs = [
        "//phasicj/agent/svm:svm_dynamic.h",
        "//phasicj/agent/svm:graal_isolate_dynamic.h",
    ],
    outs = ["svm_dynamic.rs"],
    cmd_bash = 'bindgen --output "$@" $(execpath //phasicj/agent/svm:svm_dynamic.h) -- -I`dirname "$(execpath //phasicj/agent/svm:graal_isolate_dynamic.h)"`',
)

rust_library(
    name = "svm_raw",
    srcs = ["svm_dynamic.rs"],
    edition = "2018",
    rustc_flags = [
        "-A",
        "non-camel-case-types",
        "-A",
        "non-upper-case-globals",
    ],
    visibility = ["//phasicj/agent/svm:__subpackages__"],
)

rust_library(
    name = "phasicj_agent_svm_rust",
    srcs = [
        "lib.rs",
        "svm_function_types.rs",
    ],
    deps = [
        "//third_party/cargo:libloading",
        "//phasicj/agent/svm/rust:svm_raw",
    ],
    edition = "2018",
)

rust_test(
    name = "test",
    srcs = ["test.rs"],
    compile_data = [
        "//phasicj/agent/svm/test/data:SvmTestClass.class",
    ],
    data = [
        "//phasicj/agent/svm:libsvm.so",
    ],
    rustc_env = {
        # NOTE(dwtj): This file is read at test run-time.
        "SVM_LIBRARY_FILE_PATH": "$(rootpath //phasicj/agent/svm:libsvm.so)",

        # NOTE(dwtj): This file is read at compile-time and its bytes are
        #  embedded in the test executable.
        "SVM_TEST_CLASS_FILE_PATH": "$(execpath //phasicj/agent/svm/test/data:SvmTestClass.class)",
    },
    deps = [
        ":phasicj_agent_svm_rust",
    ],
    edition = "2018",
)
