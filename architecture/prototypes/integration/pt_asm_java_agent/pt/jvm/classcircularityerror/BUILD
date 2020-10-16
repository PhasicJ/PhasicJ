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
    can_retransform_classes = True,
    premain_class = "pt.jvm.classcircularityerror.MyAgent",
    deps = ["//third_party/maven:asm"],
)

dwtj_java_test(
    name = "test",
    srcs = ["//pt:MyTestApp.java"],
    java_agents = {
        ":my_agent": "",
    },
    main_class = "pt.MyTestApp",
)
