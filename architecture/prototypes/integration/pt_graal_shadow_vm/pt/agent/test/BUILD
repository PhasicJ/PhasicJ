load("@dwtj_rules_java//java:defs.bzl", "dwtj_java_binary")

dwtj_java_binary(
    name = "test_app",
    srcs = ["TestApp.java"],
    main_class = "pt.agent.test.TestApp",
    output_jar = "test_app.jar",
)

sh_test(
    name = "test",
    srcs = ["test.sh"],
    args = [
        "$(location @graalvm_linux_x64//:bin/java)",
        "$(location //pt/agent)",
        "$(location :test_app.jar)",
    ],
    data = [
        ":test_app.jar",
        "//pt/agent",
        "@graalvm_linux_x64//:bin/java",
    ],
)
