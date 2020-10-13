load("@dwtj_rules_java//java:defs.bzl", "dwtj_java_test")

# NOTE(dwtj): This target exists to make sure that we can fetch and run the
#  Renaissance benchmark suite.
dwtj_java_test(
    name = "scrabble",
    # TODO(dwtj): Remove this attribute and file once it is no longer necessary.
    srcs = ["DoNothing.java"],
    args = [
        "--repetitions",
        "1",
        "scrabble",
    ],
    main_class = "org.renaissance.core.Launcher",
    deps = ["//third_party/renaissance"],
)
