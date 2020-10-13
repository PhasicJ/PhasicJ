load("@rules_cc//cc:defs.bzl", "cc_binary")

cc_binary(
    name = "agent",
    srcs = ["agent.cc"],
    linkshared = True,
    visibility = ["//visibility:public"],
    deps = [
        "//pt/svm/exec/embedded:analysis",
        "@graalvm_linux_x64//cc/jni:headers",
        "@graalvm_linux_x64//cc/jvmti:headers",
    ],
)
