load("@dwtj_rules_java//graalvm:defs.bzl", "graalvm_native_image_binary")
load("@dwtj_rules_embed//embed:defs.bzl", "embedded_file_cc_library")

graalvm_native_image_binary(
    name = "analysis_native_image",
    main_class = "pt.svm.Analysis",
    deps = ["//pt/svm:analysis"],
    output = "analysis_executable"
)

embedded_file_cc_library(
    name = "analysis",
    file = ":analysis_executable",
    cc_namespace = "pt::svm::exec::embedded",
    native_symbol_name = "pt_svm_exec_embedded",
    visibility = ["//visibility:public"],
)
