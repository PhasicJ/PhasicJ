# PhasicJ Agent JVM Integration

The PhasicJ agent is a JVMTI agent which runs inside of a JVM process. This
Bazel package includes code which directly interacts with the JVM via the JVMTI
and JNI APIs. These files are Rust submodules meant to be included in the Rust
target `//phasicj/agent:agent` under the Rust module, `crate::jvm::*`.
