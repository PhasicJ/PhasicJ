load("@dwtj_rules_markdown//markdown:defs.bzl", "markdown_library")
load("@dwtj_rules_java//java:defs.bzl", "dwtj_java_library")

markdown_library(
    name = "markdown",
    srcs = glob(["*.md"]),
)

dwtj_java_library(
    name = "instr",
    srcs = [
        "Instrumenter.java",
        "MonitorInsnInstrumenter.java",
    ],
    visibility = ["//phasicj/agent:__subpackages__"],
    deps = [
        "//third_party/asm",
    ],
)
