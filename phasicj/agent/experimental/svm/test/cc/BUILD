load(
    "@rules_cc//cc:defs.bzl",
    "cc_library",
    "cc_test",
)

cc_library(
    name = "graal_isolate_dynamic_header",
    hdrs = ["//phasicj/agent/experimental/svm:graal_isolate_dynamic.h"],
    strip_include_prefix = "/phasicj/agent/experimental/svm",
)

cc_library(
    name = "svm_dynamic_headers",
    hdrs = ["//phasicj/agent/experimental/svm:svm_dynamic.h"],
    deps = ["graal_isolate_dynamic_header"],
)

cc_test(
    name = "test_svm_dylib",
    srcs = ["test_svm_dylib.cc"],
    deps = [":svm_dynamic_headers"],
    linkopts = [
        "-ldl",
    ],
    args = [
        "$(rootpath //phasicj/agent/experimental/svm)",
        "$(rootpath //phasicj/agent/experimental/svm/test/data:SvmTestClass.class)",
    ],
    data = [
        "//phasicj/agent/experimental/svm",
        "//phasicj/agent/experimental/svm/test/data:SvmTestClass.class",
    ],
)
