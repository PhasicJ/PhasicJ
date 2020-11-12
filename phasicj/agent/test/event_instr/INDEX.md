# Check For Some Instrumented PhasicJ Events

The PhasicJ Agent uses JVM bytecode instrumentation to forward various JVM
events to the PhasicJ Runtime (e.g. `MONITORENTER` instructions). This
package checks that for a trivial JVM program, `TestApp`, at least some of
these events are emitted by instrumentation sites and handled by the runtime.
