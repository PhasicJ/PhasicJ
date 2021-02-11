load("//bazel:docs.bzl", "index_md")

load("@com_github_bazelbuild_buildtools//buildifier:def.bzl", "buildifier")

index_md()

buildifier(
    name = "buildifier",
    lint_mode = "warn",
    mode = "check",
    verbose = True,
)
