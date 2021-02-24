#!/bin/sh -

set -e

export JAVA_EXEC="$1"
export AGENT_PATH="$2"
export RENAISSANCE_CLASS_PATH="$3"
export RENAISSANCE_MAIN_CLASS="$4"
export PHASICJ_EXEC="$5"
export EXTRA_AGENT_OPTIONS="$6"
shift 6

export PATH="$PATH:$(dirname $PHASICJ_EXEC)"

AGENT_PATH="${PWD}/${AGENT_PATH}"

"./${JAVA_EXEC}" "-agentpath:${AGENT_PATH}=phasicj_exec=${PHASICJ_EXEC},${EXTRA_AGENT_OPTIONS}" \
                 --class-path "${RENAISSANCE_CLASS_PATH}" \
                 "$RENAISSANCE_MAIN_CLASS" \
                 "$@"
