load("@dwtj_rules_java//java:defs.bzl", "dwtj_java_binary")

# This target exists to be run with `bazel run` for experimentation and
#  troubleshooting.
dwtj_java_binary(
    name = "exec",
    # TODO(dwtj): Remove this attribute and file once it is no longer necessary.
    srcs = ["DoNothing.java"],
    main_class = "org.renaissance.core.Launcher",
    deps = ["//third_party/renaissance"],
)
