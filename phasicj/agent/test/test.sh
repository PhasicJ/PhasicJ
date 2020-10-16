#!/bin/sh -

set -e

export JAVA_EXEC="$1"
export AGENT_PATH="$2"
export EXECUTABLE_JAR="$3"

AGENT_PATH="${PWD}/${AGENT_PATH}"

"./${JAVA_EXEC}" "-agentpath:${AGENT_PATH}" -jar "${EXECUTABLE_JAR}" | tee stdout.log

# NOTE(dwtj): These `grep` commands make sure that these strings appears
#  somewhere in the above command's standard output. If one of them doesn't,
#  then `grep` will a non-zero exit code, and because `set -e` was set above,
#  this will fail this Bazel test.
grep 'Hello, from `on_load()`, implemented in Rust.' stdout.log
grep 'Hello, from `phasicj.agent.test.TestApp`, implemented in Java.' stdout.log
