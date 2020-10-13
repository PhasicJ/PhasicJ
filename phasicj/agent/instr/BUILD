load("@dwtj_rules_java//java:defs.bzl", "dwtj_java_library")

dwtj_java_library(
    name = "instr",
    srcs = [
        "Instrumenter.java",
        "MonitorInsnInstr.java",
    ],
    visibility = ["//phasicj/agent:__subpackages__"],
    deps = [
        "//phasicj/agent/rt",
        "//third_party/asm",
    ],
)
