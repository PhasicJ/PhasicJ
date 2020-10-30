load("@dwtj_rules_java//java:defs.bzl", "dwtj_java_binary")
load("test.bzl", "test")

dwtj_java_binary(
    name = "TestApp",
    srcs = ["TestApp.java"],
    main_class = "phasicj.agent.test.TestApp",
    output_jar = "TestApp.jar",
    deps = ["//phasicj/agent/rt"],
)

test(
    name = "test_with_openjdk_linux_x64",
    java_exec = "@openjdk_linux_x64//jdk:bin/java",
)

test(
    name = "test_with_openjdk_11_linux_x64",
    java_exec = "@openjdk_11_linux_x64//jdk:bin/java",
)

# NOTE(dwtj): This may be useful for debugging: this JVM has debug symbols.
# test(
#     name = "test_with_openjdk_15_ga_slowdebug",
#     java_exec = "@openjdk_15_ga_slowdebug//jdk:bin/java",
# )
