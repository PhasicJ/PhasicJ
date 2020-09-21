'''Defines some helper macros to be called in a `WORKSPACE` file to add some
external repositories.
'''

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")

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
    "commit": "6fda17081894b1044c2659c2c58a14489f06641e",
    "sha256": "ba1a927b03176543bb71933871815f443464254f1195b26f2c1e92262226ec9b",
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
        #strip_prefix = "rules_java-{}".format(_RULES_JAVA_RELEASE),
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
