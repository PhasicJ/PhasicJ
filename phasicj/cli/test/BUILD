load("@dwtj_rules_java//java:defs.bzl", "dwtj_java_test")

dwtj_java_test(
    name = "smoke_test_cli_java_binary",
    main_class = "phasicj.cli.test.SmokeTest",
    srcs = ["SmokeTest.java"],
    deps = ["//phasicj/cli:phasicj_java_binary"],
)

sh_test(
    name = "smoke_test_cli_graalvm_native_image_binary",
    srcs = ["smoke_test.sh"],
    data = ["//phasicj/cli:exec"],
    args = ["$(rootpath //phasicj/cli:exec)"],
)
