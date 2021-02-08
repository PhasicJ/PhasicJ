#!/bin/sh -

set -e

TEST_REPL_EXEC="$1"

"$TEST_REPL_EXEC" <<EOF
debug_dump_classes_before_instr
EOF
