load(
    "@dwtj_rules_java//java:defs.bzl",
    "dwtj_java_agent",
    "dwtj_java_test",
)

dwtj_java_agent(
    name = "agent",
    srcs = [
        "MonitorEnterInstrInstrumenter.java",
        "Agent.java",
        "Printer.java",
    ],
    premain_class = "pt.agent.Agent",
    deps = ["//third_party/asm"],
    visibility = ["//visibility:public"],
    output_jar = "agent.jar",
)

dwtj_java_test(
    name = "test",
    main_class = "org.renaissance.core.Launcher",
    # TODO(dwtj): Remove the `srcs` attribute and this file when they are no
    #  longer needed. It is currently needed because the `srcs` attribute is
    #  mandatory and required to be non-empty.
    srcs = ["DoNothing.java"],
    args = ["--repetitions", "1", "scrabble"],
    java_agents = {
        ":agent": "",
    },
    jvm_flags = [
        # TODO(dwtj): Consider ways to not need to hard-code these paths to the
        #  JARs. I tried modifying my Java rules to support $(location)
        #  expansion, but that gave me paths to these JARs at compile time, not
        #  at run time.
        "-Xbootclasspath/a:external/org_ow2_asm/jar/downloaded.jar:pt/agent/agent.jar",
    ],
    data = [
        "@org_ow2_asm//jar",
    ],
    deps = [
        ":agent",
        "//third_party/renaissance",
    ],
)
