load("@dwtj_rules_java//java:defs.bzl", "dwtj_java_test")

dwtj_java_test(
    name = "renaissance_with_pt_agent",
    main_class = "org.renaissance.core.Launcher",
    # TODO(dwtj): Remove this attribute and file when they are no longer needed.
    srcs = ["DoNothing.java"],
    java_agents = {
        "//pt/agent": ""
    },
    args = ["--repetitions", "1", "scrabble"],
    deps = [
        "//third_party/renaissance",
        "//third_party/maven:asm",
        "//pt/agent",
    ],
)
