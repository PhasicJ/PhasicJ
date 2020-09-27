load(
    "@dwtj_rules_java//java:defs.bzl",
    "dwtj_java_agent",
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

# NOTE(dwtj): This `sh_test` target is a hack to invoke `java` with a custom-set
#  `-Xbootclasspath/a` argument.
sh_test(
    name = "instrument_renaissance_benchmark_test",
    srcs = ["run_renaissance_benchmark_with_instrumenting_agent.sh"],
    args = [
        "$(location @dwtj_local_openjdk//:java)",
        "$(location :agent.jar)",
        "$(location @org_ow2_asm//jar)",
        "$(location @com_github_renaissance_benchmarks//jar)",
    ],
    data = [
        "@dwtj_local_openjdk//:java",
        ":agent.jar",
        "@org_ow2_asm//jar",
        "@com_github_renaissance_benchmarks//jar",
    ]
)
