#!/bin/sh -

set -e

OUTPUT_DB_FILE="$1"

SQLITE3_EXEC="sqlite3"

"$SQLITE3_EXEC" "$OUTPUT_DB_FILE" <<HERE_DOC_EOF
CREATE TABLE rtevents (id INTEGER PRIMARY KEY, description TEXT);
INSERT INTO rtevents (description) VALUES ("Hello, PhasicJ!");
HERE_DOC_EOF
