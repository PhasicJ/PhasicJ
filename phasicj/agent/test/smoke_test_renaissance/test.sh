#!/bin/sh -

set -e

JAVA_EXEC="$1"
AGENT_PATH="$2"
RENAISSANCE_CLASS_PATH="$3"
RENAISSANCE_MAIN_CLASS="$4"
AGENT_OPTIONS="$5"
shift 5

AGENT_PATH="${PWD}/${AGENT_PATH}"

"./${JAVA_EXEC}" "-agentpath:${AGENT_PATH}=${AGENT_OPTIONS}" \
                 --class-path "${RENAISSANCE_CLASS_PATH}" \
                 "$RENAISSANCE_MAIN_CLASS" \
                 "$@"
