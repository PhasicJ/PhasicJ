load("@dwtj_rules_java//java:defs.bzl", "dwtj_legacy_java_import")

package(
    default_visibility = ["//visibility:public"],
)

dwtj_legacy_java_import(
    name = "renaissance",
    imports = ["//third_party/renaissance/sbt:legacy_java_import"],
)

alias(
    name = "jar",
    actual = "//third_party/renaissance/sbt:jar"
)
