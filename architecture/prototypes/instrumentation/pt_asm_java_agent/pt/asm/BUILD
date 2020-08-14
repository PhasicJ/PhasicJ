load(
    "@dwtj_rules_java//java:defs.bzl",
    "dwtj_java_agent",
    "dwtj_java_test",
)

dwtj_java_agent(
    name = "my_agent",
    srcs = [
        "MonitorEnterInstrCounter.java",
        "MyAgent.java",
    ],
    premain_class = "pt.asm.MyAgent",
    deps = ["//third_party/maven:asm"],
)

dwtj_java_test(
    name = "test",
    srcs = ["//pt:MyTestApp.java"],
    java_agents = {
        ":my_agent": "",
    },
    main_class = "pt.MyTestApp",
    deps = ["//third_party/maven:asm"],  # TODO(dwtj): Remove this once `deps` behavior is fixed.
)
