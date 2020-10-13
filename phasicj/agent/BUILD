load("@io_bazel_rules_rust//rust:rust.bzl", "rust_library")
load("@dwtj_rules_java//graalvm:defs.bzl", "graalvm_native_image_library")

rust_library(
    name = "agent",
    srcs = ["agent.rs"],
    edition = "2018",
    deps = ["@graalvm_linux_x64//rust/jvmti"],
)

# This target uses GraalVM's `native-image` tool to compile all of the agent's
# Java code into a native library and to wrap it in a `cc_library` target.
graalvm_native_image_library(
    name = "agent_graal_native_image",
    deps = ["//phasicj/agent/instr"],
    native_image_options = [
        "--no-fallback",
        "--allow-incomplete-classpath",
    ],
    image_name = "phasicj_agent_instr",
    library_output = "libinstr.so",
    header_output = "instr.h",
    dynamic_header_output = "instr_dynamic.h",
    graal_isolate_header_output = "graal_isolate.h",
    graal_isolate_dynamic_header_output = "graal_isolate_dynamic.h",
)
