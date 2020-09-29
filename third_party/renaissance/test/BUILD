load("@dwtj_rules_java//java:defs.bzl", "dwtj_java_test")

# NOTE(dwtj): This target exists to make sure that we can fetch and run the
#  Renaissance benchmark suite.
dwtj_java_test(
    name = "scrabble",
    main_class = "org.renaissance.core.Launcher",
    args = ["--repetitions", "1", "scrabble"],
    # TODO(dwtj): Remove this attribute and file once it is no longer necessary.
    srcs = ["DoNothing.java"],
    deps = ["//third_party/renaissance"],
)
