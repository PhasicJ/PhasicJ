# ASM Bytecode Instrumentation from JVMTI Agent

This integration prototype attempts to demonstrate how ASM bytecode
instrumentation can be performed over all classes in a JVM when deployed from a
alone JVMTI agent.

One goal is to require as little configuration from the user as possible. (In
particular, the user should not need to modify their application's class path
configuration.) Ideally, the user should only need to modify their `java`
invocation by adding an `-agentlib` or `-agentpath` argument.
