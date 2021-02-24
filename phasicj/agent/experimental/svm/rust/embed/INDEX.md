# Rust Helper Code to Extracting Embedded SVM Library to Filesystem

This package defines a Rust crate which includes an embedded copy of the SVM
shared library (which was compiled by GraalVM `native-image`). This crate
also includes helper functions to write this to the local file system (so that
it can then be dynamically linked).
