load(
    "@rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_test",
)
load("//bazel:util/files/hash_file.bzl", "sha1_hash_file")
load("//bazel:util/files/copy_file.bzl", "copy_file")

# NOTE(dwtj): We copy this file to abreviate its usage in this `BUILD` file and
#  to make the `include_bytes!` macro not need to look any further than this
#  package.
copy_file(
    name = "copy_rt_jar",
    src = "//phasicj/agent/rt:rt.jar",
    out = "rt.jar",
)

sha1_hash_file(
    name = "rt_jar_sha1",
    src = "rt.jar",
    out = "rt.jar.sha1",
)

rust_library(
    name = "phasicj_agent_rt_jar_embed",
    srcs = ["lib.rs"],
    compile_data = [
        ":rt.jar",
        ":rt.jar.sha1",
    ],
    edition = "2018",
    rustc_env = {
        "RT_JAR_EXEC_PATH": "$(execpath :rt.jar)",
        "RT_JAR_SHA1_EXEC_PATH": "$(execpath :rt.jar.sha1)",
    },
    visibility = ["//phasicj/agent:__subpackages__"],
)

rust_test(
    name = "test",
    srcs = ["test.rs"],
    edition = "2018",
    deps = ["phasicj_agent_rt_jar_embed"],
)
