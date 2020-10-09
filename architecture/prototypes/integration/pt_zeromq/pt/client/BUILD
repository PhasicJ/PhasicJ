load("@dwtj_rules_java//java:defs.bzl", "dwtj_java_binary")
load("@dwtj_rules_java//graalvm:defs.bzl", "graalvm_native_image_binary")

dwtj_java_binary(
    name = "client_java_binary",
    main_class = "pt.client.Client",
    srcs = ["Client.java"],
)

graalvm_native_image_binary(
    name = "client_native",
    main_class = "pt.client.Client",
    deps = [":client_java_binary"],
    visibility = ["//visibility:public"],
    output = "client",
)
