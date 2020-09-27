#!/bin/sh -

set -e

JAVA_EXEC="$1"
AGENT_JAR="$2"
ASM_JAR="$3"
RENAISSANCE_JAR="$4"

"${JAVA_EXEC}" "-Xbootclasspath/a:${AGENT_JAR}:${ASM_JAR}" \
    "-javaagent:${AGENT_JAR}" \
    -jar "${RENAISSANCE_JAR}" \
    --repetitions 1 \
    scrabble
