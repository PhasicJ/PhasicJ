load(
    "@dwtj_rules_java//java:defs.bzl",
    "dwtj_java_agent",
    "dwtj_java_test",
)

dwtj_java_agent(
    name = "agent",
    srcs = ["PtAsmJavaAgent.java"],
    deps = ["//third_party/maven:asm"],
    premain_class = "PtAsmJavaAgent",
)

dwtj_java_test(
    name = "test",
    srcs = ["MyTestApp.java"],
    java_agents = {
        ":agent": "",
    },
    main_class = "MyTestApp",
)
