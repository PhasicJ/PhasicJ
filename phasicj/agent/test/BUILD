load("@dwtj_rules_java//java:defs.bzl", "dwtj_java_binary")

dwtj_java_binary(
    name = "TestApp",
    srcs = ["TestApp.java"],
    main_class = "phasicj.agent.test.TestApp",
    deps = ["//phasicj/agent/rt"],
    output_jar = "TestApp.jar",
)

sh_test(
    name = "test",
    srcs = ["test.sh"],
    args = [
        "$(rootpath @openjdk_linux_x64//jdk:bin/java)",
        # "$(rootpath @jdk_15_ga_slowdebug//jdk:bin/java)", # DEBUG
        "$(rootpath //phasicj/agent:libpjagent.so)",
        "$(rootpath :TestApp.jar)",
        "$(rootpath //phasicj/agent/svm:libsvm.so)",
    ],
    data = [
        "@openjdk_linux_x64//jdk:bin/java",
        # "@jdk_15_ga_slowdebug//jdk:bin/java", # DEBUG
        ":TestApp.jar",
        # NOTE(dwtj): Just including `//phasicj/agent:agent` doesn't work. The
        #  `sh_test` rule doesn't include it in the test's runfiles. See
        #  [bazelbuild/bazel#6783](https://github.com/bazelbuild/bazel/issues/6783).
        "//phasicj/agent:libpjagent.so",
        "//phasicj/agent/svm:libsvm.so",
    ],
)
