load("@dwtj_rules_java//java:defs.bzl", java_library = "dwtj_java_library")
load("@dwtj_rules_java//graalvm:defs.bzl", "graalvm_native_image_binary")

java_library(
    name = "cli_java",
    srcs = ["Cli.java"],
    deps = [
        "//phasicj/agent/instr",
        "//third_party/guava",
        "//phasicj/agent/rt:java_lang_object_amendment",
    ],
)

graalvm_native_image_binary(
    name = "cli",
    main_class = "phasicj.agent.instr.cli.Cli",
    deps = [":cli_java"],
    output = "phasicj_agent_instr",
    native_image_options = [
        "--no-fallback",
        "-H:IncludeResources=phasicj/agent/rt/JavaLangObjectAmendment.class",
    ],
    visibility = ["//phasicj/agent:__subpackages__"],
)
