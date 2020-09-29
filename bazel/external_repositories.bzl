'''Defines some helper macros to be called in a `WORKSPACE` file to add some
external repositories.
'''

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
    '''Fetches and creates the `@rules_jvm_external` external repository.
    '''
    http_archive(
        name = "rules_jvm_external",
        strip_prefix = "rules_jvm_external-{}".format(_RULES_JVM_EXTERNAL_TAG),
        sha256 = _RULES_JVM_EXTERNAL_SHA256,
        url = "https://github.com/bazelbuild/rules_jvm_external/archive/{}.zip".format(_RULES_JVM_EXTERNAL_TAG),
    )

_DWTJ_RULES_JAVA_ARCHIVE_INFO = {
    "commit": "b469c5bbc0fa158b13e5e7eb80c3cda8a7e48398",
    "sha256": "7a8f596e7800d28ecf39e4450d063a2d72a8717deb0f29d9a00f3e3fdd3a701f",
}

def dwtj_rules_java():
    http_archive(
        name = "dwtj_rules_java",
        url = "https://github.com/dwtj/dwtj_rules_java/archive/{}.zip".format(_DWTJ_RULES_JAVA_ARCHIVE_INFO["commit"]),
        strip_prefix = "dwtj_rules_java-{}".format(_DWTJ_RULES_JAVA_ARCHIVE_INFO["commit"]),
        sha256 = _DWTJ_RULES_JAVA_ARCHIVE_INFO["sha256"],
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
        sha256 = _RENAISSANCE_BENCHMARKS_JAR_SHA256
    )

_ORG_OW2_ASM_JAR_VERSION = "8.0.1"
_ORG_OW2_ASM_JAR_SHA256 = "ca5b8d11569e53921b0e3486469e7c674361c79845dad3d514f38ab6e0c8c10a"
def org_ow2_asm():
    http_jar(
        name = "org_ow2_asm",
        url = "https://repository.ow2.org/nexus/content/repositories/releases/org/ow2/asm/asm/{0}/asm-{0}.jar".format(_ORG_OW2_ASM_JAR_VERSION),
        sha256 = _ORG_OW2_ASM_JAR_SHA256,
    )
