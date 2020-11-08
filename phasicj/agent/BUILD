load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")
load("//bazel:util/rust/rename_cdylib.bzl", "rename_cdylib")

rust_library(
    name = "agent",
    srcs = [
        "agent.rs",
        "jni_env.rs",
        "jvmti_entrypoints.rs",
        "jvmti_env.rs",
        "jvmti_events.rs",
        "jvmti_setup.rs",
        "lib.rs",
    ],
    crate_type = "cdylib",
    edition = "2018",
    visibility = ["//visibility:public"],
    deps = [
        "//phasicj/agent/rt/embed:phasicj_agent_rt_jar_embed",
        "//phasicj/agent/svm",
        "//phasicj/agent/svm/embed",
        "//phasicj/agent/svm:svm_native_image_cc_import",
        "@graalvm_linux_x64//rust/jvmti",
    ],
)

# NOTE(dwtj): Because of the behavior of `@rules_rust`, the `agent` target
#  generates an output file which includes a hash value. This target exists to
#  copy/rename this file to a consistent name.
rename_cdylib(
    name = "libpjagent_so",
    src = ":agent",
    out = "libpjagent.so",
    visibility = ["//visibility:public"],
)
