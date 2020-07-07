# Architectural Technical Prototype Style Decisions

- Prototypes should be stored under [`//architecture/prototypes/`][1]. Each
prototype should be stored in its own directory with a name prefixed by `pt-`.

- Each prototype should be studying a fixed set of technical artifacts. For
example, the [ASM Java Agent][2] illustrates how the ASM library can be used
inside of a Java Agent to instrument JVM bytecode on the fly.

- Prototypes should be kept isolated from each other and from the PhasicJ
workspace. Each prototype instance should have its own independently configured
build system.

- Prototypes are not a part of the main Bazel workspace. Each prototype's
directory should be added to the [`.bazelignore`][3] file in the root of the
PhasicJ repository. (See [here][4] for background on the `.bazelignore` file.)

- Usually, a prototype should use Bazel as its (primary) build system. A
Bazel-based prototype should have its own [`WORKSPACE` file][5].

- Ideally, each prototype workspace should be able to successfully build and
test in every repository commit.

- Code within a prototype does not need to follow style guidelines as
stringently as main PhasicJ code.

---

[1]: /architecture/prototypes/
[2]: /architecture/prototypes/instrumentation/pt-asm_java_agent/README.md
[3]: /.bazelignore
[4]: https://docs.bazel.build/versions/3.3.0/guide.html#bazelignore
[5]: https://docs.bazel.build/versions/3.3.0/build-ref.html#workspace
