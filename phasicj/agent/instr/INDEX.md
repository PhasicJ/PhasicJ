# PhasicJ Agent Instrumentation Support

The PhasicJ agent collects various events which occur within the user's JVM
application under analysis and forwards these to other PhasicJ components to be
analyzed. The agent collects some of these events by instrumenting the user's
JVM application. This package is responsible for implementing this
instrumentation code. This package's subpackages are responsible for wrapping
and/or testing this instrumentation code.

This Bazel package is named `instr`, which stands for "instrumentation".
Within this package we use "instr" as an abbreviation for "instrumentation"
and its variants, for example, "instrument", "instrumenter", etc. (Note that
this abbreviation is similar to but distinct from "insn", an abbreviation
which we use for instruction, following the ASM library's example.)
