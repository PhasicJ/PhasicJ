load("@dwtj_rules_markdown//markdown:defs.bzl", "markdown_library")

markdown_library(
    name = "all_markdown",
    srcs = glob(["*.md"]),
)

exports_files(
    glob(["*.rs"]),
    visibility = ["//phasicj/agent:__pkg__"],
)

filegroup(
    name = "jvm",
    srcs = glob(["*.rs"]),
    visibility = ["//phasicj/agent:__pkg__"],
)
