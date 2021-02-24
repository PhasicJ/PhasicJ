#!/bin/sh -

set -e

PHASICJ_EXEC="$1"
INPUT_CLASS_FILE="$2"
INSTRUMENTED_CLASS_FILE="$INPUT_CLASS_FILE.instrumented.class"
JAVAP_OUTPUT_FILE="$INPUT_CLASS_FILE.instrumented.class.javap"

EXPECTED_INSTRUMENTED_METHOD_PATTERN="java/lang/Object.phasicj\$agent\$rt\$monitorEnter:()V"

"$PHASICJ_EXEC" agent instrument-class < "$INPUT_CLASS_FILE" > "$INSTRUMENTED_CLASS_FILE"
javap -c "$INSTRUMENTED_CLASS_FILE" > "$JAVAP_OUTPUT_FILE"

# Check to make sure that a method matching this pattern appears was added via
# instrumentation. If it does not appear in the `javap` output, then grep will
# fail return non-zero and this test will fail.
grep "$EXPECTED_INSTRUMENTED_METHOD_PATTERN" "$JAVAP_OUTPUT_FILE"
