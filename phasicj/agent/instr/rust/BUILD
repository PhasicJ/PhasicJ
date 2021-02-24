load(
    "@rules_rust//rust:rust.bzl",
    "rust_library",
    "rust_test",
)

package(
    default_visibility = ["//phasicj/agent:__subpackages__"],
)

alias(
    name = "rust",
    actual = ":phasicj_agent_instr_rust",
)

rust_library(
    name = "phasicj_agent_instr_rust",
    srcs = ["lib.rs"],
    edition = "2018",
)

_SYNCHBLOCK_CLASS = "//phasicj/agent/instr/test/class_data/simple:SynchBlock.class"

rust_test(
    name = "test",
    srcs = ["test.rs"],
    edition = "2018",
    deps = [":phasicj_agent_instr_rust"],
    compile_data = [_SYNCHBLOCK_CLASS],
    rustc_env = {
        "SYNCHBLOCK_CLASS": "$(execpath {})".format(_SYNCHBLOCK_CLASS),
    },
)
