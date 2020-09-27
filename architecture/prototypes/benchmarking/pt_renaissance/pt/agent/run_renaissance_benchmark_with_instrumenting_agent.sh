#!/bin/sh -

find .

JAVA_EXEC="java"
AGENT_JAR="pt/agent/agent.jar"
ASM_JAR="external/org_ow2_asm/jar/downloaded.jar"
RENAISSANCE_JAR="external/com_github_renaissance_benchmarks/jar/downloaded.jar"

"${JAVA_EXEC}" "-Xbootclasspath/a:${AGENT_JAR}:${ASM_JAR}" \
    "-javaagent:${AGENT_JAR}" \
    -jar "${RENAISSANCE_JAR}" \
    --repetitions 1 \
    scrabble
