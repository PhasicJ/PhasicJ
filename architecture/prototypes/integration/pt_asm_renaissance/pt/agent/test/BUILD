load("@dwtj_rules_java//java:defs.bzl", "dwtj_java_test")

dwtj_java_test(
    name = "test",
    # TODO(dwtj): Remove the `srcs` attribute and this file when they are no
    #  longer needed. It is currently needed because the `srcs` attribute is
    #  mandatory and required to be non-empty.
    srcs = ["DoNothing.java"],
    args = [
        "--repetitions",
        "1",
        "scrabble",
    ],
    data = [
        "@org_ow2_asm//jar",
    ],
    java_agents = {
        "//pt/agent": "",
    },
    jvm_flags = [
        # TODO(dwtj): Consider ways to not need to hard-code these paths to the
        #  JARs. I tried modifying my Java rules to support $(location)
        #  expansion, but that gave me paths to these JARs at compile time, not
        #  at run time.
        # TODO(dwtj): Consider trying $(location) expansion again with
        #  $(rootpath) and $(execpath).
        "-Xbootclasspath/a:external/org_ow2_asm/jar/downloaded.jar:pt/agent/agent.jar",
    ],
    main_class = "org.renaissance.core.Launcher",
    deps = [
        "//pt/agent",
        "@phasicj//third_party/renaissance",
    ],
)
