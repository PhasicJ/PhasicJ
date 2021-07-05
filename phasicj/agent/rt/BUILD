load("@dwtj_rules_java//java:defs.bzl", "dwtj_java_library")
load("@rules_rust//rust:defs.bzl", "rust_library")
load("//bazel:docs.bzl", "index_md")

index_md()

dwtj_java_library(
    name = "rt",
    srcs = ["ApplicationEvents.java"],
    output_jar = "rt.jar",
    visibility = ["//phasicj/agent:__subpackages__"],
)

# NOTE(dwtj): This class is not meant to be "run". We compile it to create
#  snippets of bytecode to be used by the PhasicJ agent's runtime. Specifically,
#  this class definition used to specify some fields & methods to be added to
#  the `java.lang.Object` class of a user's Java application when monitored by
#  the PhasicJ agent.
dwtj_java_library(
    name = "java_lang_object_amendment",
    srcs = ["JavaLangObjectAmendment.java"],
    visibility = ["//phasicj/agent:__subpackages__"],
    resources = {
        # NOTE(dwtj): We include this GraalVM `native-image` configuration file
        #  in order to tell the `native-image` tool that the
        #  `JavaLangObjectAmendment.class` file should be treated as a resource
        #  file. This is similar to `native-image -H:IncludeResources=...`,
        #  except it is implicit. [This link][1] provides some background.
        #
        #  [1]: https://www.graalvm.org/reference-manual/native-image/Resources/
        "META-INF$native-image$phasicj$agent$rt$resource-config.json":
            "META-INF/native-image/phasicj/agent/rt/resource-config.json",
    },
    deps = [":rt"],
)
