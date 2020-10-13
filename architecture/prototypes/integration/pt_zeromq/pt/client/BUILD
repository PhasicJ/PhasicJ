load("@dwtj_rules_java//java:defs.bzl", "dwtj_java_binary")
load("@dwtj_rules_java//graalvm:defs.bzl", "graalvm_native_image_binary")

dwtj_java_binary(
    name = "client_java_binary",
    srcs = ["Client.java"],
    main_class = "pt.client.Client",
)

graalvm_native_image_binary(
    name = "client_native",
    main_class = "pt.client.Client",
    output = "client",
    visibility = ["//visibility:public"],
    deps = [":client_java_binary"],
)
