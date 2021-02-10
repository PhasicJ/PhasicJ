load(
    "@rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_test",
)

rust_library(
    name = "phasicj_agent_instr",
    edition = "2018",
    srcs = ["lib.rs"],
)

rust_test(
    name = "phasicj_agent_instr_test",
    edition = "2018",
    srcs = ["test.rs"],
    deps = [
        ":phasicj_agent_instr",
        "//phasicj/agent/conf",
    ],
)
