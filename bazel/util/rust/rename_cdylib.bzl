def rename_cdylib(name, src, out, visibility = ["//visibility:__pkg__"]):
    native.genrule(
        name = name,
        srcs = [src],
        outs = [out],
        cmd = "cp $< $(@)",
        visibility = visibility,
    )
