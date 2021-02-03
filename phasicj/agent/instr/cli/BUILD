load("@dwtj_rules_java//java:defs.bzl", java_library = "dwtj_java_library")
load("@dwtj_rules_java//graalvm:defs.bzl", "graalvm_native_image_binary")

java_library(
    name = "cli",
    srcs = ["Cli.java"],
    deps = ["//phasicj/agent/instr"],
)

graalvm_native_image_binary(
    name = "cli_native_image",
    main_class = "phasicj.agent.instr.cli.Cli",
    deps = [":cli"],
    output = "phasicj_agent_instr",
)
