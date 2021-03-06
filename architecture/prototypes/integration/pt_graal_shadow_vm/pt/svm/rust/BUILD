load("@dwtj_rules_java//experimental/graalvm:defs.bzl", "graalvm_native_image_library")
load(
    "@rules_rust//rust:rust.bzl",
    "rust_binary",
    "rust_library",
)
load("@rules_cc//cc:defs.bzl", "cc_import")

graalvm_native_image_library(
    name = "analysis_native_image_headers",
    dynamic_header_output = "analysis_dynamic.h",
    graal_isolate_dynamic_header_output = "graal_isolate_dynamic.h",
    graal_isolate_header_output = "graal_isolate.h",
    header_output = "analysis.h",
    library_output = "libanalysis.so",
    main_class = "pt.svm.Analysis",
    deps = ["//pt/svm:analysis"],
)

cc_import(
    name = "graalvm_native_image_cc_import",
    hdrs = [
        "analysis.h",
        "graal_isolate.h",
    ],
    shared_library = "libanalysis.so",
)

genrule(
    name = "analysis_bindgen",
    srcs = [
        "analysis.h",
        "graal_isolate.h",
    ],
    outs = ["analysis.rs"],
    cmd_bash = 'bindgen --output "$@" $(execpath :analysis.h) -- -I`dirname "$(execpath graal_isolate.h)"`',
)

rust_library(
    name = "analysis",
    srcs = [":analysis_bindgen"],
    rustc_flags = [
        "-A",
        "non-camel-case-types",
        "-A",
        "non-upper-case-globals",
    ],
)

rust_binary(
    name = "myapp",
    srcs = ["myapp.rs"],
    edition = "2018",
    deps = [
        ":analysis",
        ":graalvm_native_image_cc_import",
    ],
)

sh_test(
    name = "test",
    srcs = ["test.sh"],
    args = ["$(rootpath :myapp)"],
    data = [":myapp"],
)
