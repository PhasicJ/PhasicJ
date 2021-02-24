load("@dwtj_rules_java//java:defs.bzl", "dwtj_java_library")
load("//bazel:util/files/extract_file_from_jar.bzl", "extract_file_from_jar")

dwtj_java_library(
    name = "svm_test_class_jar",
    srcs = ["SvmTestClass.java"],
    output_jar = "svm_test_class.jar",
)

extract_file_from_jar(
    name = "svm_test_class_unjar",
    in_jar = ":svm_test_class.jar",
    file_to_extract = "phasicj/agent/experimental/svm/test/SvmTestClass.class",
    out_file = "SvmTestClass.class",
    visibility = ["//phasicj/agent/experimental/svm:__subpackages__"],
)

# TODO(dwtj): Consider adding a test where `java.lang.Object` is instrumented.
#  This would be useful because instrumentation of this class is rather more
#  complicated than other classes. I spent a minute trying this, but I don't
#  know how to invoke `javac` such that I can compile a fake `java.lang.Object`
#  class. I could use ASM to rename a class.
