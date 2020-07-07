load("@dwtj_rules_markdown//markdown:defs.bzl", "markdown_library")
load("@com_github_bazelbuild_buildtools//buildifier:def.bzl", "buildifier")

markdown_library(
    name = "markdown",
    srcs = glob(["*.md"])
)

buildifier(
    name = "buildifier",
    lint_mode = "warn",
    mode = "check",
    verbose = True,
)
