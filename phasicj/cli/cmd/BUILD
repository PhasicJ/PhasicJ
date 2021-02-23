load("@dwtj_rules_java//java:defs.bzl", "dwtj_java_library")

dwtj_java_library(
    name = "cmd",
    srcs = [
        "Pj.java",
        "PjAgent.java",
        "PjAgentInstrumentClass.java",
    ],
    deps = [
        "//third_party/picocli",
        # We use the picocli annotation processor to generate some files to
        # [assist `native-image`](https://picocli.info/#_annotation_processor).
        # These files configure `native-image` to deal with certain picocli
        # features (e.g., its use of reflection). Without this configuration,
        # picocli doesn't work within a `native-image`.
        "//third_party/picocli:picocli_codegen",
    ],
    javac_flags = [
        # The `project` option is used by the picocli annotation processor.
        # See [link](https://picocli.info/#_processor_option_project).
        "-Aproject=phasicj/phasicj-cli-cmd",
    ],
    visibility = ["//phasicj/cli:__subpackages__"],
)
