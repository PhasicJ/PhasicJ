# Testing PhasicJ Instrumentation Features via Java Agent Deployment

PhasicJ uses instrumentation to generate certain events runtime within an
application under analysis. This package and its subpackages help test and
debug this instrumentation code.

To help test and debug the instrumentation code specifically, we do not
deploy this instrumentation code as normally (i.e. as a Graal Native Image
embedded in a JVMTI agent). Instead, we deploy the instrumentation code
within a Java Agent.
