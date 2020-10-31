# Rust Dependency Management with Cargo Raze

[cargo-raze][1] is a plugin for Cargo which creates Bazel packages and targets
to build Rust dependencies obtained from the Cargo ecosystem. The targets in
this package can be used as `deps` in our own Bazel Rust targets. For exmaple,

```
rust_library(
    name = "mylib",
    srcs = ["mylib.rs"],
    deps = ["//third_party/cargo:some_cargo_dependency"],
)
```

The `Cargo.toml` file in this directory specifies a "fake" library. We don't
actually care about building this library. It is just a way for us to list all
of the Cargo dependencies which we want to use throughout our workspace and to
specify some cargo-raze-specific configuration. Essentially, this `Cargo.toml`
is how we encode cargo-raze's inputs.

When `cargo raze` is invoked from this directory, the cargo-raze plugin is able
to discover this list of direct dependencies, their transitive dependencies and
the interdependencies between them. It is then able to use this information to
compile appropriate Bazel `BUILD` files which contain Bazel targets which can
build each of these dependencies.

Additionally, because we are using "Remote" (i.e. non-vendored) mode, a `cargo
raze` invocation creates the `crates.bzl` file. This includes a repository
macro which we call from the root `WORKSPACE` file.  Calling this macro creates
external repositories for our downloaded copies of our dependencies.

We commit all of these `cargo raze` outputs to the Git repository. When the
list of dependencies in this directory's `Cargo.toml` changes, `cargo raze`
should be re-run from this directory and any changes should be committed to the
Git repository.

Before one can run a `cargo raze` command, one must first install the
cargo-raze plugin locally. This can be done by invoking `cargo install
cargo-raze`.

---

[1]: https://github.com/google/cargo-raze
