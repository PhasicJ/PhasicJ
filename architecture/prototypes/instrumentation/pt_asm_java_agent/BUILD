load(
    "@dwtj_rules_java//java:defs.bzl",
    "dwtj_java_agent",
    "dwtj_java_test",
)

dwtj_java_agent(
    name = "agent",
    srcs = [
        "pt/AsmJavaAgent.java",
        "pt/MonitorEnterCounter.java",
    ],
    can_retransform_classes = True,
    premain_class = "pt.AsmJavaAgent",
    deps = ["//third_party/maven:asm"],
)

dwtj_java_test(
    name = "test",
    srcs = ["pt/MyTestApp.java"],
    java_agents = {
        ":agent": "",
    },
    main_class = "pt.MyTestApp",
    deps = ["//third_party/maven:asm"],  # NOTE(dwtj): No support for the `exports` attribute yet.
)
