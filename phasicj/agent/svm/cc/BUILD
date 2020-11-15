load(
    "@rules_cc//cc:defs.bzl",
    "cc_library",
    "cc_test",
)

cc_library(
    name = "graal_isolate_dynamic_header",
    hdrs = ["//phasicj/agent/svm:graal_isolate_dynamic.h"],
    strip_include_prefix = "/phasicj/agent/svm",
)

cc_library(
    name = "svm_dynamic_headers",
    hdrs = ["//phasicj/agent/svm:svm_dynamic.h"],
    deps = ["graal_isolate_dynamic_header"],
)

cc_test(
    name = "test",
    srcs = ["test.cc"],
    deps = [":svm_dynamic_headers"],
    linkopts = [
        "-ldl",
    ],
    args = [
        "$(rootpath //phasicj/agent/svm)",
        "$(rootpath //phasicj/agent/svm/test:SvmTestClass.class)",
    ],
    data = [
        "//phasicj/agent/svm",
        "//phasicj/agent/svm/test:SvmTestClass.class",
    ],
)
