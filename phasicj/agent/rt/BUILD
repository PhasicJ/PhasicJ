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
    # TODO(dwtj): Consider making this a compile-time only dependency once that
    #  is supported in `@dwtj_rules_java`. The amendment itself isn't meant to
    #  be "run", after all.
    deps = [":rt"],
)
