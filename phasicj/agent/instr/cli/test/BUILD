sh_test(
    name = "test",
    srcs = ["test.sh"],
    data = [
        "//phasicj/agent/instr/cli:phasicj_agent_instr",
        "//phasicj/agent/instr/test/class_data/simple:SynchBlock.class",
    ],
    args = [
        "$(rootpath //phasicj/agent/instr/cli:phasicj_agent_instr)",
        "$(rootpath //phasicj/agent/instr/test/class_data/simple:SynchBlock.class)",
    ],
)
