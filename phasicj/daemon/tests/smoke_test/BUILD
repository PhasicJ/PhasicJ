sh_test(
    name = "smoke_test",
    srcs = ["smoke_test.sh"],
    data = [
        "//phasicj/daemon",
        "//phasicj/services/rtevents/rust/test:test_client",
    ],
    args = [
        "$(rootpath //phasicj/daemon)",
        "$(rootpath //phasicj/services/rtevents/rust/test:test_client)",
    ],
    env = {
        "RUST_LOG": "info",
        "RUST_BACKTRACE": "1",
    },
    # NOTE(dwtj): This test is since there is technically a race between the
    # start of the daemon and the start of the client.
    flaky = True,
)
