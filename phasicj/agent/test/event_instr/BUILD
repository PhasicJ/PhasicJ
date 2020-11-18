load("@dwtj_rules_java//java:defs.bzl", "dwtj_java_binary")
load("test.bzl", "test")

dwtj_java_binary(
    name = "TestApp",
    srcs = ["TestApp.java"],
    main_class = "phasicj.agent.test.event_instr.TestApp",
    output_jar = "TestApp.jar",
    deps = ["//phasicj/agent/rt"],
)

# NOTE(dwtj): There are two reasons why a `dwtj_java_test` isn't right for this
#  purpose.
#
#  1. The `@dwtj_rules_java` don't yet support JVMTI agents.
#  2. We don't want `//phasicj/agent/rt` to be added to the test's runtime
#     class-path; we want to test that the RT JAR is added to the boot class path
#     automatically by the JVMTI agent at JVM startup.
test(
    name = "test",
    java_exec = "//third_party/openjdk:java",
)

test(
    name = "test_with_openjdk_11",
    java_exec = "//third_party/openjdk/jdk11:java",
)
