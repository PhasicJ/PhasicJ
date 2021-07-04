#!/bin/sh -

set -e

DAEMON_EXEC="$1"
CLIENT_EXEC="$2"

UDS_NAME="pjevents.S"
DATABASE_NAME="pjevents.db"

"$DAEMON_EXEC" --socket "$UDS_NAME" --database "$DATABASE_NAME" &
DAEMON_PID="$!"
echo Daemon PID: "$DAEMON_PID"

sleep 1

"$CLIENT_EXEC" "$UDS_NAME" &
CLIENT_PID="$!"
echo Client PID: "$CLIENT_PID"

echo 'Waiting for client to finish'
wait "$CLIENT_PID"
