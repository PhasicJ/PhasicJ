# TODO(dwtj): Try again to get Renaissance to compile from source.
#
#load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_archive")
#
#_RENAISSANCE_SOURCE_TAG = "v0.11.0"
#_RENAISSANCE_SOURCE_ARCHIVE_PREFIX = "renaissance-0.11.0"
#_RENAISSANCE_SOURCE_ARCHIVE_SHA256 = "93ce4c458a8c365a24a0de997b021a0c69409d3a7be4f831cca378c45bcf3132"
#
#def com_github_renaissance_benchmarks():
#    http_archive(
#        name = "com_github_renaissance_benchmarks",
#        url = "https://github.com/renaissance-benchmarks/renaissance/archive/{}.tar.gz".format(_RENAISSANCE_SOURCE_TAG),
#        sha256 = _RENAISSANCE_SOURCE_ARCHIVE_SHA256,
#        strip_prefix = _RENAISSANCE_SOURCE_ARCHIVE_PREFIX,
#        build_file = "//bazel:renaissance.BUILD",
#    )

load("@bazel_tools//tools/build_defs/repo:http.bzl", "http_jar")

_RENAISSANCE_BENCHMARKS_JAR_VERSION = "0.11.0"
_RENAISSANCE_BENCHMARKS_JAR_SHA256 = "6b6038cc0dfab4f44fa97f4918f75332ad91ae332db7867b18baf17ed55d2ce4"

def com_github_renaissance_benchmarks():
    http_jar(
        name = "com_github_renaissance_benchmarks",
        url = "https://github.com/renaissance-benchmarks/renaissance/releases/download/v{0}/renaissance-gpl-{0}.jar".format(_RENAISSANCE_BENCHMARKS_JAR_VERSION),
        sha256 = _RENAISSANCE_BENCHMARKS_JAR_SHA256
    )
