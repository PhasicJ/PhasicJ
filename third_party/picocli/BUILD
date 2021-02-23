load("@dwtj_rules_java//java:defs.bzl", "dwtj_legacy_java_import")

package(
    default_visibility = ["//visibility:public"],
)

_PICOCLI_VERSION = "4.6.1"

dwtj_legacy_java_import(
    name = "picocli",
    imports = ["@maven//:info_picocli_picocli"],
)

dwtj_legacy_java_import(
    name = "picocli_codegen",
    imports = ["@maven//:info_picocli_picocli_codegen"],
)

alias(
    name = "picocli_jar",
    actual = "@maven//:v1/https/jcenter.bintray.com/info/picocli/picocli/{0}/picocli-{0}.jar".format(_PICOCLI_VERSION),
)

alias(
    name = "picocli_codegen_jar",
    actual = "@maven//:v1/https/jcenter.bintray.com/info/picocli/picocli-codegen/{0}/picocli-codegen-{0}.jar".format(_PICOCLI_VERSION),
)
