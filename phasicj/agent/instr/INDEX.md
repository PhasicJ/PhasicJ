# PhasicJ Agent Instrumentation Support

The PhasicJ agent collects and forwards various events from the user's JVM
application under analysis. It collects some of these events by instrumenting
the user's JVM application. This package and its subpackages are responsible
for this instrumentation code.

This Bazel package is named `instr`. Within package this use this name as an
abbreviation for "instrumentation" and its variants (e.g., "instrument",
"instrumenter", etc.). Note that this abbreviation is similar but distinct from
"insn", which is an abbreviation which we use (following ASM's example) for
"instruction".
