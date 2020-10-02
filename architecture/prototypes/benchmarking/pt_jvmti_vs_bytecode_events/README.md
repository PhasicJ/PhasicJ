# Prototype Comparing Performance of JVMTI Events vs JVM Bytecode Events

Currently, one requirement for PhasicJ is to collect and process low-level
JVM events (e.g. `MONITORENTER` instructions). For some of these events, both
JVMTI and JVM bytecode instrumentation (via ASM or similar) provide
opportunities to be informed of these events.

This prototype is designed to collect metrics to help compare the performance
characteristics of both approches. These metics are meant to help inform the
architecture decision-making process.
