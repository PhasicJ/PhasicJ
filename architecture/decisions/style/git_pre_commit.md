# Git Repository Pre-Commit Style Decisions

- A PhasicJ developer should ensure that both `bazel build //...` and
`bazel test //...` should pass without error (and preferably without warnings)
before commiting changes to this Git repository.

- Pre-commit checks should be automated, possibly supported by Git's
pre-commit hook. However, this has not yet been implemented.

- When pre-commit checks are implemented, they should be used to automate style
checks when possible. Ideally, these automated checks should be integrated into
Bazel, either

  - via `bazel build` (like how `markdownlint` is run automatically during
  `bazel build //...` using `dwtj_rules_markdown`), or
  - via `bazel run` (like how `buildifier` is run using
  `bazel run //devtools/lint/bazel:buildifier`).
