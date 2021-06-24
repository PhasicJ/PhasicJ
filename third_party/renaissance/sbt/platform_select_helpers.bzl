"""Helper functions in this file are used to help build `select()` expressions.

These functions are only meant to be used in the definition of the genrule
`//third_party/renaissance/sbt:jar`, which compiles the Renaissance benchmarks
from source using the `sbt` build tool and makes a single self-contained JAR
file.
"""

# TODO(dwtj): How can we make these paths less brittle?
_ENTER_RENAISSANCE_SOURCE_ARCHIVE_ROOT="./external/com_github_renaissance_benchmarks"
_LEAVE_RENAISSANCE_SOURCE_ARCHIVE_ROOT="../.."

# NOTE(dwtj): We use the `@graalvm_<OS>_x64//jdk:BUILD` file to help find the
# `JAVA_HOME` directory.
# NOTE(dwtj): We use the `--no-share` flag in our call to `sbt` to prevent
# problems when it tries to grab certain file locks (e.g.,
# `~/.sbt/boot/sb.boot.lock`). The problem is that when `sbt` is run from within
# a Bazel action sandbox, these files are part of a read-only filesystem, so
# attempts to grab them will fail. Some other approaches which I considered to
# solve this problem: use the `--sbt-dir` and `--ivy` flags; use `local = True`.
def cmd_bash_select_clause(os):
    if os != "macos" and os != "linux":
        # TODO(dwtj): Add better error reporting!
        return "ERROR!"

    return "\
        cd '{0}' \
        && ./tools/sbt/bin/sbt \
               --no-share \
               --java-home {2}/`dirname $(rootpath @graalvm_{1}_x64//jdk:BUILD)` \
               assembly \
        && cd '{2}' \
        && mv {0}/target/renaissance-gpl-0.12.0-*.jar $@".format(
            _ENTER_RENAISSANCE_SOURCE_ARCHIVE_ROOT,
            os,
            _LEAVE_RENAISSANCE_SOURCE_ARCHIVE_ROOT,
        )


def srcs_select_clause(os):
    if os != "macos" and os != "linux":
        # TODO(dwtj): Add better error reporting!
        return "ERROR!"

    return [
        "@graalvm_{}_x64//jdk:BUILD".format(os),
        "@graalvm_{}_x64//jdk:java_home".format(os),
        "@com_github_renaissance_benchmarks//:all_renaissance_source_files",
    ]
