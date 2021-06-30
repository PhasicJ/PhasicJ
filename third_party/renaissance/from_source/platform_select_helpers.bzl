"""Helper functions in this file are used to help build `select()` expressions.

These functions are only meant to be used in the definition of the genrule
`//third_party/renaissance/from_source:jar`, which compiles the Renaissance
benchmarks from source using the `sbt` build tool to make a single self-
contained JAR file.
"""

def cmd_bash_select_clause(os):
    if os != "macos" and os != "linux":
        # TODO(dwtj): Add better error reporting!
        return "ERROR!"

    return '"$(rootpath :build_renaissance_jar.sh)" \
                `dirname $(rootpath @graalvm_{}_x64//jdk:BUILD)` \
                `dirname $(rootpath @com_github_renaissance_benchmarks//:build.sbt)` \
                $@'.format(os)

# NOTE(dwtj): We use the `@graalvm_<OS>_x64//jdk:BUILD` file to help give our
#  script a path to the `JAVA_HOME` directory.
# NOTE(dwtj): We use the `@com_github_renaissance_benchmarks//:build.sbt` file
#  to help give ours script a path to the root of the Renaissance source
#  directory.
# TODO(dwtj): Using these is a bit of a hack. Is there a better way?
def srcs_select_clause(os):
    if os != "macos" and os != "linux":
        # TODO(dwtj): Add better error reporting!
        return "ERROR!"

    return [
        "build_renaissance_jar.sh",
        "@graalvm_{}_x64//jdk:BUILD".format(os),
        "@graalvm_{}_x64//jdk:java_home".format(os),
        "@com_github_renaissance_benchmarks//:all_renaissance_source_files",
        "@com_github_renaissance_benchmarks//:build.sbt",

    ]
