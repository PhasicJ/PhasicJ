#!/bin/sh -

set -e

DAEMON_EXEC="$1"
CLIENT_EXEC="$2"

UDS_NAME="pjevents.S"
DATABASE_NAME="pjevents.db"
export DATABASE_TEXT_FILE="pjevents.txt"

EXPECTED_NUMBER_OF_LINES="1"
EXPECTED_PATTERN_TO_FIND="Hello, Tonic!"

"$DAEMON_EXEC" --socket "$UDS_NAME" --database "$DATABASE_NAME" &
DAEMON_PID="$!"
echo Daemon PID: "$DAEMON_PID"

sleep 1

"$CLIENT_EXEC" "$UDS_NAME" &
CLIENT_PID="$!"
echo Client PID: "$CLIENT_PID"

echo 'Waiting for client to finish'
wait "$CLIENT_PID"

sqlite3 $DATABASE_NAME 'select * from rtevents' > "$DATABASE_TEXT_FILE"

# Check that the contents of the database are as expected.
ACTUAL_NUMBER_OF_LINES=$(wc -l "$DATABASE_TEXT_FILE" | cut -d " " -f1)
if [ "$ACTUAL_NUMBER_OF_LINES" -ne "$EXPECTED_NUMBER_OF_LINES" ]; then
   echo "Error: Unexpected number of lines"
   exit 1
fi

grep "$EXPECTED_PATTERN_TO_FIND" "$DATABASE_TEXT_FILE"
