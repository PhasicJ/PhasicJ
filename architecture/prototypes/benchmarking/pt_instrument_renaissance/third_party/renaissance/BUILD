load(
    "@dwtj_rules_java//java:defs.bzl",
    "dwtj_legacy_java_import",
    "dwtj_java_binary",
)

dwtj_legacy_java_import(
    name = "renaissance",
    imports = ["@com_github_renaissance_benchmarks//jar"],
    visibility = ["//visibility:public"],
)

# This target exists to be run with `bazel run` for experimentation and
#  troubleshooting.
dwtj_java_binary(
    name = "renaissance_exec",
    main_class = "org.renaissance.core.Launcher",
    # TODO(dwtj): Remove this attribute and file once it is no longer necessary.
    srcs = ["DoNothing.java"],
    deps = [":renaissance"],
)
