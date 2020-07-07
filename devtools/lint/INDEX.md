# Linters

This directory holds some code, configuration, and documentation to help run the
various linters used by this project.

These linters are meant to be just some of the static style checks expected
by the PhasicJ project.

Here, "linter" means some sort of simple standalone checking tool which can run
on individual files without any context. Thus, these tools can be run without
being integrated into the build or deployment process. For example,
[`markdownlint`][1] is a linter, whereas the Java type checker is not a
linter.

---

[1]: https://github.com/DavidAnson/markdownlint
