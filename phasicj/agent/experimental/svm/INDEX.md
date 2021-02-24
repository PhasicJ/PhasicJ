# Experimental PhasicJ Agent Substrate VM Module

NOTE(dwtj): This module is currently marked experimental because my attempts
to integrate it with the PhasicJ Agent have stalled.

Substrate VM (SVM) is the virtual machine (or alternatively, "runtime") used as
the foundations for Java programs when compiled to native code using the
GraalVM `native-image` tool.

We have some Java code which we want to use from our native, Rust-implemented
PhasicJ agent. We only want one copy of SVM to be included in the PhasicJ agent.
This Bazel package is used for wrapping our `native-image`/SVM library.

More specifically, this package has three purposes:

- To combine and compile all of the PhasicJ agent's Java dependencies with a
  single `native-image` invocation.
- To use `bindgen` to generate unsafe Rust bindings for this library.
- Possibly write some Rust code to wrap these unsafe bindings in safe code.

Through these layers of adaptation, the PhasicJ agent should be able to call
our `native-image`-compiled Java code.
