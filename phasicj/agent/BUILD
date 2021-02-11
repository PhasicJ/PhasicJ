load("@rules_rust//rust:rust.bzl", "rust_library")
load("//bazel:util/files/copy_file.bzl", "copy_file")
load("//bazel:docs.bzl", "index_md")

package(
    default_visibility = ["//visibility:public"],
)

index_md()

rust_library(
    name = "agent",
    srcs = [
        "agent.rs",
        "lib.rs",
        "jvm.rs",
        "debug.rs",
        "//phasicj/agent/jvm",
    ],
    crate_type = "cdylib",
    edition = "2018",
    deps = [
        "//phasicj/agent/rt/embed:phasicj_agent_rt_jar_embed",
        "//phasicj/agent/svm/rust",
        "//phasicj/agent/svm/rust/embed",
        "//phasicj/agent/conf",
        "@graalvm_linux_x64//rust/jvmti",
    ],
)

# NOTE(dwtj): Because of the behavior of `@rules_rust`, the `agent` target
#  generates an output file which includes a hash value. These targets exists
#  to copy this file to a known (platform-specific) name.
copy_file(
    name = "libpjagent_so",
    src = ":agent",
    out = "libpjagent.so",
)

copy_file(
    name = "libjpagent_dylib",
    src = ":agent",
    out = "libpjagent.dylib",
)

alias(
    name = "libpjagent",
    actual = select({
        "@bazel_tools//src/conditions:linux_x86_64": "libpjagent.so",
        "@bazel_tools//src/conditions:darwin": "libpjagent.dylib",
    }),
)
