load("@dwtj_rules_java//java:defs.bzl", "dwtj_java_library")
load("//bazel:util/files/extract_file_from_jar.bzl", "extract_file_from_jar")

dwtj_java_library(
    name = "test_jar",
    srcs = [":SynchBlock.java"],
    output_jar = "test.jar"
)

extract_file_from_jar(
    name = "SynchBlock_class",
    in_jar = "test.jar",
    file_to_extract = "phasicj/agent/instr/test/class_data/simple/SynchBlock.class",
    out_file = "SynchBlock.class",
    visibility = ["//phasicj/agent/instr:__subpackages__"],
)
