workspace(name = "phasicj")

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
load(
    "//bazel:external/repositories.bzl",
    "apply_remote_graalvm_linux_repository",
    "apply_remote_graalvm_macos_repository",
    "com_github_renaissance_benchmarks",
    "dwtj_rules_java",
    "rules_rust",
    "org_ow2_asm",
    "org_ow2_asm_commons",
    "rules_cc",
    "rules_jvm_external",
)

# CONFIGURE `@rules_cc` #######################################################

rules_cc()

# CONFIGURE `buildifier` FOR BAZEL FILE LINTING AND FORMATTING ################
#
# This configuration was adapted from [the buildifier README.md](https://github.com/bazelbuild/buildtools/tree/8de190500912105ad59bdb5e65458dcc9df23256/buildifier)

# `buildifier` is written in Go and hence needs rules_go to be built.
# See https://github.com/bazelbuild/rules_go for the up to date setup instructions.

# NOTE(dwtj): This version was choosen because it was the most recent release as
#  of 2020-06-14.
http_archive(
    name = "io_bazel_rules_go",
    sha256 = "a8d6b1b354d371a646d2f7927319974e0f9e52f73a2452d2b3877118169eb6bb",
    urls = [
        "https://mirror.bazel.build/github.com/bazelbuild/rules_go/releases/download/v0.23.3/rules_go-v0.23.3.tar.gz",
        "https://github.com/bazelbuild/rules_go/releases/download/v0.23.3/rules_go-v0.23.3.tar.gz",
    ],
)

load("@io_bazel_rules_go//go:deps.bzl", "go_register_toolchains", "go_rules_dependencies")

go_rules_dependencies()

go_register_toolchains()

# NOTE(dwtj): This version was choosen because it was the most recent release as
#  of 2020-06-14.
http_archive(
    name = "bazel_gazelle",
    sha256 = "cdb02a887a7187ea4d5a27452311a75ed8637379a1287d8eeb952138ea485f7d",
    url = "https://github.com/bazelbuild/bazel-gazelle/releases/download/v0.21.1/bazel-gazelle-v0.21.1.tar.gz",
)

load("@bazel_gazelle//:deps.bzl", "gazelle_dependencies")

gazelle_dependencies()

# NOTE(dwtj): This version was choosen because it was the most recent release as
#  of 2020-06-14.
http_archive(
    name = "com_google_protobuf",
    sha256 = "e5265d552e12c1f39c72842fa91d84941726026fa056d914ea6a25cd58d7bbf8",
    strip_prefix = "protobuf-3.12.3",
    url = "https://github.com/protocolbuffers/protobuf/archive/v3.12.3.zip",
)

load("@com_google_protobuf//:protobuf_deps.bzl", "protobuf_deps")

protobuf_deps()

# NOTE(dwtj): This version was choosen because it was the most recent release as
#  of 2020-06-14.
http_archive(
    name = "com_github_bazelbuild_buildtools",
    sha256 = "55095ba38ab866166052d7e99a5ff237797cf86de4481f07d7f43540e79641df",
    strip_prefix = "buildtools-3.2.0",
    url = "https://github.com/bazelbuild/buildtools/archive/3.2.0.zip",
)

# CONFIGURE `@dwtj_rules_markdown` ############################################
#
# This dependency is used to add `markdownlint` actions during `bazel build`.

# This version was chosen because it was the latest as of 2020-07-06.
DWTJ_RULES_MARKDOWN_COMMIT = "f65f338a9e7ed15fe7b2630c7e1cbf47e81a2837"

DWTJ_RULES_MARKDOWN_SHA256 = "ff905bbb200aa6a5195e435c433e9b463aa60a4765e93315a37e3b9bcbc9bd04"

http_archive(
    name = "dwtj_rules_markdown",
    sha256 = DWTJ_RULES_MARKDOWN_SHA256,
    strip_prefix = "dwtj_rules_markdown-{}".format(DWTJ_RULES_MARKDOWN_COMMIT),
    url = "https://github.com/dwtj/dwtj_rules_markdown/archive/{}.tar.gz".format(DWTJ_RULES_MARKDOWN_COMMIT),
)

load("@dwtj_rules_markdown//markdown:repositories.bzl", "local_markdownlint_repository")

local_markdownlint_repository(
    name = "local_markdownlint",
    config = "//:.markdownlint.json",
)

load("@local_markdownlint//:defs.bzl", "register_local_markdownlint_toolchain")

register_local_markdownlint_toolchain()

# CONFIGURE `@dwtj_rules_java` ################################################

dwtj_rules_java()

load(
    "@dwtj_rules_java//java:repositories.bzl",
    "known_openjdk_repository",
    "maven_error_prone_repository",
    "remote_google_java_format_repository",
)
load(
    "@dwtj_rules_java//graalvm:repositories.bzl",
    "remote_graalvm_repository",
)

# CONFIGURE `@graalvm_linux_x64` ##############################################

apply_remote_graalvm_linux_repository(
    "graalvm_linux_x64",
    remote_graalvm_repository,
)

# NOTE(dwtj): We use GraalVM as our Java compiler toolchain.
register_toolchains("@graalvm_linux_x64//java:java_compiler_toolchain")

load("@graalvm_linux_x64//graalvm:defs.bzl", register_graalvm_linux_toolchains = "register_graalvm_toolchains")

register_graalvm_linux_toolchains()

# CONFIGURE `@graalvm_macos_x64` ##############################################

apply_remote_graalvm_macos_repository(
    "graalvm_macos_x64",
    remote_graalvm_repository,
)

# NOTE(dwtj): We use GraalVM as our Java compiler toolchain.
register_toolchains("@graalvm_macos_x64//java:java_compiler_toolchain")

load("@graalvm_macos_x64//graalvm:defs.bzl", register_graalvm_macos_toolchains = "register_graalvm_toolchains")

register_graalvm_macos_toolchains()

# CONFIGURE `@openjdk_linux_x64` ##############################################

known_openjdk_repository("openjdk_linux_x64")

register_toolchains("@openjdk_linux_x64//java:java_runtime_toolchain")

# CONFIGURE `@openjdk_macos_x64` ##############################################

known_openjdk_repository(
    "openjdk_macos_x64",
    os = "macos",
)

register_toolchains("@openjdk_macos_x64//java:java_runtime_toolchain")

# CONFIGURE `@openjdk_11_linux_x64` ###########################################

known_openjdk_repository(
    name = "openjdk_11_linux_x64",
    provider = "adoptopenjdk",
    version = "11.0.9+11.1",
)

# CONFIGURE `@openjdk_11_macos_x64` ###########################################

known_openjdk_repository(
    name = "openjdk_11_macos_x64",
    provider = "adoptopenjdk",
    version = "11.0.9+11.1",
    os = "macos",
)

# CONFIGURE `@google_java_format` #############################################

remote_google_java_format_repository(
    name = "google_java_format",
)

load("@google_java_format//:defs.bzl", "register_google_java_format_toolchain")

register_google_java_format_toolchain()

# CONFIGURE `@com_github_renaissance_benchmarks` ##############################

com_github_renaissance_benchmarks()

# CONFIGURE `@org_ow2_asm` ################################################

org_ow2_asm()

# CONFIGURE `@org_ow2_asm_commons` ########################################

org_ow2_asm_commons()

# CONFIGURE `@rules_jvm_external` #############################################

rules_jvm_external()

load("@rules_jvm_external//:defs.bzl", "maven_install")

maven_install(
    name = "maven",
    artifacts = [
        "com.google.guava:guava:30.1-jre",
        "info.picocli:picocli:4.6.1",
    ],
    repositories = [
        "https://jcenter.bintray.com/",
        "https://maven.google.com",
        "https://repo1.maven.org/maven2",
    ],
)

# CONFIGURE `@error_prone` ####################################################

maven_error_prone_repository(
    name = "error_prone",
)

load(
    "@error_prone//:defs.bzl",
    "fetch_error_prone_toolchain",
    "register_error_prone_toolchain",
)

fetch_error_prone_toolchain()

register_error_prone_toolchain()

# CONFIGURE `@rules_rust` #####################################################

rules_rust()

load("@rules_rust//rust:repositories.bzl", "rust_repositories")

rust_repositories()

# CONFIGURE cargo-raze ########################################################

load("//third_party/cargo:crates.bzl", "raze_fetch_remote_crates")

raze_fetch_remote_crates()
