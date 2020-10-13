load("@dwtj_rules_java//java:defs.bzl", "dwtj_java_agent")

dwtj_java_agent(
    name = "java_agent",
    srcs = ["InstrTestJavaAgent.java"],
    premain_class = "phasicj.agent.instr.test.java_agent.InstrTestJavaAgent",
    visibility = [":__subpackages__"],
    deps = ["//phasicj/agent/instr"],
)
