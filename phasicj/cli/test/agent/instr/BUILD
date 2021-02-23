sh_test(
    name = "smoke_test",
    srcs = ["smoke_test.sh"],
    data = [
        "//phasicj/cli",
        "//phasicj/agent/instr/test/class_data/simple:SynchBlock_class",
    ],
    args = [
        "$(rootpath //phasicj/cli)",
        "$(rootpath //phasicj/agent/instr/test/class_data/simple:SynchBlock_class)",
    ],
)
