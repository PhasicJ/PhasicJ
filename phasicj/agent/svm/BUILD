load("@dwtj_rules_java//graalvm:defs.bzl", "graalvm_native_image_library")
load("@rules_cc//cc:defs.bzl", "cc_import")
load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")

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
    native_image_options = [
        "--no-fallback",
        # TODO(dwtj): Consider moving this into a `resource-config.json` file, possibly in the JAR
        #  which holds this class.
        "-H:IncludeResources=phasicj/agent/rt/JavaLangObjectAmendment.class",
    ],
    visibility = ["//phasicj/agent:__subpackages__"],
    deps = [
        "//phasicj/agent/instr",
        "//phasicj/agent/rt:java_lang_object_amendment",
        "//third_party/asm",
    ],
)

cc_import(
    name = "svm_native_image_cc_import",
    hdrs = [
        "graal_isolate.h",
        "svm.h",
    ],
    shared_library = "libsvm.so",
    visibility = ["//phasicj/agent:__subpackages__"],
)

# TODO(dwtj): Consider rewriting this using a `@rules_rust` `bindgen` rule.
genrule(
    name = "svm_bindgen",
    srcs = [
        ":svm.h",
        ":graal_isolate.h",
    ],
    outs = ["svm_raw.rs"],
    cmd_bash = 'bindgen --output "$@" $(execpath :svm.h) -- -I`dirname "$(execpath graal_isolate.h)"`',
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
    deps = [":svm_native_image_cc_import"],
)

rust_library(
    name = "svm",
    srcs = ["lib.rs"],
    deps = [":svm_raw"],
    edition = "2018",
    visibility = ["//phasicj/agent:__subpackages__"],
)
