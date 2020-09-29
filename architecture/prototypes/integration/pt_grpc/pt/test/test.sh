#!/bin/sh -

set -e

MY_SERVER_EXEC="$1"
MY_CLIENT_EXEC="$2"

timeout 5s "./${MY_SERVER_EXEC}" &
sleep 1s
"./${MY_CLIENT_EXEC}"
