# Prototype: Deploy a JAR to a JVM embedded in a JVMTI Agent

This integration prototype attempts to demonstrate how a Java JAR can be
deployed to a JVM by embedding the JAR within a JVMTI agent.

One goal is to require as little configuration from the user as possible. (In
particular, the user should not need to modify their application's class path
configuration.) Ideally, the user should only need to modify their `java`
invocation by adding an `-agentlib` or `-agentpath` argument.
