load(
    ":platform_select_helpers.bzl",
    "srcs_select_clause",
    "cmd_bash_select_clause",
)

package(
    default_visibility = ["//third_party/renaissance:__subpackages__"],
)

# NOTE(dwtj): The JAR file compiled during `:jar` will have a timestamp
# included in the file name. We rename this file to a known name,
# `renaissance.jar` so that we can tell Bazel where to expect the output name.
_CANONICAL_JAR_NAME="renaissance.jar"

# TODO(dwtj): Figure out what is wrong with this.
# genrule(
#     name = "jar",
#     outs = [_CANONICAL_JAR_NAME],
#     srcs = select({
#         "@platforms//os:macos": srcs_select_clause("macos"),
#         "@platforms//os:linux": srcs_select_clause("linux"),
#     }),
#     cmd_bash = select({
#         "@platforms//os:macos": cmd_bash_select_clause("macos"),
#         "@platforms//os:linux": cmd_bash_select_clause("linux"),
#     }),
#     # TODO(dwtj): Figure out how to make this work without making it local.
#     # Specifically, I'm still having problems with `~/.cache/coursier/`.
#     local = True,
# )
