load("@dwtj_rules_java//java:defs.bzl", "dwtj_java_library")

dwtj_java_library(
    name = "rt",
    srcs = ["ApplicationEvents.java"],
    output_jar = "rt.jar",
    visibility = ["//phasicj/agent:__subpackages__"],
)

dwtj_java_library(
    name = "java_lang_object_amendment",
    srcs = ["JavaLangObjectAmendment.java"],
    visibility = ["//phasicj/agent:__subpackages__"],
    deps = [":rt"],
)
