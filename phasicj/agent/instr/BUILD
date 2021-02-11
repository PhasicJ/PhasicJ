load("//bazel:docs.bzl", "index_md")
load("@dwtj_rules_java//java:defs.bzl", "dwtj_java_library")

index_md()

dwtj_java_library(
    name = "instr",
    srcs = [
        "Amendment.java",
        "AsmConfig.java",
        "Instrumenter.java",
        "MonitorInsnInstrumenter.java",
    ],
    visibility = ["//phasicj/agent:__subpackages__"],
    deps = [
        "//third_party/asm",
        "//third_party/asm:asm_commons",
    ],
)
