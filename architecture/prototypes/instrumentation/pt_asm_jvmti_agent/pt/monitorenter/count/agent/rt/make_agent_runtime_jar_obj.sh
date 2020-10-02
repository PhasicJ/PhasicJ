#!/bin/sh -

set -e

SYMBOLS_NAME_BASE="$1"
INPUT_JAR_PATH="$2"
OUTPUT_OBJ_PATH="$3"

cp "${INPUT_JAR_PATH}" "${SYMBOLS_NAME_BASE}"

ld --relocatable \
    --format binary \
    --output "${OUTPUT_OBJ_PATH}" \
    --require-defined "_binary_${SYMBOLS_NAME_BASE}_end" \
    --require-defined "_binary_${SYMBOLS_NAME_BASE}_size" \
    --require-defined "_binary_${SYMBOLS_NAME_BASE}_start" \
    "${SYMBOLS_NAME_BASE}"
