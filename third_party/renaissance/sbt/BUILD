load(
    ":platform_select_helpers.bzl",
    "srcs_select_clause",
    "cmd_bash_select_clause",
)

load("@rules_java//java:defs.bzl", "java_import")

package(
    default_visibility = ["//third_party/renaissance:__subpackages__"],
)

# The JAR file compiled during `:sbt_assemble` will have a timestamp included in
# the file name. We rename this file to a known name so that we can tell Bazel
# the expect output name.
_CANONICAL_JAR_NAME="renaissance-gpl-0.12.0-SNAPSHOT.jar"

genrule(
    name = "jar",
    outs = [_CANONICAL_JAR_NAME],
    srcs = select({
        "@platforms//os:macos": srcs_select_clause("macos"),
        "@platforms//os:linux": srcs_select_clause("linux"),
    }),
    cmd_bash = select({
        "@platforms//os:macos": cmd_bash_select_clause("macos"),
        "@platforms//os:linux": cmd_bash_select_clause("linux"),
    }),
    # TODO(dwtj): Try to remove this.
    # NOTE(dwtj): When local = False (the default), `sbt` will error when it
    # tries to write to `~/.cache/coursier` because the sandbox makes this
    # part of the filesystem read-only.
    local = True,
)

java_import(
    name = "legacy_java_import",
    jars = [":jar"],
)
