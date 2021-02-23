load("//bazel:docs.bzl", "index_md")
load("@dwtj_rules_java//java:defs.bzl", "dwtj_java_binary")
load("@dwtj_rules_java//graalvm:defs.bzl", "graalvm_native_image_binary")

package(
    default_visibility = ["//phasicj/cli:__subpackages__"],
)

index_md()

alias(
    name = "exec",
    actual = ":phasicj",
    visibility = ["//visibility:public"],
)

alias(
    name = "cli",
    actual = "exec",
    visibility = ["//visibility:public"],
)

dwtj_java_binary(
    name = "phasicj_java_binary",
    main_class = "phasicj.cli.Main",
    srcs = [
        "Main.java",
    ],
    deps = [
        "//phasicj/cli/cmd",
        "//third_party/picocli",
    ],
)

graalvm_native_image_binary(
    name = "phasicj_graalvm_native_image_binary",
    main_class = "phasicj.cli.Main",
    deps = [":phasicj_java_binary"],
    visibility = ["//phasicj/cli:__subpackages__"],
    output = "phasicj",
    native_image_options = ["--no-fallback"],
)
