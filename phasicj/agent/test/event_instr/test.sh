#!/bin/sh -

set -euo pipefail

export JAVA_EXEC="$1"
export AGENT_PATH="$2"
export EXECUTABLE_JAR="$3"
export PHASICJ_EXEC="$4"

echo "JAVA_EXEC: $JAVA_EXEC"
echo "AGENT_PATH: $AGENT_PATH"
echo "EXECUTABLE_JAR: $EXECUTABLE_JAR"
echo "PHASICJ_EXEC: $PHASICJ_EXEC"

export AGENT_PATH="${PWD}/${AGENT_PATH}"

"./${JAVA_EXEC}" "-agentpath:${AGENT_PATH}=verbose,phasicj_exec=${PHASICJ_EXEC}" \
                 -jar "${EXECUTABLE_JAR}" \
    | tee stdout.log

# NOTE(dwtj): These `grep` commands make sure that these strings appears
#  somewhere in the above command's standard output. If one of them doesn't,
#  then `grep` will a non-zero exit code, and because `set -e` was set above,
#  this will fail this Bazel test.
grep 'Hello, from `phasicj.agent.test.event_instr.TestApp`, implemented in Java.' stdout.log > /dev/null
