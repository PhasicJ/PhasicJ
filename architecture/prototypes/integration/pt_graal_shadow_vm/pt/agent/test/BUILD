load("@dwtj_rules_java//java:defs.bzl", "dwtj_java_binary")

dwtj_java_binary(
    name = "test_app",
    main_class = "pt.agent.test.TestApp",
    srcs = ["TestApp.java"],
    output_jar = "test_app.jar",
)

sh_test(
    name = "test",
    srcs = ["test.sh"],
    args = [
        "$(location @openjdk_linux_x64//:bin/java)",
        "$(location //pt/agent)",
        "$(location :test_app.jar)",
    ],
    data = [
        "@openjdk_linux_x64//:bin/java",
        "//pt/agent",
        ":test_app.jar",
    ],
)
