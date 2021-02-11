load(
    "@dwtj_rules_java//java:defs.bzl",
    "dwtj_java_binary",
    "dwtj_java_test",
)

dwtj_java_test(
    name = "test",
    main_class = "third_party.picocli.test.Test",
    srcs = ["Test.java"],
    deps = [":checksum"],
)

dwtj_java_binary(
    name = "checksum",
    main_class = "third_party.picocli.test.CheckSum",
    srcs = [":CheckSum.java"],
    deps = ["//third_party/picocli"],
)
