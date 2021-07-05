sh_test(
    name = "smoke_test",
    srcs = ["smoke_test.sh"],
    data = [
        "//phasicj/daemon",
        "//phasicj/daemon/tests/sqlite_replay_client",
        ":hello_world.db",
    ],
    args = [
        "$(rootpath //phasicj/daemon)",
        "$(rootpath //phasicj/daemon/tests/sqlite_replay_client)",
        "$(rootpath :hello_world.db)",
    ],
    env = {
        "RUST_LOG": "info",
        "RUST_BACKTRACE": "1",
    },
    # NOTE(dwtj): This test is flaky since there is technically a race between
    # the daemon binding to the socket and the client connecting to the socket.
    flaky = True,
)

genrule(
    name = "make_hello_world_db",
    outs = ["hello_world.db"],
    # TODO(dwtj): This script relies on getting `sqlite3` executable via the
    #  system PATH. Consider ways to get it through Bazel instead.
    cmd_bash = '"$(rootpath :make_hello_world_db.sh)" "$@"',
    srcs = [":make_hello_world_db.sh"],
)
