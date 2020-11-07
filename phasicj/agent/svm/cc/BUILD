load(
    "@rules_cc//cc:defs.bzl",
    "cc_import",
    "cc_library",
    "cc_test",
)

alias(
    name = "cc",
    actual = "svm",
    visibility = ["//phasicj/agent:__subpackages__"],
)

cc_library(
    name = "svm",
    deps = [
        "graal_isolate_dynamic_header",
        "svm_cc_import",
    ],
    visibility = ["//phasicj/agent:__subpackages__"],
)

cc_library(
    name = "graal_isolate_dynamic_header",
    hdrs = ["//phasicj/agent/svm:graal_isolate_dynamic.h"],
    strip_include_prefix = "/phasicj/agent/svm",
)

cc_import(
    name = "svm_cc_import",
    hdrs = ["//phasicj/agent/svm:svm_dynamic.h"],
    shared_library = "//phasicj/agent/svm:libsvm.so",
)

cc_test(
    name = "test",
    srcs = ["test.cc"],
    deps = [":cc"],
    linkopts = ["-ldl"],
    args = [
        "$(rootpath //phasicj/agent/svm:libsvm.so)",
        "$(rootpath //phasicj/agent/svm/test:SvmTestClass.class)",
    ],
    data = [
        "//phasicj/agent/svm:libsvm.so",
        "//phasicj/agent/svm/test:SvmTestClass.class",
    ],
)
