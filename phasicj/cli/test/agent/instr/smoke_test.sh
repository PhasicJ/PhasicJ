#!/bin/sh -

set -e

PHASICJ_EXEC="$1"
CLASS_TO_INSTR="$2"

"$PHASICJ_EXEC" agent instrument-class < "$CLASS_TO_INSTR"
