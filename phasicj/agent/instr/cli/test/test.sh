#!/bin/sh -

set -e

INSTR_EXEC="$1"
INPUT_CLASS_FILE="$2"
OUTPUT_CLASS_FILE="$2.instrumented.class"
JAVAP_OUTPUT_FILE="$2.instrumented.class.javap"
EXPECTED_INSTRUMENTED_METHOD_PATTERN="java/lang/Object.phasicj\$agent\$rt\$monitorEnter:()V"

"$INSTR_EXEC" < "$INPUT_CLASS_FILE" > "$OUTPUT_CLASS_FILE"
javap -c "$OUTPUT_CLASS_FILE" > "$JAVAP_OUTPUT_FILE"

# Check to make sure that
grep "$EXPECTED_INSTRUMENTED_METHOD_PATTERN" "$JAVAP_OUTPUT_FILE"
