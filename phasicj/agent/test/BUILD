load("@dwtj_rules_java//java:defs.bzl", "dwtj_java_binary")

dwtj_java_binary(
    name = "TestApp",
    srcs = ["TestApp.java"],
    main_class = "phasicj.agent.test.TestApp",
    output_jar = "TestApp.jar",
)

sh_test(
    name = "test",
    srcs = ["test.sh"],
    args = [
        "$(location @openjdk_linux_x64//:bin/java)",
        "$(location //phasicj/agent:libpjagent.so)",
        "$(location :TestApp.jar)",
    ],
    data = [
        "@openjdk_linux_x64//:bin/java",
        ":TestApp.jar",
        # NOTE(dwtj): Just including `//phasicj/agent:agent` doesn't work. The
        #  `sh_test` rule doesn't include it in the test's runfiles. See
        #  [bazelbuild/bazel#6783](https://github.com/bazelbuild/bazel/issues/6783).
        "//phasicj/agent:libpjagent.so",
    ],
)
