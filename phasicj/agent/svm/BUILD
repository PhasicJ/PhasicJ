load("@dwtj_rules_java//experimental/graalvm:defs.bzl", "graalvm_native_image_library")
load("//bazel:util/files/copy_file.bzl", "copy_file")

# NOTE(dwtj): I am currently assuming that one cannot (or at least should not)
#  link multiple `native-image` outputs together into one library. So, this
#  `native-image` invocation should include all Java dependencies that need to
#  be included in the agent as as native code. Currently, it just includes one
#  dependency, `instr`.
graalvm_native_image_library(
    name = "svm_native_image",
    dynamic_header_output = "svm_dynamic.h",
    graal_isolate_dynamic_header_output = "graal_isolate_dynamic.h",
    # NOTE(dwtj): The non-dynamic headers aren't currently being used.
    graal_isolate_header_output = "_graal_isolate.h",
    header_output = "_svm.h",
    image_name = "phasicj_agent_svm",
    library_output = "libsvm.so",
    native_image_options = [
        "--no-fallback",
        # TODO(dwtj): Consider moving this into a `resource-config.json` file, possibly in the JAR
        #  which holds this class.
        "-H:IncludeResources=phasicj/agent/rt/JavaLangObjectAmendment.class",
    ],
    visibility = ["//phasicj/agent:__subpackages__"],
    deps = [
        "//phasicj/agent/instr/svm",
        "//phasicj/agent/rt:java_lang_object_amendment",
        "//third_party/asm",
    ],
)

copy_file(
    name = "copy_so_to_dylib",
    src = "libsvm.so",
    out = "libsvm.dylib",
)

alias(
    name = "svm",
    actual = select({
        "@bazel_tools//src/conditions:linux_x86_64": "libsvm.so",
        "@bazel_tools//src/conditions:darwin": "libsvm.dylib",
    }),
    visibility = ["//phasicj/agent:__subpackages__"],
)
