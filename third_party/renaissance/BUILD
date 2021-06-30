load("@dwtj_rules_java//java:defs.bzl", "dwtj_legacy_java_import")

package(
    default_visibility = ["//visibility:public"],
)

dwtj_legacy_java_import(
    name = "renaissance",
    imports = ["jar"],
)

alias(
    name = "jar",
    actual = "@com_github_dwtj_renaissance_jar//jar"
)
