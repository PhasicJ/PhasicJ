# Bazel Style Decisions

## Name Build Files `BUILD`

In the context of this repository, a file named `BUILD` will unambiguously be a
Bazel package build file. Don't name them `BUILD.bazel` or something else.

## Use `buildifier` Before Committing Changes to Bazel Files

There are three kinds of Bazel files:

- The Workspace file (i.e. `WORKSPACE` at the root of a repository.)
- Package build files (i.e. `BUILD` files).
- Starlark extension files (i.e. `<prefix>.bzl` files).

The [`buildifier`][1] tool can be used as a linter and as a code formatter for
these kinds of files. For this project, prefer all style conventions enforced by
`buildifier` for these kinds of files.

Adapting [this guidance][2] from the `buildifier` `README.md`, two bazel targets
have been added to this project to help automate use of `buildifier`:

- `bazel run //devtools/lint/bazel:buildifier` will lint all Bazel files in the
repo. Warnings will be printed to `stderr`.
- `bazel run //devtools/format/bazel:buildifier` will format all Bazel files in
the repo.

Before committing changes to the repository, any lint warnings/errors should be
fixed and the formatter should be run.

Note that the `buildifier` tool is currently made available to bazel by
*building it with bazel*. This is the approach currently recommended by the
`buildifier` `README.md`. This approach makes the build more hermetic, but
it introduces some undesired complexity to the workspace. In particular, Go,
Protobuf, and Gazelle are all loaded as external repositories. (See the
[`WORKSPACE` file](/WORKSPACE).)

## Consider Using `buildifier` in Your Editor

Note that `buildifier` can be used automatically by your editor/IDE to reveal
lint warnings/errors on-the-fly. For example, the plugin [`vscode-bazel`][3]
from the Bazel team has `buildifier` support.

---

[1]: https://github.com/bazelbuild/buildtools/blob/a60df6e5d134ed103669f206ad74ec2154f1a562/buildifier/README.md
[2]: https://github.com/bazelbuild/buildtools/blob/a60df6e5d134ed103669f206ad74ec2154f1a562/buildifier/README.md#setup-and-usage-via-bazel
[3]: https://github.com/bazelbuild/vscode-bazel
