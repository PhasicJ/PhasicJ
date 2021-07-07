#!/bin/sh -

set -euo pipefail

export JAVA_EXEC="$1"
export AGENT_PATH="$2"
export EXECUTABLE_JAR="$3"
export PHASICJ_EXEC="$4"
export DAEMON_EXEC="$5"

export DAEMON_SOCKET="pjrtevents.S"
export DAEMON_DATABASE="pjrtevents.db"
export DAEMON_DATABASE_TXT="pjrtevents.db.txt"
export SQLITE3_EXEC="sqlite3"

# export EXPECTED_NUMBER_OF_EVENTS="???"

echo "JAVA_EXEC: $JAVA_EXEC"
echo "AGENT_PATH: $AGENT_PATH"
echo "EXECUTABLE_JAR: $EXECUTABLE_JAR"
echo "PHASICJ_EXEC: $PHASICJ_EXEC"
echo "DAEMON_EXEC: $DAEMON_EXEC"

export AGENT_PATH="${PWD}/${AGENT_PATH}"

"$DAEMON_EXEC" --socket "$DAEMON_SOCKET" --database "$DAEMON_DATABASE" &
DAEMON_PID=$!

# Wait for the daemon to bind to the socket.
sleep 1

"${JAVA_EXEC}" "-agentpath:${AGENT_PATH}=verbose,phasicj_exec=${PHASICJ_EXEC},daemon_socket=${DAEMON_SOCKET}" \
               -jar "${EXECUTABLE_JAR}"

# Wait for the daemon to finish using the database.
sleep 1
kill "$DAEMON_PID"

"$SQLITE3_EXEC" "$DAEMON_DATABASE" 'select * from rtevents' > "$DAEMON_DATABASE_TXT"
cat "$DAEMON_DATABASE_TXT"

# TODO(dwtj): Figure out an expected number of events.
#ACTUAL_NUMBER_OF_EVENTS=$("$SQLITE3_EXEC" "$DAEMON_DATABASE" 'select count(id) from rtevents')
#if [ "$ACTUAL_NUMBER_OF_EVENTS" -ne "$EXPECTED_NUMBER_OF_EVENTS" ]; then
#   echo "Error: Unexpected number of events in database!"
#   echo "Expected: $EXPECTED_NUMBER_OF_EVENTS"
#   echo "Actual: $ACTUAL_NUMBER_OF_EVENTS"
#   exit 1
#fi
