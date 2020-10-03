load("@rules_cc//cc:defs.bzl", "cc_binary")

cc_binary(
    name = "agent",
    srcs = ["agent.cc"],
    linkshared = True,
    deps = [
        "//third_party/net/java/jdk:jni_headers",
        "//third_party/net/java/jdk:jvmti_headers",
    ],
)
