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
    "commit": "5d57355344dd4763f82f500011e2b4925f7ea8f1",
    "sha256": "a819b0c567b72f4f5960391c3a8111592c91dd94ae86b45d2c7fb7b9243143b2",
}

def dwtj_rules_java():
    http_archive(
        name = "dwtj_rules_java",
        url = "https://github.com/dwtj/dwtj_rules_java/archive/{}.zip".format(_DWTJ_RULES_JAVA_ARCHIVE_INFO["commit"]),
        strip_prefix = "dwtj_rules_java-{}".format(_DWTJ_RULES_JAVA_ARCHIVE_INFO["commit"]),
        sha256 = _DWTJ_RULES_JAVA_ARCHIVE_INFO["sha256"],
    )
