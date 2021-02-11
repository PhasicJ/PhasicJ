load("//bazel:docs.bzl", "index_md")
load("@com_github_bazelbuild_buildtools//buildifier:def.bzl", "buildifier")

index_md()

buildifier(
    name = "buildifier",
    mode = "fix",
    verbose = True,
)

alias(
    name = "bazel",
    actual = ":buildifier",
)
