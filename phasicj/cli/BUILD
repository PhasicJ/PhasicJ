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

dwtj_java_binary(
    name = "phasicj_java_binary",
    main_class = "phasicj.cli.Main",
    srcs = [
        "Main.java",
        "PjCommand.java",
        "PjEvaluator.java",
    ],
    deps = [
        "//third_party/picocli",
        # We use the picocli annotation processor to generate some files to
        # [assist `native-image`](https://picocli.info/#_annotation_processor).
        # These files configure `native-image` to deal with certain picocli
        # features (e.g., its use of reflection). Without this configuration,
        # picocli doesn't work within a `native-image`.
        "//third_party/picocli:picocli_codegen"
    ],
    javac_flags = [
        # The `project` option is used by the picocli annotation processor.
        # See [link](https://picocli.info/#_processor_option_project).
        "-Aproject=phasicj/phasicj-cli"
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
