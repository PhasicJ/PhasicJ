def test(name, java_exec):
    native.sh_test(
        name = name,
        srcs = ["test.sh"],
        args = [
            "$(rootpath {})".format(java_exec),
            "$(rootpath //phasicj/agent:libpjagent.so)",
            "$(rootpath :TestApp.jar)",
            "$(rootpath //phasicj/agent/svm:libsvm.so)",
        ],
        data = [
            java_exec,
            ":TestApp.jar",
            # NOTE(dwtj): Just including `//phasicj/agent:agent` doesn't work. The
            #  `sh_test` rule doesn't include it in the test's runfiles. See
            #  [bazelbuild/bazel#6783](https://github.com/bazelbuild/bazel/issues/6783).
            "//phasicj/agent:libpjagent.so",
            "//phasicj/agent/svm:libsvm.so",
        ],
    )
