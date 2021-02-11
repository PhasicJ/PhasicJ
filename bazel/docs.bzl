load("@dwtj_rules_markdown//markdown:defs.bzl", "markdown_library")

def index_md(name = "index_md"):
    markdown_library(
        name = name,
        srcs = ["INDEX.md"],
    )
