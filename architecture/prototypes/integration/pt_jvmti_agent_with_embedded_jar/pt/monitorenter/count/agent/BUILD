load("@dwtj_rules_java//java:defs.bzl", "dwtj_java_binary")
load("@rules_cc//cc:defs.bzl", "cc_binary")

cc_binary(
    name = "agent",
    srcs = ["agent.cc"],
    linkshared = True,
    deps = [
        "//pt/monitorenter/count/agent/rt:extract_embedded_jar",
        "@openjdk_linux_x64//cc/jni:headers",
        "@openjdk_linux_x64//cc/jvmti:headers",
    ],
)

dwtj_java_binary(
    name = "my_bin",
    srcs = ["MyBin.java"],
    main_class = "pt.monitorenter.count.agent.MyBin",
    output_jar = "my_bin.jar",
    deps = ["//pt/monitorenter/count/agent/rt:agent_runtime"],
)

# NOTE(dwtj): Above we build `:my_bin` with the `:agent_runtime` (so that it
#  will compile successfully), but below we don't run it with the
#  `:agent_runtime` JAR on the class path. Instead, we are testing to make sure
#  that the JVMTI agent actually extracts its embedded copy of this JAR and puts
#  it on the class path such that the `AgentRuntime` class is found at runtime.

sh_test(
    name = "agent_test",
    srcs = ["agent_test.sh"],
    args = [
        "$(rootpath @openjdk_linux_x64//jdk:bin/java)",
        "$(rootpath :agent)",
        "$(rootpath :my_bin.jar)",
    ],
    data = [
        ":agent",
        ":my_bin.jar",
        "@openjdk_linux_x64//jdk:bin/java",
    ],
)
