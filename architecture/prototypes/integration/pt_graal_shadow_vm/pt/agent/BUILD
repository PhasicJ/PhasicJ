load("@rules_cc//cc:defs.bzl", "cc_binary")

cc_binary(
    name = "agent",
    srcs = ["agent.cc"],
    linkshared = True,
    deps = [
        "@openjdk_linux_x64//cc/jni:headers",
        "@openjdk_linux_x64//cc/jvmti:headers",
        "//pt/svm/exec/embedded:analysis",
    ],
    visibility = ["//visibility:public"],
)
