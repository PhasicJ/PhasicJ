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
    name = "test",
    java_exec = "//third_party/openjdk:java",
)

test(
    name = "test_with_openjdk_11",
    java_exec = "//third_party/openjdk/jdk11:java",
)

# NOTE(dwtj): This may be useful for debugging: this JVM has debug symbols.
# test(
#     name = "test_with_openjdk_15_ga_slowdebug",
#     java_exec = "@openjdk_15_ga_slowdebug//jdk:bin/java",
# )
