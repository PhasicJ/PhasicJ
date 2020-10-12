#!/bin/sh -

set -e

JAVA_EXEC="$1"
JVMTI_AGENT_SHARED_LIBRARY=$2
MAIN_JAR="$3"

JVMTI_AGENT_SHARED_LIBRARY_ABSOLUTE_PATH="${PWD}/${JVMTI_AGENT_SHARED_LIBRARY}"

"./$JAVA_EXEC" \
    "-agentpath:${JVMTI_AGENT_SHARED_LIBRARY_ABSOLUTE_PATH}" \
    -jar "$MAIN_JAR" | tee test.log

grep 'Hello, from `pt.agent.test.TestApp`.' test.log
grep 'Hello, from `pt.svm.Analysis`, implemented in Java.' test.log
