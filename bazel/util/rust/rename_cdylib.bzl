load("//bazel:util/files/copy_file.bzl", "copy_file")

def rename_cdylib(name, src, out, visibility = ["//visibility:__pkg__"]):
    copy_file(
        name = name,
        src = src,
        out = out,
        visibility = visibility,
    )
