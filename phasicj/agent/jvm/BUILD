load("//bazel:docs.bzl", "index_md")

index_md()

exports_files(
    glob(["*.rs"]),
    visibility = ["//phasicj/agent:__pkg__"],
)

filegroup(
    name = "jvm",
    srcs = glob(["*.rs"]),
    visibility = ["//phasicj/agent:__pkg__"],
)
