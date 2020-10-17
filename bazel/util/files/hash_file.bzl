def sha1_hash_file(name, src, out, visibility = ["//visibility:__pkg__"]):
    native.genrule(
        name = name,
        srcs = [src],
        outs = [out],
        # NOTE(dwtj): I'm using `awk` because my previous attempt to use `cut`
        #  added a trailing newline. `sed` might also work but it's noisy and
        #  error prone.
        cmd_bash = '''shasum "$<" | awk '{printf($$1)}' > "$(@)"''',
    )
