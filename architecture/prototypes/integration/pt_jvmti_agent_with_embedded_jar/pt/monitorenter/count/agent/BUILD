load("@dwtj_rules_java//java:defs.bzl", "dwtj_java_binary")
load("@rules_cc//cc:defs.bzl", "cc_binary")

cc_binary(
    name = "agent",
    srcs = ["agent.cc"],
    linkshared = True,
    deps = [
        "//third_party/net/java/jdk:jni_headers",
        "//third_party/net/java/jdk:jvmti_headers",
        "//pt/monitorenter/count/agent/rt:extract_embedded_jar",
    ],
)

dwtj_java_binary(
    name = "my_bin",
    main_class = "pt.monitorenter.count.agent.MyBin",
    srcs = ["MyBin.java"],
    deps = ["//pt/monitorenter/count/agent/rt:agent_runtime"],
    output_jar = "my_bin.jar",
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
        "$(location @net_java_jdk_15_linux_x64//:bin/java)",
        "$(location :agent)",
        "$(location :my_bin.jar)",
    ],
    data = [
        "@net_java_jdk_15_linux_x64//:bin/java",
        ":agent",
        ":my_bin.jar",
    ],
)
