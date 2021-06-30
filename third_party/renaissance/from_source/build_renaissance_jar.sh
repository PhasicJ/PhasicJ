#!/bin/sh -

set -e

export BAZEL_EXECROOT=`pwd`

# E.g., `external/graalvm_linux_x64/jdk`
export JAVA_HOME="$1"

# E.g., `external/com_github_renaissance_benchmarks`
RENAISSANCE_BENCHMARKS_SOURCE_ROOT="$2"

# E.g., `bazel-out/k8-fastbuild/bin/third_party/renaissance/from_source/renaissance.jar`
OUTPUT_JAR="$3"

cd "$RENAISSANCE_BENCHMARKS_SOURCE_ROOT"

# NOTE(dwtj): We use the `--no-share` flag in our call to `sbt` to prevent
#  errors when it tries modify certain files in the user's home directory (e.g.,
#  `~/.sbt/boot/sb.boot.lock`). These attempts will fail when `sbt` is
#  run from within a Bazel action sandbox because these files are part of a
#  read-only filesystem.
./tools/sbt/bin/sbt \
   --no-share \
   --java-home "$OLDPWD/$JAVA_HOME" \
   assembly

cd $BAZEL_EXECROOT

mv "$RENAISSANCE_BENCHMARKS_SOURCE_ROOT"/target/renaissance-*.jar "$OUTPUT_JAR"
