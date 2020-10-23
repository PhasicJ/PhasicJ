"""Defines some helper macros to be called in a `WORKSPACE` file to add some
external repositories.
"""

load(
    "@bazel_tools//tools/build_defs/repo:http.bzl",
    "http_archive",
    "http_jar",
)

# NOTE(dwtj): This version was chosen because it was the most recent release as
#  of 2020-07-09, when this code was drafted.
_RULES_JVM_EXTERNAL_TAG = "3.3"
_RULES_JVM_EXTERNAL_SHA256 = "d85951a92c0908c80bd8551002d66cb23c3434409c814179c0ff026b53544dab"

def rules_jvm_external():
    """Fetches and creates the `@rules_jvm_external` external repository.
    """
    http_archive(
        name = "rules_jvm_external",
        strip_prefix = "rules_jvm_external-{}".format(_RULES_JVM_EXTERNAL_TAG),
        sha256 = _RULES_JVM_EXTERNAL_SHA256,
        url = "https://github.com/bazelbuild/rules_jvm_external/archive/{}.zip".format(_RULES_JVM_EXTERNAL_TAG),
    )

_DWTJ_RULES_JAVA_ARCHIVE_INFO = {
    "commit": "b810cbd9cf685e4d3af993ba2bf2c5726ea692d0",
    "sha256": "1914f60aa2fa93322066d39d2d3827606dc606277c643f2f7ef298795ff03384",
}

def dwtj_rules_java():
    http_archive(
        name = "dwtj_rules_java",
        url = "https://github.com/dwtj/dwtj_rules_java/archive/{}.tar.gz".format(_DWTJ_RULES_JAVA_ARCHIVE_INFO["commit"]),
        strip_prefix = "dwtj_rules_java-{}".format(_DWTJ_RULES_JAVA_ARCHIVE_INFO["commit"]),
        sha256 = _DWTJ_RULES_JAVA_ARCHIVE_INFO["sha256"],
    )

# TODO(dwtj): Document this.
# TODO(dwtj): Generalize this.
def apply_dwtj_remote_openjdk_repository(name, dwtj_remote_openjdk_repository_rule):
    dwtj_remote_openjdk_repository_rule(
        name = name,
        url = "https://download.java.net/java/GA/jdk15/779bf45e88a44cbd9ea6621d33e33db1/36/GPL/openjdk-15_linux-x64_bin.tar.gz",
        sha256 = "bb67cadee687d7b486583d03c9850342afea4593be4f436044d785fba9508fb7",
        strip_prefix = "jdk-15",
        os = "linux",
    )

_REMOTE_GRAALVM_VERSION = "20.2.0"
_REMOTE_GRAALVM_ARCHIVE_SHA256 = "5db74b5b8888712d2ac3cd7ae2a8361c2aa801bc94c801f5839351aba5064e29"
_REMOTE_GRAALVM_NATIVE_IMAGE_INSTALLABLE_JAR_SHA256 = "92b429939f12434575e4d586f79c5b686d322f29211d1608ed6055a97a35925c"

def apply_remote_graalvm_repository(name, remote_graalvm_repository_rule):
    remote_graalvm_repository_rule(
        name = name,
        url = "https://github.com/graalvm/graalvm-ce-builds/releases/download/vm-{0}/graalvm-ce-java11-linux-amd64-{0}.tar.gz".format(_REMOTE_GRAALVM_VERSION),
        sha256 = _REMOTE_GRAALVM_ARCHIVE_SHA256,
        strip_prefix = "graalvm-ce-java11-{}".format(_REMOTE_GRAALVM_VERSION),
        native_image_installable_jar_url = "https://github.com/graalvm/graalvm-ce-builds/releases/download/vm-{0}/native-image-installable-svm-java11-linux-amd64-{0}.jar".format(_REMOTE_GRAALVM_VERSION),
        native_image_installable_jar_sha256 = _REMOTE_GRAALVM_NATIVE_IMAGE_INSTALLABLE_JAR_SHA256,
        os = "linux",
    )

_RULES_JAVA_RELEASE = "0.1.1"
_RULES_JAVA_SHA256 = "220b87d8cfabd22d1c6d8e3cdb4249abd4c93dcc152e0667db061fb1b957ee68"

def rules_java():
    http_archive(
        name = "rules_java",
        url = "https://github.com/bazelbuild/rules_java/releases/download/{0}/rules_java-{0}.tar.gz".format(_RULES_JAVA_RELEASE),
        sha256 = _RULES_JAVA_SHA256,
    )

# NOTE(dwtj): This version was chosen because it was the most recent commit to
#  [master] when this code was drafted, 2020-09-19.
#_RULES_PROTO_COMMIT = "40298556293ae502c66579620a7ce867d5f57311"
#_RULES_PROTO_SHA256 = "aa1ee19226f707d44bee44c720915199c20c84a23318bb0597ed4e5c873ccbd5"
#
#def rules_proto():
#    http_archive(
#        name = "rules_proto",
#        sha256 = _RULES_PROTO_SHA256,
#        strip_prefix = "rules_proto-{}".format(_RULES_PROTO_COMMIT),
#        url = "https://github.com/bazelbuild/rules_proto/archive/{}.tar.gz".format(_RULES_PROTO_COMMIT),
#    )

# NOTE(dwtj): We currently use a forked version of `@rules_proto` by Yannic was
#  because it fixes a [bug][1] seen in the latest [master] of `@rules_proto` (as
#  of 2020-09-19). Yannic's fix appears to just bump the Protobuf version to
#  v3.13.0.
#
#  [1]: https://github.com/bazelbuild/rules_proto/issues/67

# TODO(dwtj): Use the official `@rules_proto` release once this issue is
#  resolved.

_YANNIC_RULES_PROTO_COMMIT = "6103a187ba73feab10b5c44b52fa093675807d34"
_YANNIC_RULES_PROTO_SHA256 = "397d82596ae66101626d0eed39a0f09a01b07e6b1de362fe05c900c2eae6848a"

def rules_proto():
    http_archive(
        name = "rules_proto",
        sha256 = _YANNIC_RULES_PROTO_SHA256,
        strip_prefix = "rules_proto_bazelbuild-{}".format(_YANNIC_RULES_PROTO_COMMIT),
        url = "https://github.com/Yannic/rules_proto_bazelbuild/archive/{}.tar.gz".format(_YANNIC_RULES_PROTO_COMMIT),
    )

_RULES_CC_COMMIT = "02becfef8bc97bda4f9bb64e153f1b0671aec4ba"
_RULES_CC_SHA256 = "00cb11a249c93bd59f8b2ae61fba9a34fa26a673a4839fb2f3a57c185a00524a"

def rules_cc():
    http_archive(
        name = "rules_cc",
        urls = ["https://github.com/bazelbuild/rules_cc/archive/{}.tar.gz".format(_RULES_CC_COMMIT)],
        strip_prefix = "rules_cc-{}".format(_RULES_CC_COMMIT),
        sha256 = _RULES_CC_SHA256,
    )

_RENAISSANCE_BENCHMARKS_JAR_VERSION = "0.11.0"
_RENAISSANCE_BENCHMARKS_JAR_SHA256 = "6b6038cc0dfab4f44fa97f4918f75332ad91ae332db7867b18baf17ed55d2ce4"

def com_github_renaissance_benchmarks():
    http_jar(
        name = "com_github_renaissance_benchmarks",
        url = "https://github.com/renaissance-benchmarks/renaissance/releases/download/v{0}/renaissance-gpl-{0}.jar".format(_RENAISSANCE_BENCHMARKS_JAR_VERSION),
        sha256 = _RENAISSANCE_BENCHMARKS_JAR_SHA256,
    )

_ORG_OW2_ASM_JAR_VERSION = "9.0"
_ORG_OW2_ASM_JAR_SHA256 = "0df97574914aee92fd349d0cb4e00f3345d45b2c239e0bb50f0a90ead47888e0"

def org_ow2_asm():
    http_jar(
        name = "org_ow2_asm",
        urls = [
            "https://repository.ow2.org/nexus/content/repositories/releases/org/ow2/asm/asm/{0}/asm-{0}.jar".format(_ORG_OW2_ASM_JAR_VERSION),
            "https://repo1.maven.org/maven2/org/ow2/asm/asm/{0}/asm-{0}.jar".format(_ORG_OW2_ASM_JAR_VERSION),
        ],
        sha256 = _ORG_OW2_ASM_JAR_SHA256,
    )

_DWTJ_RULES_EMBED_SHA256 = "40069df3088f7df175d6fba7f53a1475aa284a32c971443c81966bb743da77a6"
_DWTJ_RULES_EMBED_COMMIT = "182466efc926c103a40e3acb3e8aa031c8a92c2f"

def dwtj_rules_embed(name = "dwtj_rules_embed"):
    http_archive(
        name = name,
        url = "https://github.com/dwtj/dwtj_rules_embed/archive/{}.tar.gz".format(_DWTJ_RULES_EMBED_COMMIT),
        sha256 = _DWTJ_RULES_EMBED_SHA256,
        strip_prefix = "dwtj_rules_embed-{}".format(_DWTJ_RULES_EMBED_COMMIT),
    )

_BAZEL_SKYLIB_RELEASE_VERSION = "1.0.3"
_BAZEL_SKYLIB_RELEASE_SHA256 = "1c531376ac7e5a180e0237938a2536de0c54d93f5c278634818e0efc952dd56c"

def bazel_skylib():
    http_archive(
        name = "bazel_skylib",
        urls = [
            "https://github.com/bazelbuild/bazel-skylib/releases/download/{0}/bazel-skylib-{0}.tar.gz".format(_BAZEL_SKYLIB_RELEASE_VERSION),
            "https://mirror.bazel.build/github.com/bazelbuild/bazel-skylib/releases/download/{0}/bazel-skylib-{0}.tar.gz".format(_BAZEL_SKYLIB_RELEASE_VERSION),
        ],
        sha256 = _BAZEL_SKYLIB_RELEASE_SHA256,
    )

_RULES_RUST_COMMIT = "726b1821508655432873619edd5abe10a6d73055"
_RULES_RUST_SHA256 = "0ff801f659d3ae4fc279dbfc00412a1468e0e6d4b7ab0b614a1c9cfb9f5b2c8b"

def io_bazel_rules_rust():
    http_archive(
        name = "io_bazel_rules_rust",
        url = "https://github.com/bazelbuild/rules_rust/archive/{}.tar.gz".format(_RULES_RUST_COMMIT),
        sha256 = _RULES_RUST_SHA256,
        strip_prefix = "rules_rust-{}".format(_RULES_RUST_COMMIT),
    )
