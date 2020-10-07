load("@rules_cc//cc:defs.bzl", "cc_library")

licenses(["restricted"])  # GPLv2, portions of GPLv2 with "Classpath" exception

cc_library(
    name = "jni_headers",
    hdrs = ["@net_java_jdk_15_linux_x64//:include/jni.h"],
    deps = [":jni_md_headers"],
    strip_include_prefix = "/external/net_java_jdk_15_linux_x64/include",
    visibility = ["//visibility:public"],
)

cc_library(
    name = "jni_md_headers",
    hdrs = ["@net_java_jdk_15_linux_x64//:include/linux/jni_md.h"],
    strip_include_prefix = "/external/net_java_jdk_15_linux_x64/include/linux",
)

cc_library(
    name = "jvmti_headers",
    hdrs = [
        "@net_java_jdk_15_linux_x64//:include/jvmti.h",
        "@net_java_jdk_15_linux_x64//:include/jvmticmlr.h",
    ],
    strip_include_prefix = "/external/net_java_jdk_15_linux_x64/include",
    visibility = ["//visibility:public"],
)
