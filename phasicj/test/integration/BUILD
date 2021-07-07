sh_test(
    name = "smoke_test",
    srcs = ["smoke_test.sh"],
    args = [
        "$(rootpath //third_party/openjdk:java)",
        "$(rootpath //phasicj/agent:libpjagent)",
        "$(rootpath :test_app_jar)",
        "$(rootpath //phasicj/cli)",
        "$(rootpath //phasicj/daemon)",
    ],
    data = [
        "//third_party/openjdk:java",

        # NOTE(dwtj): Just including `//phasicj/agent:agent` doesn't work.
        #  The `sh_test` rule doesn't include it in the test's runfiles.
        #  See [bazelbuild/bazel#6783][1]. Fortunately, we seem to be able
        #  to work around this problem using our `file_copy()` macro and
        #  its underlying `genrule()`.
        #
        #  [1]: https://github.com/bazelbuild/bazel/issues/6783).
        "//phasicj/agent:libpjagent",

        ":test_app_jar",
        "//phasicj/cli",
        "//phasicj/daemon",
    ],
    env = {
        "RUST_LOG": "pjeventsd,agent",
        "RUST_BACKTRACE": "1",
    },
    # NOTE(dwtj): We make this flaky because there is a race between the
    # binding of the daemon to the socket and the connection of the agent to the
    # socket.
    flaky = True,
)

alias(
    name = "test_app_jar",
    actual = "//phasicj/test/java_apps:hello_synchronized.jar",
)
