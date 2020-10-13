load("@dwtj_rules_java//java:defs.bzl", "dwtj_java_agent")

dwtj_java_agent(
    name = "agent",
    srcs = [
        "Agent.java",
        "MonitorEnterInstrInstrumenter.java",
        "Printer.java",
    ],
    output_jar = "agent.jar",
    premain_class = "pt.agent.Agent",
    visibility = ["//visibility:public"],
    deps = ["@phasicj//third_party/asm"],
)
