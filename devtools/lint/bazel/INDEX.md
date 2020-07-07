# Lint PhasicJ's Bazel Files

## Lint Files Using Buildifier Bazel Target

The [buildifier][1] tool can be used to lint Bazel BUILD files and Starlark
files (i.e. `.bzl` files). [The buildifier README][2] describes how
buildifier can be invoked as a Bazel target. Such a bazel target is defined
in this package, and it can be run with the following command:

```sh
bazel run //devtools/lint/bazel:buildifier
```

---

[1]: https://github.com/bazelbuild/buildtools/blob/a60df6e5d134ed103669f206ad74ec2154f1a562/buildifier/README.md
[2]: https://github.com/bazelbuild/buildtools/blob/a60df6e5d134ed103669f206ad74ec2154f1a562/buildifier/README.md#setup-and-usage-via-bazel
