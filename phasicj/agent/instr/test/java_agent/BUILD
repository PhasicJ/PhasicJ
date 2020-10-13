load("@dwtj_rules_java//java:defs.bzl", "dwtj_java_agent")

dwtj_java_agent(
    name = "java_agent",
    premain_class = "phasicj.agent.instr.test.java_agent.JavaAgent",
    srcs = ["JavaAgent.java"],
    deps = ["//phasicj/agent/instr"],
    visibility = [":__subpackages__"],
)
