load("@dwtj_rules_java//graalvm:defs.bzl", "graalvm_native_image_library")
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

rust_library(
    name = "svm",
    srcs = [
        "instr.rs",
        "lib.rs",
    ],
    edition = "2018",
    visibility = ["//visibility:public"],
    deps = ["svm_unsafe"],
)

# NOTE(dwtj): I am currently assuming that one cannot (or at least should not)
#  link multiple `native-image` outputs together into one library. So, this
#  `native-image` invocation should include all Java dependencies that need to
#  be included in the agent as as native code. Currently, it just includes one
#  dependency, `instr`.
graalvm_native_image_library(
    name = "svm_native_image",
    dynamic_header_output = "svm_dynamic.h",
    graal_isolate_dynamic_header_output = "graal_isolate_dynamic.h",
    graal_isolate_header_output = "graal_isolate.h",
    header_output = "svm.h",
    image_name = "phasicj_agent_svm",
    library_output = "libsvm.so",
    native_image_options = ["--no-fallback"],
    deps = [
        "//phasicj/agent/instr",
        "//third_party/asm",
    ],
)

# TODO(dwtj): Consider rewriting this using a `@rules_rust` `bindgen` rule.
genrule(
    name = "svm_bindgen",
    srcs = [
        ":svm.h",
        ":graal_isolate.h",
    ],
    outs = ["svm_unsafe.rs"],
    cmd_bash = 'bindgen --output "$@" $(location :svm.h) -- -I`dirname "$(location graal_isolate.h)"`',
)

rust_library(
    name = "svm_unsafe",
    srcs = ["svm_unsafe.rs"],
    edition = "2018",
    rustc_flags = [
        "-A",
        "non-camel-case-types",
        "-A",
        "non-upper-case-globals",
    ],
)
