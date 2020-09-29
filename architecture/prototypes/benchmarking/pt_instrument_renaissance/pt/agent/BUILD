load("@dwtj_rules_java//java:defs.bzl", "dwtj_java_agent")

dwtj_java_agent(
    name = "agent",
    srcs = [
        "MonitorEnterInstrInstrumenter.java",
        "Agent.java",
        "Printer.java",
    ],
    premain_class = "pt.agent.Agent",
    deps = ["@phasicj//third_party/asm"],
    visibility = ["//visibility:public"],
    output_jar = "agent.jar",
)
