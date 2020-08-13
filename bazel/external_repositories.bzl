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
    "commit": "a99941fe5163938b18c2b6edc7c0c8696ebe4c15",
    "sha256": "8128ec00855279c92183eaf1a63c104a120c7a5da4f2ac029b19217b9133f788",
}

def dwtj_rules_java():
    http_archive(
        name = "dwtj_rules_java",
        url = "https://github.com/dwtj/dwtj_rules_java/archive/{}.zip".format(_DWTJ_RULES_JAVA_ARCHIVE_INFO["commit"]),
        strip_prefix = "dwtj_rules_java-{}".format(_DWTJ_RULES_JAVA_ARCHIVE_INFO["commit"]),
        sha256 = _DWTJ_RULES_JAVA_ARCHIVE_INFO["sha256"],
    )
