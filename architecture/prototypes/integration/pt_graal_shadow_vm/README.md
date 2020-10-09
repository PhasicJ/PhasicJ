# Prototype of Graal Shadow VM

This document gives an overview of the design and purpose of this prototype.

This prototype is meant to demonstrate a particular potential strategy for
deploying analyses of Java applications using a combination of technologies:
the JVM, [JVMTI][jvmti], and GraalVM's `native-image`.

GraalVM's `native-image` tool provides support to compile Java applications to
native code. This prototype demonstrates how JVMTI can be used to deploy a
`native-image` application which can instrument, monitor, and analyze a Java
application. In this design, we have two Java programs operating simultaneously:

- An application. This is the program under analysis. This is the user's
  Java program running on a traditional JVM. We primarily target variants of
  OpenJDK + HotspotVM.
- An analysis. This is a program performing an analysis of the application.
  This is the Java application which we compile using `native-image`. It runs
  supported by the GraalVM project's SubstrateVM.

This analysis program is at run-time a second VM. Following the example of
Marek et al. [1][marek2013shadowvm], we call this second VM the shadow VM.
(Note that that work does not use the GraalVM `native-image` tool, just a
traditional JVM in a separate process.)

In this prototype, our shadow VM is deployed via JVMTI agent. Specifically, the
shadow VM's executable code is embedded in a JVMTI agent. Then, when the JVMTI
agent is installed in an application JVM, the JVMTI agent extracts this shadow
VM's executable and runs it in a separate process.

Our JVMTI agent instruments the application process to generate events
describing its behavior as the application executes. These events should be
sent to the analysis process in a timely manner.

This prototype is intended to clarify some key aspects related to defining,
building, deploying, and testing components arranged in this manner. It does
not explore implementing any sophisticated or even useful instrumentation or
analysis. Said another way, this prototype's instrumentation and analysis is
kept intentionally trivial.

---

[jvmti]: https://docs.oracle.com/en/java/javase/14/docs/specs/jvmti.html
[marek2013shadowvm]: https://dl.acm.org/doi/abs/10.1145/2637365.2517219
