"""Defines some helper macros to be called in a `WORKSPACE` file to add some
external repositories.
"""

load(
    "@bazel_tools//tools/build_defs/repo:http.bzl",
    "http_archive",
    "http_jar",
)

# NOTE(dwtj): This version is used because it is the latest release as of
#  2021-02-03.
_RULES_JVM_EXTERNAL_TAG = "4.0"
_RULES_JVM_EXTERNAL_SHA256 = "31701ad93dbfe544d597dbe62c9a1fdd76d81d8a9150c2bf1ecf928ecdf97169"

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
    "commit": "9d979544ab2a242ff777caee7cd83fe2fe6844d3",
    "sha256": "06cc8702a42f3ab8bfd0de462d07e2590ecd1a05a3f0841c3b9fa49809a6ff69",
}

def dwtj_rules_java():
    http_archive(
        name = "dwtj_rules_java",
        url = "https://github.com/dwtj/dwtj_rules_java/archive/{}.tar.gz".format(_DWTJ_RULES_JAVA_ARCHIVE_INFO["commit"]),
        strip_prefix = "dwtj_rules_java-{}".format(_DWTJ_RULES_JAVA_ARCHIVE_INFO["commit"]),
        sha256 = _DWTJ_RULES_JAVA_ARCHIVE_INFO["sha256"],
    )

_REMOTE_GRAALVM_VERSION = "21.1.0"
_REMOTE_GRAALVM_LINUX_ARCHIVE_SHA256 = "39252954d2cb16dbc8ce4269f8b93a326a0efffdce04625615e827fe5b5e4ab7"
_REMOTE_GRAALVM_LINUX_NATIVE_IMAGE_INSTALLABLE_JAR_SHA256 = "1ab725616fede21ad5ae900d00ec6d45753db6f6f4fe51a836d538c79d79614a"

def apply_remote_graalvm_linux_repository(name, remote_graalvm_repository_rule):
    remote_graalvm_repository_rule(
        name = name,
        url = "https://github.com/graalvm/graalvm-ce-builds/releases/download/vm-{0}/graalvm-ce-java11-linux-amd64-{0}.tar.gz".format(_REMOTE_GRAALVM_VERSION),
        sha256 = _REMOTE_GRAALVM_LINUX_ARCHIVE_SHA256,
        strip_prefix = "graalvm-ce-java11-{}".format(_REMOTE_GRAALVM_VERSION),
        native_image_installable_jar_url = "https://github.com/graalvm/graalvm-ce-builds/releases/download/vm-{0}/native-image-installable-svm-java11-linux-amd64-{0}.jar".format(_REMOTE_GRAALVM_VERSION),
        native_image_installable_jar_sha256 = _REMOTE_GRAALVM_LINUX_NATIVE_IMAGE_INSTALLABLE_JAR_SHA256,
        os = "linux",
        cpu = "x64",
    )

_REMOTE_GRAALVM_DARWIN_X64_ARCHIVE_SHA256 = "b53cd5a085fea39cb27fc0e3974f00140c8bb774fb2854d72db99e1be405ae2b"
_REMOTE_GRAALVM_DARWIN_X64_NATIVE_IMAGE_INSTALLABLE_JAR_SHA256 = "71790754752e5ae744912c0def358c9e2176f2ad3fd371dc6b401705dfef0137"

def apply_remote_graalvm_macos_repository(name, remote_graalvm_repository_rule):
    remote_graalvm_repository_rule(
        name = name,
        url = "https://github.com/graalvm/graalvm-ce-builds/releases/download/vm-{0}/graalvm-ce-java11-darwin-amd64-{0}.tar.gz".format(_REMOTE_GRAALVM_VERSION),
        sha256 = _REMOTE_GRAALVM_DARWIN_X64_ARCHIVE_SHA256,
        strip_prefix = "graalvm-ce-java11-{}/Contents/Home".format(_REMOTE_GRAALVM_VERSION),
        native_image_installable_jar_url = "https://github.com/graalvm/graalvm-ce-builds/releases/download/vm-{0}/native-image-installable-svm-java11-darwin-amd64-{0}.jar".format(_REMOTE_GRAALVM_VERSION),
        native_image_installable_jar_sha256 = _REMOTE_GRAALVM_DARWIN_X64_NATIVE_IMAGE_INSTALLABLE_JAR_SHA256,
        os = "macos",
        cpu = "x64",
    )

_RULES_JAVA_RELEASE = "0.1.1"
_RULES_JAVA_SHA256 = "220b87d8cfabd22d1c6d8e3cdb4249abd4c93dcc152e0667db061fb1b957ee68"

def rules_java():
    http_archive(
        name = "rules_java",
        url = "https://github.com/bazelbuild/rules_java/releases/download/{0}/rules_java-{0}.tar.gz".format(_RULES_JAVA_RELEASE),
        sha256 = _RULES_JAVA_SHA256,
    )

_RULES_PROTO_COMMIT = "c0b62f2f46c85c16cb3b5e9e921f0d00e3101934"
_RULES_PROTO_SHA256 = "e0cab008a9cdc2400a1d6572167bf9c5afc72e19ee2b862d18581051efab42c9"

def rules_proto():
    http_archive(
        name = "rules_proto",
        sha256 = _RULES_PROTO_SHA256,
        strip_prefix = "rules_proto-{}".format(_RULES_PROTO_COMMIT),
        url = "https://github.com/bazelbuild/rules_proto/archive/{}.tar.gz".format(_RULES_PROTO_COMMIT),
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

# This is the latest commit on the master branch of the Renaissance benchmarks
# as of 2021-06-24.
_RENAISSANCE_BENCHMARKS_COMMIT = "4cccf729d40d03928fe2c21e518ea89a2c403349"
_RENAISSANCE_BENCHMARKS_SHA256 = "33a25951c0bac9939ad8ea902084703f07e227aa32900423e61dbf7c845a5eb8"

def com_github_renaissance_benchmarks():
    http_archive(
        name = "com_github_renaissance_benchmarks",
        url = "https://github.com/renaissance-benchmarks/renaissance/archive/{}.tar.gz".format(_RENAISSANCE_BENCHMARKS_COMMIT),
        strip_prefix = "renaissance-{}".format(_RENAISSANCE_BENCHMARKS_COMMIT),
        sha256 = _RENAISSANCE_BENCHMARKS_SHA256,
        build_file = "//bazel:external/BUILD.renaissance",
    )

def com_github_dwtj_renaissance_jar():
    http_jar(
        name = "com_github_dwtj_renaissance_jar",
        url = "https://github.com/dwtj/renaissance/releases/download/v0.12.0-snapshot/renaissance.jar",
        sha256 = "2ea4f40c127710a8c78804e9b1115f735ea8a8ea00a3556a85a66b0682d076eb",
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

_ORG_OW2_ASM_COMMONS_JAR_VERSION = "9.0"
_ORG_OW2_ASM_COMMONS_JAR_SHA256 = "1b9090acb7e67bd4ed2f2cfb002063316d79cecace237bd07cc4f7f1b302092f"

def org_ow2_asm_commons():
    http_jar(
        name = "org_ow2_asm_commons",
        urls = [
            "https://repository.ow2.org/nexus/content/repositories/releases/org/ow2/asm/asm-commons/{0}/asm-commons-{0}.jar".format(_ORG_OW2_ASM_COMMONS_JAR_VERSION),
            "https://repo1.maven.org/maven2/org/ow2/asm/asm-commons/{0}/asm-commons-{0}.jar".format(_ORG_OW2_ASM_COMMONS_JAR_VERSION),
        ],
        sha256 = _ORG_OW2_ASM_COMMONS_JAR_SHA256,
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

# Latest commit to master branch as of 2021-07-01.
_RULES_RUST_COMMIT = "4bf51b3c790c4c3d0ee8c28c15b7307b64a7069a"
_RULES_RUST_SHA256 = "fb1954ddf1fcd2e214441fa282b020bf15f412bf6d3b31dfe0c14fcc788dcb1f"

def rules_rust():
    http_archive(
        name = "rules_rust",
        url = "https://github.com/bazelbuild/rules_rust/archive/{}.tar.gz".format(_RULES_RUST_COMMIT),
        sha256 = _RULES_RUST_SHA256,
        strip_prefix = "rules_rust-{}".format(_RULES_RUST_COMMIT),
    )

_PLATFORMS_RELEASE = "0.0.4"
_PLATFORMS_SHA256 = "079945598e4b6cc075846f7fd6a9d0857c33a7afc0de868c2ccb96405225135d"

def platforms():
    http_archive(
        name = "platforms",
        urls = [
            "https://mirror.bazel.build/github.com/bazelbuild/platforms/releases/download/{0}/platforms-{0}.tar.gz".format(_PLATFORMS_RELEASE),
            "https://github.com/bazelbuild/platforms/releases/download/{0}/platforms-{0}.tar.gz".format(_PLATFORMS_RELEASE),
        ],
        sha256 = _PLATFORMS_SHA256,
    )
