load("@dwtj_rules_java//java:defs.bzl", "dwtj_java_library")
load(
    "@io_bazel_rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_test",
)
load("@io_bazel_rules_rust//cargo:cargo_build_script.bzl", "cargo_build_script")
load("//bazel:util/files/hash_file.bzl", "sha1_hash_file")

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
    deps = [":rt"],
)

cargo_build_script(
    name = "do_nothing_cargo_build_script",
    srcs = ["do_nothing_cargo_build_script.rs"],
)

sha1_hash_file(
    name = "rt_jar_sha1",
    src = "rt.jar",
    out = "rt.jar.sha1",
)

rust_library(
    name = "phasicj_agent_rt_jar_embed",
    srcs = ["jar_embed.rs"],
    data = [
        "rt.jar",
        "rt.jar.sha1",
    ],
    edition = "2018",
    visibility = ["//phasicj/agent:__subpackages__"],
    deps = [":do_nothing_cargo_build_script"],
)

rust_test(
    name = "jar_embed_test",
    srcs = ["jar_embed_test.rs"],
    edition = "2018",
    deps = ["phasicj_agent_rt_jar_embed"],
)
